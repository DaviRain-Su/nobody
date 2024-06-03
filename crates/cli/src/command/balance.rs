use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config};
use colored::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::native_token::Sol;
use solana_sdk::signature::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Balance {
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
}

impl Balance {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, payer, rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);
        let balance = rpc_client.get_balance(&payer.pubkey()).await?;
        let sol_balance = Sol(balance);
        log::info!("地址 {} 有 {} SOL", payer.pubkey(), sol_balance);
        println!(
            "地址 {} 有 {} SOL",
            payer.pubkey().to_string().red(),
            sol_balance
        );

        let keypairs = get_all_keypairs(&self.file_name)?;
        for keypair in keypairs.keypairs {
            let balance = rpc_client.get_balance(&keypair.pubkey()).await?;
            let sol_balance = Sol(balance);
            log::info!("地址 {} 有 {} SOL", keypair.pubkey(), sol_balance);
            println!(
                "地址 {} 有 {} SOL",
                keypair.pubkey().to_string().red(),
                sol_balance
            );
        }
        Ok(())
    }
}
