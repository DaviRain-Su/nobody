use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config};
use colored::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::native_token::Sol;
use solana_sdk::signature::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Balance {
    /// 私钥文件的名字
    #[structopt(short, long)]
    pub file_name: String,
}

impl Balance {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, payer, rpc_enpoint) = config
            .read_global_config()
            .map_err(|e| Error::from(format!("Error: {}", e.to_string())))?;
        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);
        let balance = rpc_client.get_balance(&payer.pubkey()).await?;
        log::info!("地址 {} 有 {} Sol", payer.pubkey(), Sol(balance));
        println!(
            "地址 {} 有 {} Sol",
            payer.pubkey().to_string().red(),
            Sol(balance)
        );

        let keypairs = get_all_keypairs(&self.file_name)?;
        for keypair in keypairs.keypairs {
            let balance = rpc_client.get_balance(&keypair.pubkey()).await?;
            log::info!("地址 {} 有 {} Sol", keypair.pubkey(), Sol(balance));
            println!(
                "地址 {} 有 {} Sol",
                keypair.pubkey().to_string().red(),
                Sol(balance)
            );
        }
        Ok(())
    }
}
