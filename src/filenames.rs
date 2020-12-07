use chrono::{DateTime, Utc};
use std::path::PathBuf;

pub struct Filenames {
    media_dir: PathBuf,
    posts_dir: PathBuf,
    bucket: String,
    date: DateTime<Utc>,
    slug: String,
}

impl Filenames {
    pub fn attachment_fullsize_url(&self, count: usize, ext: Option<&str>) -> String {
        format!(
            "http://{}/{}-{}-fullsize-{}.{}",
            self.bucket,
            self.date.format("%Y-%m-%d"),
            self.slug,
            count,
            ext.unwrap_or_default()
        )
    }

    pub fn attachment_thumb_url(&self, count: usize, ext: Option<&str>) -> String {
        format!(
            "http://{}/{}-{}-thumb-{}.{}",
            self.bucket,
            self.date.format("%Y-%m-%d"),
            self.slug,
            count,
            ext.unwrap_or_default()
        )
    }

    pub fn attachment_fullsize_filename(&self, count: usize, ext: Option<&str>) -> PathBuf {
        let filename = format!(
            "{}-{}-fullsize-{}.{}",
            self.date.format("%Y-%m-%d"),
            self.slug,
            count,
            ext.unwrap_or_default()
        );
        let mut path = self.media_dir.clone();
        path.push(filename);
        path
    }

    pub fn attachment_thumb_filename(&self, count: usize, ext: Option<&str>) -> PathBuf {
        let filename = format!(
            "{}-{}-thumb-{}.{}",
            self.date.format("%Y-%m-%d"),
            self.slug,
            count,
            ext.unwrap_or_default()
        );
        let mut path = self.media_dir.clone();
        path.push(filename);
        path
    }

    pub fn post_filename(&self) -> PathBuf {
        let filename = format!("{}-{}.md", self.date.format("%Y-%m-%d"), self.slug);
        let mut path = self.posts_dir.clone();
        path.push(filename);
        path
    }

    pub fn new(
        media_dir: &PathBuf,
        posts_dir: &PathBuf,
        bucket: &str,
        date: &DateTime<Utc>,
        slug: &str,
    ) -> Filenames {
        Filenames {
            media_dir: media_dir.clone(),
            posts_dir: posts_dir.clone(),
            bucket: bucket.to_string(),
            date: date.clone(),
            slug: slug.to_string(),
        }
    }
}
