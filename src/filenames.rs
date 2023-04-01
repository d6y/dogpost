use time::macros::format_description;
use time::{format_description, OffsetDateTime};

pub struct Filenames {
    media_path: String,
    github_media_path: String,
    github_post_path: String,
    date: OffsetDateTime,
    slug: String,
}

impl Filenames {
    pub fn attachment_markdown_url(&self, count: usize, ext: &str) -> String {
        // TODO: propagate parsing up to command line parsing
        let path_format = format_description::parse(&self.media_path).unwrap();

        // TODO: move YMD to lazy! const
        format!(
            "{}/{}-{}-{}.{}",
            self.date.format(&path_format).unwrap(),
            self.date
                .format(format_description!("[year]-[month]-[day]"))
                .unwrap(),
            self.slug,
            count,
            ext
        )
    }

    pub fn attachment_github_path(&self, count: usize, ext: &str) -> String {
        let path_format = format_description::parse(&self.github_media_path).unwrap();

        format!(
            "{}/{}-{}-{}.{}",
            self.date.format(&path_format).unwrap(),
            self.date
                .format(format_description!("[year]-[month]-[day]"))
                .unwrap(),
            self.slug,
            count,
            ext
        )
    }

    pub fn post_github_path(&self) -> String {
        format!(
            "{}/{}-{}.md",
            self.github_post_path,
            self.date
                .format(format_description!("[year]-[month]-[day]"))
                .unwrap(),
            self.slug
        )
    }

    pub fn new(
        media_path: &str,
        github_media_path: &str,
        github_post_path: &str,
        date: &OffsetDateTime,
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
