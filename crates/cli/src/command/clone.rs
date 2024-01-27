use crate::errors::Error;
use structopt::StructOpt;
use tempfile::tempdir;

#[derive(Debug, StructOpt)]
pub struct Clone {}

impl Clone {
    pub async fn run(&self) -> Result<(), Error> {
        // URL of the file you want to download
        let url = "https://github.com/jup-ag/token-list/blob/main/src/partners/data/solana-fm.csv";

        // Send a GET request to the URL
        let response = reqwest::get(url)
            .await
            .map_err(|e| Error::from(e.to_string()))?;

        // Read the response body as bytes
        let content = response
            .bytes()
            .await
            .map_err(|e| Error::from(e.to_string()))?;

        // Create a temporary directory
        let dir = tempdir().map_err(|e| Error::from(e.to_string()))?;
        let file_path = dir.path().join("solana-fm.csv");

        // Write the content to a file in the temporary directory
        std::fs::write(&file_path, &content).map_err(|e| Error::from(e.to_string()))?;

        println!("File downloaded to: {}", file_path.display());

        Ok(())
    }
}
