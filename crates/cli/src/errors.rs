use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("custom error: {0}")]
    Custom(String),
    #[error("Read Keypair Failed: {0}")]
    ReadKeypairFailed(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Custom(s)
    }
}
