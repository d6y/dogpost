use crate::tag::Tag;

use super::mishaps::Mishap;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::io::Write;
use std::path::PathBuf;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

#[derive(Debug)]
pub struct PostInfo {
    pub title: String,
    pub author: String,
    pub content: Option<String>,
    pub date: OffsetDateTime,
    pub attachments: Vec<Attachment>,
    pub file_path: String,
    pub tags: Vec<Tag>,
}

impl PostInfo {
    pub fn new(
        title: String,
        author: String,
        content: Option<String>,
        date: OffsetDateTime,
        tags: Vec<Tag>,
        attachments: Vec<Attachment>,
        file_path: String,
    ) -> PostInfo {
        PostInfo {
            title: title.trim().to_owned(),
            author: author.trim().to_owned(),
            content: content.map(|str| str.trim().to_owned()),
            tags,
            date,
            attachments,
            file_path,
        }
    }

    pub fn map_attachments<F>(self, f: F) -> Result<PostInfo, Mishap>
    where
        F: Fn(Attachment) -> Result<Attachment, Mishap>,
    {
        let mut mapped_attachments = Vec::new();
        for attachment in self.attachments.into_iter() {
            let mapped_attachment = f(attachment)?;
            mapped_attachments.push(mapped_attachment);
        }

        Ok(PostInfo {
            title: self.title,
            author: self.author,
            content: self.content,
            date: self.date,
            attachments: mapped_attachments,
            file_path: self.file_path,
            tags: self.tags,
        })
    }
}

#[derive(Debug)]
pub struct Attachment {
    pub file_path: PathBuf,
    pub url_path: String,
    pub github_path: String,
    pub mime_type: String,
}

impl Attachment {
    pub fn is_image(&self) -> bool {
        self.mime_type.starts_with("image/")
    }

    pub fn is_video(&self) -> bool {
        self.mime_type.starts_with("video/")
    }

    fn markdown(&self) -> Option<String> {
        if self.is_image() {
            Some(format!(r#"![]({})"#, &self.url_path))
        } else if self.is_video() {
            let media_path = &self.url_path;
            Some(format!(r#"{{{{< video src="{}" >}}}}"#, media_path))
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
struct FrontMatter {
    title: String,
    author: String,
    date: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    image: Option<String>,

    #[serde(rename = "type")]
    post_type: String,

    tags: Vec<Tag>,
}

pub fn write(post: &PostInfo) -> Result<String, Mishap> {
    let mut markdown = Vec::new();
    write!(markdown, "{}", post_meta(post))?;
    write!(markdown, "\n\n")?;

    match &post.content {
        Some(text) => write!(markdown, "{}\n\n", text)?,
        None => {}
    };

    for media in post.attachments.iter() {
        if let Some(media_md) = media.markdown() {
            write!(markdown, "{media_md}")?;
            write!(markdown, "\n\n")?;
        }
    }

    Ok(String::from_utf8(markdown)?)
}

fn post_meta(post: &PostInfo) -> String {
    let featured_image = post.attachments.first().map(|img| &img.url_path).cloned();

    let fm = FrontMatter {
        title: post.title.to_string(),
        author: post.author.to_string(),
        date: post.date.format(&Iso8601::DEFAULT).unwrap(),
        image: featured_image,
        post_type: "post".to_string(),
        tags: post.tags.clone(),
    };

    let yaml = serde_yaml::to_string(&fm).unwrap();

    format!("---\n{}\n---", yaml)
}
