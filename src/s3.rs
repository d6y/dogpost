use std::path::PathBuf;

use super::blog::PostInfo;
use super::mishaps::Mishap;
use super::settings::Settings;

use futures::future::*;

use rusoto_core::Region;
use rusoto_s3::{PutObjectRequest, S3Client, StreamingBody, S3};

pub async fn upload(settings: &Settings, post: &PostInfo) -> Result<(), Mishap> {
    let s3_client = S3Client::new(Region::UsEast1);

    let mut puts = Vec::new();
    for img in post.attachments.iter() {
        let full = put_request(&settings.s3_bucket, &img.file, &img.mimetype);
        puts.push(s3_client.put_object(full));

        let thumb = put_request(&settings.s3_bucket, &img.thumbnail.file, &img.mimetype);
        puts.push(s3_client.put_object(thumb));
    }

    let _results = try_join_all(puts).await?;
    // dbg!(_results);

    Ok(())
}

fn put_request(bucket: &str, path: &PathBuf, content_type: &str) -> PutObjectRequest {
    // TODO: https://stackoverflow.com/questions/57810173/streamed-upload-to-s3-with-rusoto
    let bytes = std::fs::read(path).ok();
    PutObjectRequest {
        bucket: bucket.to_string(),
        key: path
            .file_name()
            .map(|v| v.to_string_lossy().to_string())
            .unwrap(),
        body: bytes.map(|b| StreamingBody::from(b)),
        content_type: Some(content_type.to_string()),
        acl: Some(String::from("public-read")),
        ..Default::default()
    }
}
