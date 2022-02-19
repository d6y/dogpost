use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(global_settings(&[structopt::clap::AppSettings::DeriveDisplayOrder]))]
pub struct Settings {
    /// IMAP hostname to connect to
    #[structopt(long, default_value = "imap.gmail.com", env = "IMAP_HOSTNAME")]
    pub imap_hostname: String,

    /// IMAP port number
    #[structopt(long, default_value = "993", env = "IMAP_PORT")]
    pub imap_port: u16,

    /// Email address (or user account) to check on the IMAP server
    #[structopt(long, env = "IMAP_USER")]
    pub imap_user: String,

    /// Password for authentication
    #[structopt(long, env = "IMAP_PASSWORD", hide_env_values = true)]
    pub imap_password: String,

    // The mailbox to read from
    #[structopt(short, long, default_value = "INBOX")]
    pub mailbox: String,

    /// For local testing, turn of certificate validation
    #[structopt(long)]
    pub imap_allow_untrusted: bool,

    /// Existing directory for writing blog content (e.g., _posts)
    #[structopt(long, env = "POSTS_DIR")]
    pub posts_dir: PathBuf,

    /// Temporary directory for writing media files prior to upload
    #[structopt(long, env = "MEDIA_DIR")]
    pub media_dir: PathBuf,

    /// Thumbnail width
    #[structopt(short, long, default_value = "500")]
    pub width: u16,

    /// Archive the email after processing
    #[structopt(short, long)]
    pub expunge: bool,

    /// S3 bucketname
    #[structopt(long, env = "S3_BUCKET")]
    pub s3_bucket: String,

    /// Allow list of sender domains. If empty, all are allowed.
    #[structopt(long, env = "DOMAINS_ALLOW")]
    pub allowed_domains: Vec<String>,

}
