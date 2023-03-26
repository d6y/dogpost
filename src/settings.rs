use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Settings {
    /// IMAP hostname to connect to
    #[arg(long, default_value = "imap.gmail.com", env = "IMAP_HOSTNAME")]
    pub imap_hostname: String,

    /// IMAP port number
    #[arg(long, default_value = "993", env = "IMAP_PORT")]
    pub imap_port: u16,

    /// Email address (or user account) to check on the IMAP server
    #[arg(long, env = "IMAP_USER")]
    pub imap_user: String,

    /// Password for authentication
    #[arg(long, env = "IMAP_PASSWORD", hide_env_values = true)]
    pub imap_password: String,

    // The mailbox to read from
    #[arg(short, long, default_value = "INBOX", env = "MAILBOX")]
    pub mailbox: String,

    /// Archive the email after processing
    #[arg(short, long, env = "EXPURGE")]
    pub expunge: bool,

    /// Allow list of sender domains. If empty, all are allowed.
    #[arg(long, env = "DOMAINS_ALLOW")]
    pub allowed_domains: Vec<String>,

    /// Github bearer token
    #[arg(long, env = "GITHUB_TOKEN", hide_env_values = true)]
    pub github_token: String,

    /// Github repository in the form "user/repo"
    #[arg(long, env = "GITHUB_REPO")]
    pub github_repo: String,

    /// Github repository branch
    #[arg(long, env = "GITHUB_BRANCH", default_value = "main")]
    pub github_branch: String,

    /// Path in GitHub repostory for writing blog content (e.g., _posts)
    #[arg(long, env = "GITHUB_POST_PATH", default_value = "content/posts")]
    pub github_post_path: String,

    /// Path in GitHub repostory for writing image content (e.g., img/posts)
    #[arg(long, env = "GITHUB_MEDIA_PATH", default_value = "static/media/%Y/%m")]
    pub github_media_path: String,

    /// Path to media folder for images on the web server
    #[arg(long, env = "GITHUB_MEDIA_PATH", default_value = "/media/%Y/%m")]
    pub web_media_path: String,
}
