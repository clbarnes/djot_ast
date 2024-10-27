use std::error::Error as StdError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("general error: {0}")]
    General(String),
    #[error("wrapped error: {0}")]
    Wrapper(#[from] Box<dyn StdError>),
}

impl Error {
    pub(crate) fn general<S: Into<String>>(msg: S) -> Self {
        Self::General(msg.into())
    }
}
