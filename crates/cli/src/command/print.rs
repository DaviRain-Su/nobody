use crate::errors::Error;
use crate::utils::get_config;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Print {}

impl Print {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, payer, rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;

        println!("Commitment: {:?}", commitment);
        println!("Payer: {}", payer.to_base58_string());
        println!("RpcEndpoint: {:?}", rpc_enpoint);
        Ok(())
    }
}
