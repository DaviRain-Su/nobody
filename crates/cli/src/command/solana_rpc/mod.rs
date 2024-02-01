pub mod block;
use reqwest::Client;

#[derive(Debug)]
pub struct NobodyClient {
    pub client: Client,
    pub url: String,
}

impl NobodyClient {
    pub fn new(url: &str) -> Self {
        NobodyClient {
            client: Client::new(),
            url: url.to_string(),
        }
    }
}
