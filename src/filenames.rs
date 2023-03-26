use chrono::{DateTime, Utc};
use std::path::Path;
use std::path::PathBuf;

pub struct Filenames {
    media_dir: PathBuf,
    media_path: String,
    date: DateTime<Utc>,
    slug: String,
}

impl Filenames {
    pub fn attachment_url(&self, count: usize, ext: Option<&str>) -> String {
        format!(
            "//{}/{}-{}-fullsize-{}.{}",
            self.bucket,
            self.date.format("%Y-%m-%d"),
            self.slug,
            count,
            ext.unwrap_or_default()
        )
    }

    pub fn post_filename(&self) -> String {
        format!("{}-{}.md", self.date.format("%Y-%m-%d"), self.slug)
    }

    pub fn new(media_dir: &Path, bucket: &str, date: &DateTime<Utc>, slug: &str) -> Filenames {
        Filenames {
            media_dir: media_dir.to_path_buf(),
            bucket: bucket.to_string(),
            date: *date,
            slug: slug.to_string(),
        }
    }
}
