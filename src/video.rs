use std::{
    path::Path,
    process::{Command, Stdio},
};

use crate::mishaps::Mishap;

pub fn web_video(input_path: &Path, output_path: &Path) -> Result<(), Mishap> {
    Command::new("ffmpeg")
        .arg("-i")
        .arg(input_path)
        .arg("-c:v")
        .arg("libx264")
        .arg("-crf")
        .arg("24")
        .arg("-preset")
        .arg("medium")
        .arg("-vf")
        .arg("format=yuv420p") // NB 3x the memory usage of the scale command
        // .arg("scale='min(480,iw)':trunc(ow/a/2)*2")
        .arg("-c:a")
        .arg("copy")
        .arg("-b:a")
        .arg("64k")
        .arg("-movflags")
        .arg("+faststart")
        .arg(output_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

pub fn ffmpeg_installed() -> bool {
    let status = Command::new("ffmpeg").arg("-version").output();
    status.is_ok()
}
