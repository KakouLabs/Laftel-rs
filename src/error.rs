use thiserror::Error;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum LaftelError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, LaftelError>;
