use crate::constant::LAMPORT;
use crate::errors::Error;
use crate::utils::get_config;
use colored::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct GetBalance {}

impl GetBalance {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, payer, rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);
        let last_block_hash = rpc_client.get_latest_blockhash().await?;
        log::info!("Last block hash : {:?}", last_block_hash);
        let balance = rpc_client.get_balance(&payer.pubkey()).await?;
        log::info!("{} Balance: {}", payer.pubkey(), balance as f64 / LAMPORT);
        println!(
            "{} Balance: {}",
            payer.pubkey().to_string().red(),
            balance as f64 / LAMPORT
        );
        Ok(())
    }
}
