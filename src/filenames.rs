use chrono::{DateTime, Utc};

pub struct Filenames {
    media_path: String,
    github_media_path: String,
    github_post_path: String,
    date: DateTime<Utc>,
    slug: String,
}

impl Filenames {
    pub fn attachment_markdown_url(&self, count: usize, ext: &str) -> String {
        format!(
            "{}/{}-{}-{}.{}",
            self.date.format(&self.media_path),
            self.date.format("%Y-%m-%d"),
            self.slug,
            count,
            ext
        )
    }

    pub fn attachment_github_path(&self, count: usize, ext: &str) -> String {
        format!(
            "{}/{}-{}-{}.{}",
            self.date.format(&self.github_media_path),
            self.date.format("%Y-%m-%d"),
            self.slug,
            count,
            ext
        )
    }

    pub fn post_github_path(&self) -> String {
        format!(
            "{}/{}-{}.md",
            self.github_post_path,
            self.date.format("%Y-%m-%d"),
            self.slug
        )
    }

    pub fn new(
        media_path: &str,
        github_media_path: &str,
        github_post_path: &str,
        date: &DateTime<Utc>,
        slug: &str,
    ) -> Filenames {
        Filenames {
            media_path: media_path.to_owned(),
            github_media_path: github_media_path.to_owned(),
            github_post_path: github_post_path.to_owned(),
            date: *date,
            slug: slug.to_string(),
        }
    }
}
