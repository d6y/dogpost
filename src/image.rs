use std::io::{Error, ErrorKind};
use std::path::Path;
use std::process::{Command, Stdio};

use crate::mishaps::Mishap;

fn _thumbnail(source: &Path, target: &Path, width: u16) -> Result<(u16, u16), Error> {
    let _convert_status = Command::new("convert")
        .arg(source)
        .arg("-resize")
        .arg(width.to_string())
        .arg("-auto-orient")
        .arg(target)
        .status()
        .expect("failed to execute convert process");

    let identify_output = Command::new("identify")
        .arg("-format")
        .arg("%wx%h")
        .arg(target)
        .output()
        .expect("failed to identify thumbnail");

    let output_text = String::from_utf8_lossy(&identify_output.stdout);

    let width_height: Vec<u16> = output_text.split('x').flat_map(|str| str.parse()).collect();

    if width_height.len() != 2 {
        let msg = format!("Expected wxh, not: {:?}", output_text);
        let cause = Error::new(ErrorKind::InvalidData, msg);
        Err(cause)
    } else {
        Ok((width_height[0], width_height[1]))
    }
}

pub fn to_jpeg(input_path: &Path, output_path: &Path) -> Result<(), Mishap> {
    Command::new("convert")
        .arg(input_path)
        .arg("-quality")
        .arg("90")
        .arg("-define")
        .arg("jpeg:preserve-settings")
        .arg("-define")
        .arg("jpeg:optimize-coding")
        .arg(output_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(())
}

pub fn imagemagick_installed() -> bool {
    let convert_status = Command::new("convert").arg("-version").output();
    convert_status.is_ok()
}
