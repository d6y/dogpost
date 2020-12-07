use chrono::{DateTime, TimeZone, Utc};
use imap;
use mailparse::*;
use native_tls;
use mime_db;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::blog::{Image, PostInfo, Thumbnail};
use super::settings::Settings;
use super::signatureblock;
use super::filenames::Filenames;

use super::mishaps::Mishap;

use super::image::thumbnail;

pub fn fetch(settings: &Settings) -> Result<Option<String>, Mishap> {

    // let tls = native_tls::TlsConnector::builder().build()?;
    // let client = imap::connect(
    //     (&settings.imap_hostname[..], settings.imap_port),
    //     &settings.imap_hostname,
    //     &tls,
    // )?;

    // If using local test Greemail server with no TLS:
    let client = imap::connect_insecure(
        (&settings.imap_hostname[..], settings.imap_port),
    )?;

    let mut imap_session = client
        .login(&settings.imap_user, &settings.imap_password)
        .map_err(|(err, _client)| err)?;

    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails

    let sequence_set = "1";
    let messages = imap_session.fetch(sequence_set, "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // The body will be the mime content of the message (including heeader)
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    if settings.expunge {
        imap_session.store(sequence_set.to_string(), "+FLAGS (\\Seen \\Deleted)")?;
        let _msg_sequence_numbers = imap_session.expunge()?;
    }

    imap_session.logout()?;

    Ok(Some(body))
}

pub fn parse(mime_msg: &str) -> Result<ParsedMail, Mishap> {
    let bytes = mime_msg.as_bytes();
    let result = mailparse::parse_mail(bytes)?;
    Ok(result)
}

pub fn extract(settings: &Settings, mail: ParsedMail) -> Result<PostInfo, Mishap> {

    let sender: String = sender(&mail)?.unwrap_or_else(|| String::from("Someone"));
    let subject: Option<String> = mail.headers.get_first_value("Subject");
    let content: Option<String> = body(&mail)?.map(signatureblock::remove);
    let date: DateTime<Utc> = date(&mail)?.unwrap_or_else(Utc::now);

    // The blog post title will be the subject line, and if that's missing use the body text
    let title = subject
        .filter(|str| !str.is_empty())
        .or_else(|| content.clone())
        .unwrap_or_else(|| String::from("Untitled"));

    let slug = slug::slugify(&title);

    let conventions = Filenames::new(
        &settings.media_dir,
        &settings.posts_dir,
        &settings.s3_bucket,
        &date,
        &slug,
    );

    let attachments = attachments(&conventions, settings.width, &mail)?;

    Ok(PostInfo::new(
        title,
        sender,
        content,
        date,
        attachments,
        conventions.post_filename(),
    ))
}

fn date(mail: &ParsedMail) -> Result<Option<DateTime<Utc>>, Mishap> {
    match mail.headers.get_first_value("Date") {
        None => Ok(None),
        Some(str) => dateparse(&str)
            .map_err(|e| Mishap::EmailField(e.to_string()))
            .map(|seconds| Utc.timestamp_millis(1000 as i64 * seconds))
            .map(|utc| Some(utc)),
    }
}

fn sender(mail: &ParsedMail) -> Result<Option<String>, MailParseError> {
    let sender_text: Option<String> = mail.headers.get_first_value("From");
    match sender_text {
        None => Ok(None),
        Some(str) => match addrparse(&str) {
            Err(err) => Err(err),
            Ok(addrs) if addrs.is_empty() => Ok(None),
            Ok(addrs) => Ok(addrs
                .extract_single_info()
                .and_then(|info| info.display_name)),
        },
    }
}

fn body(mail: &ParsedMail) -> Result<Option<String>, MailParseError> {
    if mail.ctype.mimetype == "text/plain" {
        mail.get_body().map(Some)
    } else if mail.subparts.is_empty() {
        Ok(None)
    } else {
        let parts: Result<Vec<Option<String>>, MailParseError> =
            mail.subparts.iter().map(|m| body(&m)).collect();

        let valid_parts: Result<Vec<String>, MailParseError> =
            parts.map(|os| os.into_iter().flatten().collect());

        match valid_parts {
            Ok(vec) if vec.is_empty() => Ok(None),
            Err(err) => Err(err),
            Ok(vec) => Ok(Some(vec[0].clone())),
        }
    }
}

fn to_vec<T>(o: Option<T>) -> Vec<T> {
    match o {
        Some(v) => vec![v],
        None => Vec::new(),
    }
}

fn find_attachemnts<'a>(mail: &'a ParsedMail<'a>) -> Vec<&'a ParsedMail<'a>> {
    let head: Vec<&ParsedMail> =
        to_vec(Some(mail).filter(|m| m.ctype.mimetype.starts_with("image")));

    let tail = mail.subparts.iter().map(|m| find_attachemnts(m)).flatten();

    head.into_iter().chain(tail).collect()
}

fn attachments(
    conventions: &Filenames,
    width: u16,
    mail: &ParsedMail,
) -> Result<Vec<Image>, Mishap> {
    let mut images = Vec::new();

    for (count, part) in find_attachemnts(&mail).iter().enumerate() {

        let ext = mime_db::extension(&part.ctype.mimetype);

        let filename = conventions.attachment_fullsize_filename(count, ext);
        let bytes = part.get_body_raw()?;
        let _file = save_raw_body(&filename, bytes)?;

        let thumb_filename = conventions.attachment_thumb_filename(count, ext);
        let (width, height) = thumbnail(&filename, &thumb_filename, width)?;

        let thumbnail = Thumbnail {
            file: thumb_filename,
            url: conventions.attachment_thumb_url(count, ext),
            width,
            height,
        };

        images.push(Image {
            file: filename,
            url: conventions.attachment_fullsize_url(count, ext),
            thumbnail,
            mimetype: mail.ctype.mimetype.clone(),
        });
    }

    Ok(images)
}

fn save_raw_body(filename: &Path, bytes: Vec<u8>) -> Result<File, Mishap> {
    let mut file = File::create(filename)?;
    file.write_all(bytes.as_slice())?;
    Ok(file)
}
