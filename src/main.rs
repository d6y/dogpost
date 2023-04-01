use clap::Parser;
use github::{Github, NewContent};
use log::info;
use mishaps::Mishap;
use tempfile::TempDir;

mod settings;
use settings::Settings;
mod blog;
mod email;
mod filenames;
mod github;
mod image;
mod media;
mod mishaps;
mod signatureblock;
mod tag;
mod video;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::parse();

    env_logger::init();

    let working_dir = TempDir::new().expect("creating temporary directory");

    ensure_imagemagik_installed();

    let gh = Github::new(
        &settings.github_token,
        &settings.github_repo,
        &settings.github_branch,
    );

    let extract = |msg| email::extract(&settings, working_dir.path(), msg);

    match email::fetch(&settings) {
        Err(err) => stop("mailbox access", err), // Failed accessing mail box
        Ok(None) => complete(0),                 // No messages to process
        Ok(Some(mime_message)) => {
            match email::parse(&mime_message)
                .and_then(extract)
                .and_then(media::transcode)
            {
                Err(err) => stop("msg parse", err), // Message processing failed
                Ok(info) => match blog::write(&info) {
                    Err(err) => stop("Blog write", err),
                    Ok(markdown) => {
                        let commit_msg = format!("add post: {}", info.title);

                        let mut contents: Vec<NewContent> = info
                            .attachments
                            .iter()
                            .map(|a| NewContent::path(&a.github_path, &a.file_path))
                            .collect();
                        contents.push(NewContent::text(&info.file_path, &markdown));

                        gh.commit(&commit_msg, &contents).await?;
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
    info!("{}", num_msgs);
    std::process::exit(0)
}

fn ensure_imagemagik_installed() {
    if !image::imagemagic_installed() {
        panic!("Did not find ImageMagik");
    }
}
