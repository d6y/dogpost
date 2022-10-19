use github::Github;
use mishaps::Mishap;

use clap::Parser;
mod settings;
use settings::Settings;

mod blog;
mod email;
mod filenames;
mod github;
mod image;
mod mishaps;
mod s3;
mod signatureblock;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();

    if !settings.media_dir.exists() {
        std::fs::create_dir_all(&settings.media_dir).expect("creating media dir")
    };

    ensure_imagemagik_installed();

    let gh = Github::new(
        &settings.github_token,
        &settings.github_repo,
        &settings.github_branch,
    );

    let extract = |msg| email::extract(&settings, msg);

    match email::fetch(&settings) {
        Err(err) => stop("mailbox access", err), // Failed accessing mail box
        Ok(None) => complete(0),                 // No messages to process
        Ok(Some(mime_message)) => {
            match email::parse(&mime_message).and_then(extract) {
                Err(err) => stop("msg parse", err), // Message processing failed
                Ok(info) => match blog::write(&info) {
                    Err(err) => stop("Blog write", err),
                    Ok(content) => {
                        let path_name = format!("{}/{}", &settings.github_path, info.file_path);
                        let commit_msg = format!("add post: {}", info.title);
                        gh.commit(&path_name, &content, &commit_msg).await?;
                        s3::upload(&settings, &info).await?
                    }
                },
            }
        }
    }

    Ok(())
}

fn stop(context: &str, err: Mishap) -> ! {
    eprintln!("{context}: Failed: {err}", context = context, err = err);
    std::process::exit(1)
}

fn complete(num_msgs: usize) -> ! {
    println!("{}", num_msgs);
    std::process::exit(0)
}

fn ensure_imagemagik_installed() {
    if !image::imagemagic_installed() {
        panic!("Did not find ImageMagik");
    }
}
