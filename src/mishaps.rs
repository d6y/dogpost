use rusoto_s3::PutObjectError;
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

    #[error(transparent)]
    S3(#[from] rusoto_core::RusotoError<PutObjectError>),

    #[error("No FROM address found")]
    MissingSender,

    #[error("Sender {0} not in allowed list of domains")]
    Unauthorised(String),
}
