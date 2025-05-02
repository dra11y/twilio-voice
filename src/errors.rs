use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("unknown")]
    Unknown,
    #[error("env var: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse URL: {0}")]
    Url(#[from] url::ParseError),
}
