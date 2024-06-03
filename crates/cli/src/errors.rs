use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("custom error: {0}")]
    Custom(String),
    #[error("Read file failed")]
    ReadFileFailed(#[from] std::io::Error),
    #[error("Write to File failed")]
    WriteFileFailed(String),
    #[error("Parse toml file failed")]
    ParseTomlFileFailed(#[from] toml::de::Error),
    #[error("Deserialize failed")]
    DeserializeFailed(#[from] serde_json::Error),
    #[error("Serialize failed")]
    SerializeFailed(String),
    #[error("Read Keypair Failed: {0}")]
    ReadKeypairFailed(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error::Custom(s)
    }
}
