use crate::{
    blog::{Attachment, PostInfo},
    image,
    mishaps::Mishap,
    video,
};

pub fn transcode(info: PostInfo) -> Result<PostInfo, Mishap> {
    info.map_attachments(transcode_video_for_web)?
        .map_attachments(transcode_heic)?
        .map_attachments(normalize_filenames)
}

fn normalize_filenames(a: Attachment) -> Result<Attachment, Mishap> {
    let norm = |str: String| str.to_lowercase().replace(".jpeg", ".jpg");

    Ok(Attachment {
        file_path: a.file_path,
        url_path: norm(a.url_path),
        github_path: norm(a.github_path),
        mime_type: a.mime_type,
    })
}

fn transcode_heic(a: Attachment) -> Result<Attachment, Mishap> {
    let target_mime_type = "image/jpeg";
    let target_ext = "jpg";

    // Leave it alone if it's already the target image, or not JPEG-like
    if !a.is_image()
        || a.mime_type == target_mime_type
        || a.mime_type == "image/gif"
        || a.mime_type == "image/png"
    {
        Ok(a)
    } else {
        let input_path = &a.file_path;
        let output_path = &a.file_path.with_extension(target_ext);
        image::to_jpeg(input_path, output_path)?;
        Ok(Attachment {
            file_path: output_path.to_owned(),
            url_path: a.url_path.with_extension(target_ext),
            github_path: a.github_path.with_extension(target_ext),
            mime_type: target_mime_type.to_string(),
        })
    }
}

fn transcode_video_for_web(a: Attachment) -> Result<Attachment, Mishap> {
    let target_mime_type = "video/mp4";
    let target_ext = "mp4";
    if !a.is_video() || a.mime_type == target_mime_type {
        Ok(a)
    } else {
        let input_path = &a.file_path;
        let output_path = &a.file_path.with_extension(target_ext);
        video::web_video(input_path, output_path)?;

        let output_size = std::fs::metadata(output_path)?.len();
        let input_size = std::fs::metadata(input_path)?.len();
        println!("Transcoded size from {} to {}", input_size, output_size);

        Ok(Attachment {
            file_path: output_path.to_owned(),
            url_path: a.url_path.with_extension(target_ext),
            github_path: a.github_path.with_extension(target_ext),
            mime_type: target_mime_type.to_string(),
        })
    }
}

pub trait RenameExt
where
    Self: Sized,
{
    fn with_extension(self, new_ext: &str) -> Self;
    fn get_extension(&self) -> Option<&str>;
}

impl RenameExt for String {
    fn with_extension(mut self, new_ext: &str) -> Self {
        if let Some(pos) = self.rfind('.') {
            self.replace_range(pos + 1.., new_ext)
        }
        self
    }

    fn get_extension(&self) -> Option<&str> {
        self.rfind('.').map(|i| &self[i + 1..])
    }
}
