use crate::{
    blog::{Attachment, PostInfo},
    mishaps::Mishap,
    video,
};

pub fn transcode(info: PostInfo) -> Result<PostInfo, Mishap> {
    let has_videos = info.attachments.iter().any(|a| a.is_video());

    if !has_videos {
        Ok(info)
    } else {
        info.map_attachments(transcode_video_for_web)
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
        Ok(Attachment {
            file_path: output_path.to_owned(),
            url_path: a.url_path.with_extension(target_ext),
            github_path: a.github_path.with_extension(target_ext),
            mime_type: target_mime_type.to_string(),
        })
    }
}

trait RenameExt {
    fn with_extension(self, new_ext: &str) -> Self;
}

impl RenameExt for String {
    fn with_extension(mut self, new_ext: &str) -> Self {
        if let Some(pos) = self.rfind('.') {
            self.replace_range(pos + 1.., new_ext)
        }
        self
    }
}
