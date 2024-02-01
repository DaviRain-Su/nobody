use std::thread::sleep;

use crate::command::solana_rpc::NobodyClient;
use crate::errors::Error;
use crate::utils::get_config;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Blocks {}

impl Blocks {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, _payer, rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);

        let nobody_client = NobodyClient::new(&rpc_enpoint);
        let mut slots = rpc_client
            .get_slot_with_commitment(CommitmentConfig::finalized())
            .await?;

        loop {
            if let Ok(block) = nobody_client.get_block(slots).await {
                block.find_new_token();
                slots += 1;
                log::info!("slots: {:?}", slots);
                sleep(std::time::Duration::from_secs(1));
            } else {
                if let Ok(resut) = nobody_client.check_block(slots).await {
                    log::info!("check block: {:?}", resut);
                    sleep(std::time::Duration::from_secs(1));
                } else {
                    log::info!("parse failed");
                    sleep(std::time::Duration::from_secs(1));
                }
            }
        }
    }
}
