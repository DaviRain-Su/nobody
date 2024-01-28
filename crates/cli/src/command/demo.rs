use crate::errors::Error;
use crate::utils::get_config;
use colored::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey;
use solana_sdk::pubkey::Pubkey;
use structopt::StructOpt;
use time::OffsetDateTime;

// usdc
pub const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
// sol
pub const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

#[derive(Debug, StructOpt)]
pub struct Demo {}

impl Demo {
    pub async fn run(&self) -> Result<(), Error> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, _payer, rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);

        // let supply = rpc_client.supply().await.map_err(|e| {
        //     let location = std::panic::Location::caller();
        //     Error::from(format!("Error({}): {})", location, e.to_string()))
        // })?;
        // println!("Total supply: {}", supply.value.total);
        // println!("Circulating: {}", supply.value.circulating);
        // println!("NonCirculating: {}", supply.value.non_circulating);

        let url = rpc_client.url();
        println!("RPC URL: {}", url.red());

        let version = rpc_client.get_version().await.map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        println!("RPC Version: {}", version.solana_core.to_string().red());

        let usdc_supply = rpc_client.get_token_supply(&USDC_MINT).await.map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        println!(
            "USDC({}) Supply: {}",
            USDC_MINT,
            usdc_supply.amount.parse::<f64>().unwrap() / 1e6
        );
        // let wsol_supply = rpc_client
        //     .get_token_supply(&NATIVE_MINT)
        //     .await
        //     .map_err(|e| {
        //         let location = std::panic::Location::caller();
        //         Error::from(format!("Error({}): {})", location, e.to_string()))
        //     })?;
        // println!(
        //     "WSOL({}) Supply: {}",
        //     NATIVE_MINT,
        //     wsol_supply.amount.to_string().red()
        // );

        let block_height = rpc_client.get_block_height().await.map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        println!("Block height: {}", block_height.to_string().red());

        let slot = rpc_client.get_slot().await.map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        println!("Slot: {}", slot.to_string().red());

        // Get the time of the most recent finalized block
        let block_time = rpc_client.get_block_time(slot).await.map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        let time = OffsetDateTime::from_unix_timestamp(block_time).map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        println!("Block time: {}", time.to_string().red());

        let cluster_nodes = rpc_client.get_cluster_nodes().await.map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        println!("Cluster nodes Length: {}", cluster_nodes.len());

        let epoch_info = rpc_client.get_epoch_info().await.map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        println!("Epoch info: {:#?}", epoch_info);

        Ok(())
    }
}
