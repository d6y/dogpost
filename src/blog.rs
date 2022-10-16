use super::mishaps::Mishap;
use chrono::{DateTime, Utc};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug)]
pub struct PostInfo {
    pub title: String,
    pub author: String,
    pub content: Option<String>,
    pub date: DateTime<Utc>,
    pub attachments: Vec<Image>,
    pub file_path: String,
}

#[derive(Debug)]
pub struct Image {
    pub file: PathBuf,
    pub url: String,
    pub mimetype: String,
    pub thumbnail: Thumbnail,
}

#[derive(Debug)]
pub struct Thumbnail {
    pub file: PathBuf,
    pub url: String,
    pub width: u16,
    pub height: u16,
}

impl PostInfo {
    pub fn new(
        title: String,
        author: String,
        content: Option<String>,
        date: DateTime<Utc>,
        attachments: Vec<Image>,
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

pub fn write(post: &PostInfo) -> Result<String, Mishap> {
    let mut markdown = Vec::new();
    write!(markdown, "{}", post_meta(post))?;
    write!(markdown, "\n\n")?;

    for image in post.attachments.iter() {
        write!(
            markdown,
            r#"<a href="{}"><img src="{}" width="{}" height="{}"></a>"#,
            image.url, image.thumbnail.url, image.thumbnail.width, image.thumbnail.height
        )?;
        write!(markdown, "\n\n")?;
    }

    match &post.content {
        Some(text) => write!(markdown, "{}\n\n", text)?,
        None => {}
    };

    Ok(String::from_utf8(markdown)?)
}

fn post_meta(post: &PostInfo) -> String {
    let featured_image = post.attachments.first().map(|img| &img.url);

    format!(
        r#"---
title: |
    {}
author: {}
date: {}
layout: post
comments: true
{}
---"#,
        post.title,
        post.author,
        post.date.format("%Y-%m-%d %H:%M"),
        featured_image
            .map(|url| format!("image: {}", url))
            .unwrap_or_else(|| "".to_string())
    )
}
