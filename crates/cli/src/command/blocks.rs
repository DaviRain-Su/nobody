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

        loop {
            let slots = rpc_client
                .get_slot_with_commitment(CommitmentConfig::finalized())
                .await?;
            log::info!("slots: {:?}", slots);
            // 245283357
            // let block = rpc_client.get_block(226067836).await?;
            // log::info!("slots: {:?}", slots);
            let block = rpc_client.get_block(slots).await?;
            log::info!("block: {:?}", block);
        }
        // Ok(())
    }
}
