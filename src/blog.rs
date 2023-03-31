use crate::tag::Tag;

use super::mishaps::Mishap;
use chrono::{DateTime, Utc};
use mime_guess;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PostInfo {
    pub title: String,
    pub author: String,
    pub content: Option<String>,
    pub date: DateTime<Utc>,
    pub attachments: Vec<Attachment>,
    pub file_path: String,
    pub tags: Vec<Tag>,
}

#[derive(Debug)]
pub struct Attachment {
    pub file_path: PathBuf,
    pub url_path: String,
    pub github_path: String,
}

impl Attachment {
    fn markdown(&self) -> Option<String> {
        let mime_type = mime_guess::from_path(&self.file_path).first_or_octet_stream();
        let media_type = mime_type.type_();

        if media_type == mime_guess::mime::IMAGE {
            Some(format!(r#"![]({})"#, &self.url_path))
        } else if media_type == mime_guess::mime::VIDEO {
            let media_path = &self.url_path;
            Some(format!("<video height='720' controls=''><source src='{media_path}' type='{media_type}'></video>"))
        } else {
            None
        }
    }
}

impl PostInfo {
    pub fn new(
        title: String,
        author: String,
        content: Option<String>,
        date: DateTime<Utc>,
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

    for media in post.attachments.iter() {
        if let Some(media_md) = media.markdown() {
            write!(markdown, "{media_md}")?;
            write!(markdown, "\n\n")?;
        }
    }

    match &post.content {
        Some(text) => write!(markdown, "{}\n\n", text)?,
        None => {}
    };

    Ok(String::from_utf8(markdown)?)
}

fn post_meta(post: &PostInfo) -> String {
    let featured_image = post.attachments.first().map(|img| &img.url_path).cloned();

    let fm = FrontMatter {
        title: post.title.to_string(),
        author: post.author.to_string(),
        date: post.date.format("%Y-%m-%d %H:%M:%S").to_string(),
        image: featured_image,
        post_type: "post".to_string(),
        tags: post.tags.clone(),
    };

    let yaml = serde_yaml::to_string(&fm).unwrap();

    format!("---\n{}\n---", yaml)
}
