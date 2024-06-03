use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config};
use solana_sdk::signer::Signer;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Print {
    /// file name
    #[structopt(long)]
    pub file_name: Option<String>,
}

impl Print {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (_commitment, payer, _rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;

        println!(
            "账户地址: {}， 账户私钥: {}",
            payer.pubkey(),
            payer.to_base58_string()
        );

        if let Some(file_name) = &self.file_name {
            println!(
                "-------------------------------------------------------------------- 文件 {}_keypair.json 中的账户 ------------------------------------------------",
                file_name
            );
            let keypairs = get_all_keypairs(file_name)?;
            for keypair in keypairs.keypairs {
                println!(
                    "账户地址: {}， 账户私钥: {}",
                    keypair.pubkey(),
                    keypair.to_base58_string()
                );
            }
        }
        Ok(())
    }
}
