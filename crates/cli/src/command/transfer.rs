use crate::constant::LAMPORT;
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config};
use colored::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Transfer {
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
    /// is one to more
    #[structopt(short, long)]
    pub is_one_to_more: bool,
    /// transfer amount
    #[structopt(short, long)]
    pub amount: f64,
}

impl Transfer {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, payer, rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);
        let balance = rpc_client.get_balance(&payer.pubkey()).await?;
        log::info!("{} Balance: {}", payer.pubkey(), balance as f64 / LAMPORT);
        println!(
            "{} Balance: {}",
            payer.pubkey().to_string().red(),
            balance as f64 / LAMPORT
        );

        let keypairs = get_all_keypairs(&self.file_name)?;
        for keypair in keypairs.keypairs {
            let balance = rpc_client.get_balance(&keypair.pubkey()).await?;
            log::info!("{} Balance: {}", keypair.pubkey(), balance as f64 / LAMPORT);
            println!(
                "{} Balance: {}",
                keypair.pubkey().to_string().red(),
                balance as f64 / LAMPORT
            );
        }
        Ok(())
    }
}
