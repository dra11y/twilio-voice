pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown")]
    Unknown,
    #[error("env var: {0}")]
    EnvVar(#[from] std::env::VarError),
    #[error("I/O: {0}")]
    Io(#[from] std::io::Error),
    #[error("Parse URL: {0}")]
    Url(#[from] url::ParseError),
    #[error("Digits: {0}")]
    Digits(#[from] DigitsError),
    #[error("Response deserialization error: {0}, raw xml: {1}")]
    ResponseDeser(String, String),
}

#[derive(Debug, thiserror::Error, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DigitsError {
    #[error("is empty")]
    Empty,
    #[error("first digit is not numeric")]
    FirstDigitNotNumeric,
    #[error("does not contain any numeric digits")]
    NoNumeric,
    #[error("contains an alphabetic digit")]
    ContainsAlphabetic,
    #[error("numeric digits are broken by a non-numeric digit")]
    NumericAfterNonNumeric,
    #[error("numeric overflow")]
    Overflow,
    #[error("invalid character: {0}")]
    InvalidCharacter(char),
    #[error("negative number: {0}")]
    NegativeNumber(i128),
}
