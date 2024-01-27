use crate::constant::LAMPORT;
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config};
use colored::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct GetBalance {}

impl GetBalance {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, _payer, rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);

        let keypairs = get_all_keypairs()?;
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
