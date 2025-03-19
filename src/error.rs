#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Decode(serde_json::Error, String),
    Nonce(std::time::SystemTimeError),
    Base64DecodeError(base64::DecodeError),
    MacError(hmac::digest::InvalidLength),
    ApiKey(std::env::VarError),
    ApiSec(std::env::VarError),
    Other(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        match self {
            Self::ApiKey(e) => write!(fmt, "Please set API_KEY as environment variable {e}"),
            Self::ApiSec(e) => write!(fmt, "Please set API_SEC as environment variable {e}"),
            Self::Other(message) => write!(fmt, "Other error: {message}"),
            _ => write!(fmt, "{self:?}"),
        }
    }
}

impl std::error::Error for Error {}
