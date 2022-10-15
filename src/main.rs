use mishaps::Mishap;
use structopt::StructOpt;

mod settings;
use settings::Settings;

mod blog;
mod email;
mod filenames;
mod image;
mod mishaps;
mod s3;
mod signatureblock;

use tokio::runtime::Runtime;

fn main() {
    let settings = Settings::from_args();

    let extract = |msg| email::extract(&settings, msg);

    if !settings.media_dir.exists() {
        std::fs::create_dir_all(&settings.media_dir).expect("creating media dir")
    };

    if !settings.posts_dir.exists() {
        std::fs::create_dir_all(&settings.posts_dir).expect("creating posts dir")
    };

    match email::fetch(&settings) {
        Err(err) => stop("mailbox access", err), // Failed accessing mail box
        Ok(None) => complete(0),                 // No messages to process
        Ok(Some(mime_message)) => {
            match email::parse(&mime_message).and_then(extract) {
                Err(err) => stop("msg parse", err), // Message processing failed
                Ok(info) => match blog::write(&info) {
                    Err(err) => stop("Blog write", err),
                    Ok(info) => {
                        let rt = Runtime::new().unwrap();
                        rt.block_on(s3::upload(&settings, info)).expect("s3 upload");
                        complete(1)
                    }
                },
            }
        }
    }
}

fn stop(context: &str, err: Mishap) -> ! {
    eprintln!("{context}: Failed: {err}", context = context, err = err);
    std::process::exit(1)
}

fn complete(num_msgs: usize) -> ! {
    println!("{}", num_msgs);
    std::process::exit(0)
}
