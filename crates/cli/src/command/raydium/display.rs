use super::types::{RaydiumPair, RaydiumPairs};
use crate::errors::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Display {}

impl Display {
    pub async fn run(&self) -> Result<(), Error> {
        let current_dir = std::env::current_dir().unwrap();
        println!("current_dir: {:?}", current_dir);
        let read_file_path = current_dir.join("fixed/raydium_paris.json");
        println!("read_file_path: {:?}", read_file_path);

        let content = std::fs::read_to_string(read_file_path.clone()).map_err(|e| {
            Error::from(format!(
                "failed read raydium_pairs.json: Error({})",
                e.to_string()
            ))
        })?;

        let pairs: Vec<RaydiumPair> = serde_json::from_str(&content)
            .map_err(|e| Error::from(format!("failed parse raydium pairs: Error({:?})", e)))?;
        let mut pairs = pairs
            .into_iter()
            .filter(|p| !p.name.contains("unknown") && !p.name.starts_with("-"))
            .collect::<Vec<RaydiumPair>>();
        pairs.sort_by(|a, b| a.volume24h.partial_cmp(&b.volume24h).unwrap());
        // take top 10
        pairs.reverse();
        pairs.truncate(10);
        let raydium_pairs = RaydiumPairs::from_vec(pairs);
        println!(
            "raydium_pairs: {} len: {}",
            raydium_pairs,
            raydium_pairs.len()
        );
        Ok(())
    }
}
