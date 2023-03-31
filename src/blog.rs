use super::mishaps::Mishap;
use chrono::{DateTime, Utc};
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
}

#[derive(Debug)]
pub struct Attachment {
    pub file_path: PathBuf,
    pub url_path: String,
    pub github_path: String,
}

impl PostInfo {
    pub fn new(
        title: String,
        author: String,
        content: Option<String>,
        date: DateTime<Utc>,
        attachments: Vec<Attachment>,
        file_path: String,
    ) -> PostInfo {
        PostInfo {
            title: title.trim().to_owned(),
            author: author.trim().to_owned(),
            content: content.map(|str| str.trim().to_owned()),
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
    image: Option<String>,
    #[serde(rename = "type")]
    post_type: String,
}

pub fn write(post: &PostInfo) -> Result<String, Mishap> {
    let mut markdown = Vec::new();
    write!(markdown, "{}", post_meta(post))?;
    write!(markdown, "\n\n")?;

    for image in post.attachments.iter() {
        // TODO: support video
        write!(markdown, r#"![]({})"#, image.url_path)?;
        write!(markdown, "\n\n")?;
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
    };

    let yaml = serde_yaml::to_string(&fm).unwrap();

    format!("---\n{}\n---", yaml)
}
