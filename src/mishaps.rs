use thiserror::Error;

#[derive(Error, Debug)]
pub enum Mishap {
    #[error(transparent)]
    Imap(#[from] imap::error::Error),

    #[error(transparent)]
    Email(#[from] mailparse::MailParseError),

    #[error("Bad email field: {0}")]
    EmailField(String),

    #[error(transparent)]
    PostEncoding(#[from] std::string::FromUtf8Error),

    #[error(transparent)]
    File(#[from] std::io::Error),

    #[error("No FROM address found")]
    MissingSender,

    #[error("Sender {0} not in allowed list of domains")]
    Unauthorised(String),
}
