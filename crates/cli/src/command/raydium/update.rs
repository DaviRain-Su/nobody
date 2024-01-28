use crate::errors::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Update {}

impl Update {
    pub async fn run(&self) -> Result<(), Error> {
        let content = reqwest::get(crate::constant::RAYDIUM_API_PAIRS)
            .await
            .map_err(|e| Error::from(format!("failed get raydium pairs: Error({})", e)))?
            .text()
            .await
            .map_err(|e| Error::from(format!("failed get raydium pairs: Error({})", e)))?;

        let current_dir = std::env::current_dir().unwrap();
        println!("current_dir: {:?}", current_dir);
        let read_file_path = current_dir.join("fixed/raydium_paris.json");
        println!("read_file_path: {:?}", read_file_path);

        // // write to file for debug
        std::fs::write(read_file_path, content.clone()).map_err(|e| {
            Error::from(format!(
                "failed write raydium_pairs.json: Error({})",
                e.to_string()
            ))
        })?;

        Ok(())
    }
}
