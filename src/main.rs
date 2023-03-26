use clap::Parser;
use github::Github;
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
mod mishaps;
mod signatureblock;

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
            match email::parse(&mime_message).and_then(extract) {
                Err(err) => stop("msg parse", err), // Message processing failed
                Ok(info) => match blog::write(&info) {
                    Err(err) => stop("Blog write", err),
                    Ok(content) => {
                        let path_name =
                            format!("{}/{}", &settings.github_post_path, info.file_path);
                        let commit_msg = format!("add post: {}", info.title);
                        gh.commit(&path_name, &content, &commit_msg).await?;
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
