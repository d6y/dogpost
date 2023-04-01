use chrono::{DateTime, LocalResult, TimeZone, Utc};
use log::debug;
use mailparse::*;

use std::fs::File;
use std::io::Write;
use std::path::Path;

use super::blog::{Attachment, PostInfo};
use super::filenames::Filenames;
use super::settings::Settings;
use super::signatureblock;

use super::mishaps::Mishap;

pub fn fetch(settings: &Settings) -> Result<Option<String>, Mishap> {
    debug!("Fetching");
    let client = imap::ClientBuilder::new(&settings.imap_hostname, settings.imap_port).rustls()?;

    let mut imap_session = client
        .login(&settings.imap_user, &settings.imap_password)
        .map_err(|(err, _client)| err)?;

    debug!("Selecting mailbox: {}", &settings.mailbox);
    imap_session.select(&settings.mailbox)?;

    // fetch message number 1 in this mailbox
    let messages = imap_session.fetch("1", "RFC822")?;
    debug!("Messages: {:?}", messages.len());

    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        imap_session.logout()?;
        return Ok(None);
    };

    // The body will be the mime content of the message (including heeader)
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    if settings.expunge {
        imap_session.store("1", "+FLAGS (\\Seen \\Deleted)")?;
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

pub fn extract(
    settings: &Settings,
    working_dir: &Path,
    mail: ParsedMail,
) -> Result<PostInfo, Mishap> {
    validate_sender(settings, &mail).and_then(|_| read_post(settings, working_dir, mail))
}

fn validate_sender(settings: &Settings, mail: &ParsedMail) -> Result<(), Mishap> {
    let from_address: Option<String> = from(mail)?;

    if settings.allowed_domains.is_empty() {
        Ok(())
    } else {
        match from_address {
            None => Err(Mishap::MissingSender),
            Some(email) => {
                let matching_domain = settings
                    .allowed_domains
                    .iter()
                    .find(|&d| email.ends_with(d));
                if matching_domain.is_none() {
                    Err(Mishap::Unauthorised(email))
                } else {
                    Ok(())
                }
            }
        }
    }
}

fn read_post(
    settings: &Settings,
    working_dir: &Path,
    mail: ParsedMail,
) -> Result<PostInfo, Mishap> {
    let sender: String = sender_name(&mail)?.unwrap_or_else(|| String::from("Someone"));
    let subject: Option<String> = mail.headers.get_first_value("Subject");
    let content: Option<String> = body(&mail)?.map(signatureblock::remove);
    let date: DateTime<Utc> = date(&mail)?.unwrap_or_else(Utc::now);

    // The blog post title will be the subject line, and if that's missing use the body text
    let raw_title = subject
        .filter(|str| !str.is_empty())
        .or_else(|| content.clone())
        .unwrap_or_else(|| String::from("Untitled"));

    let (title, tags) = crate::tag::detag(&raw_title);

    let slug = slug::slugify(&title);

    let conventions = Filenames::new(
        &settings.web_media_path,
        &settings.github_media_path,
        &settings.github_post_path,
        &date,
        &slug,
    );

    let attachments = attachments(&conventions, working_dir, &mail)?;

    Ok(PostInfo::new(
        title,
        sender,
        content,
        date,
        tags,
        attachments,
        conventions.post_github_path(),
    ))
}

fn date(mail: &ParsedMail) -> Result<Option<DateTime<Utc>>, Mishap> {
    match mail.headers.get_first_value("Date") {
        None => Ok(None),
        Some(str) => dateparse(&str)
            .map_err(|e| Mishap::EmailField(e.to_string()))
            .map(|seconds| LocalResult::single(Utc.timestamp_millis_opt(1000_i64 * seconds))),
    }
}

fn walk_from_header<F>(mail: &ParsedMail, pick: F) -> Result<Option<String>, MailParseError>
where
    F: Fn(SingleInfo) -> Option<String>,
{
    let sender_text: Option<String> = mail.headers.get_first_value("From");
    match sender_text {
        None => Ok(None),
        Some(str) => match addrparse(&str) {
            Err(err) => Err(err),
            Ok(addrs) if addrs.is_empty() => Ok(None),
            Ok(addrs) => Ok(addrs.extract_single_info().and_then(pick)),
        },
    }
}

fn sender_name(mail: &ParsedMail) -> Result<Option<String>, MailParseError> {
    walk_from_header(mail, |info| info.display_name)
}

fn from(mail: &ParsedMail) -> Result<Option<String>, MailParseError> {
    walk_from_header(mail, |info| Some(info.addr))
}

fn body(mail: &ParsedMail) -> Result<Option<String>, MailParseError> {
    if mail.ctype.mimetype == "text/plain" {
        mail.get_body().map(Some)
    } else if mail.subparts.is_empty() {
        Ok(None)
    } else {
        let parts: Result<Vec<Option<String>>, MailParseError> =
            mail.subparts.iter().map(body).collect();

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

trait MediatTypeDetection {
    fn is_image(&self) -> bool;
    fn is_video(&self) -> bool;
    fn mime(&self) -> String;
    fn guess_ext(&self) -> String;
}

impl<'a> MediatTypeDetection for ParsedMail<'a> {
    fn is_image(&self) -> bool {
        self.ctype.mimetype.starts_with("image")
    }

    fn is_video(&self) -> bool {
        self.ctype.mimetype.starts_with("video")
    }

    fn mime(&self) -> String {
        self.ctype.mimetype.clone()
    }

    fn guess_ext(&self) -> String {
        let exts = mime_guess::get_mime_extensions_str(&self.mime());
        let ext = exts.and_then(|xs| xs.first()).unwrap_or(&"");
        ext.to_string()
    }
}

fn find_attachments<'a>(mail: &'a ParsedMail<'a>) -> Vec<&'a ParsedMail<'a>> {
    let head: Vec<&ParsedMail> = to_vec(Some(mail).filter(|m| m.is_image() || m.is_video()));

    let tail = mail.subparts.iter().flat_map(find_attachments);

    head.into_iter().chain(tail).collect()
}

fn attachments(
    conventions: &Filenames,
    working_dir: &Path,
    mail: &ParsedMail,
) -> Result<Vec<Attachment>, Mishap> {
    let mut images = Vec::new();

    for (count, part) in find_attachments(mail).iter().enumerate() {
        let ext = part.guess_ext();

        let filename = working_dir.to_owned().join(format!("{}.{}", count, ext));
        let bytes = part.get_body_raw()?;
        let _file = save_raw_body(&filename, bytes)?;

        let img = Attachment {
            file_path: filename,
            url_path: conventions.attachment_markdown_url(count, &ext),
            github_path: conventions.attachment_github_path(count, &ext),
            mime_type: part.mime(),
        };

        images.push(img);
    }

    Ok(images)
}

fn save_raw_body(filename: &Path, bytes: Vec<u8>) -> Result<File, Mishap> {
    let mut file = File::create(filename)?;
    file.write_all(bytes.as_slice())?;
    Ok(file)
}
