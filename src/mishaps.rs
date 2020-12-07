use thiserror::Error;

use imap;
use mailparse;
use native_tls;

#[derive(Error, Debug)]
pub enum Mishap {
    #[error(transparent)]
    Network(#[from] native_tls::Error),

    #[error(transparent)]
    Imap(#[from] imap::error::Error),

    #[error(transparent)]
    Email(#[from] mailparse::MailParseError),

    #[error("Bad email field: {0}")]
    EmailField(String),

    #[error(transparent)]
    File(#[from] std::io::Error),
}
