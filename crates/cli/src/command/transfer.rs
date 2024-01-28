use crate::constant::LAMPORT;
use crate::errors::Error;
use crate::utils::{get_all_keypairs, get_config};
use colored::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::rpc_config::RpcSendTransactionConfig;
use solana_sdk::signature::Signer;
use solana_sdk::system_transaction;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Transfer {
    /// keypair file name
    #[structopt(long)]
    pub file_name: String,
    /// is one to more
    #[structopt(long)]
    pub is_one_to_more: bool,
    /// transfer amount
    #[structopt(long)]
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
        log::info!(
            "{} Balance: {}",
            payer.pubkey(),
            balance as f64 / LAMPORT as f64
        );
        println!(
            "{} Balance: {}",
            payer.pubkey().to_string().red(),
            balance as f64 / LAMPORT as f64
        );

        let keypairs = get_all_keypairs(&self.file_name)?;
        if self.is_one_to_more {
            for keypair in keypairs.keypairs {
                let balance = rpc_client.get_balance(&keypair.pubkey()).await?;
                log::info!(
                    "{} Balance: {}",
                    keypair.pubkey(),
                    balance as f64 / LAMPORT as f64
                );
                println!(
                    "{} Balance: {}",
                    keypair.pubkey().to_string().red(),
                    balance as f64 / LAMPORT as f64
                );
                // Transfer lamports from Alice to Bob
                let latest_blockhash = rpc_client.get_latest_blockhash().await?;
                let lamports = self.amount * LAMPORT as f64;
                let tx = system_transaction::transfer(
                    &payer,
                    &keypair.pubkey(),
                    lamports as u64,
                    latest_blockhash,
                );
                let config = RpcSendTransactionConfig {
                    skip_preflight: true,
                    ..RpcSendTransactionConfig::default()
                };
                let signature = rpc_client.send_transaction_with_config(&tx, config).await?;
                println!(
                    "🎉🎉 {} --> {} ({} SOL) : Signature({})🎉🎉",
                    payer.pubkey(),
                    keypair.pubkey(),
                    self.amount,
                    signature
                );
            }
        } else {
            for keypair in keypairs.keypairs {
                let balance = rpc_client.get_balance(&keypair.pubkey()).await?;
                log::info!(
                    "{} Balance: {}",
                    keypair.pubkey(),
                    balance as f64 / LAMPORT as f64
                );
                println!(
                    "{} Balance: {}",
                    keypair.pubkey().to_string().red(),
                    balance as f64 / LAMPORT as f64
                );
                // Transfer lamports from Alice to Bob
                let latest_blockhash = rpc_client.get_latest_blockhash().await?;
                let lamports = (self.amount * LAMPORT as f64) * 0.99;
                // let lamports = (balance as f64 * 0.99) as u64;
                let tx = system_transaction::transfer(
                    &keypair,
                    &payer.pubkey(),
                    lamports as u64,
                    latest_blockhash,
                );
                let config = RpcSendTransactionConfig {
                    skip_preflight: true,
                    preflight_commitment: Some(
                        solana_sdk::commitment_config::CommitmentLevel::Finalized,
                    ),
                    ..RpcSendTransactionConfig::default()
                };
                if let Ok(signature) = rpc_client.send_transaction_with_config(&tx, config).await {
                    println!(
                        "🎉🎉 {} --> {} ({} SOL) : Signature({})🎉🎉",
                        payer.pubkey(),
                        keypair.pubkey(),
                        lamports as f64 / LAMPORT as f64,
                        signature
                    );
                } else {
                    println!(
                        "😭😭 {} --> {} ({} SOL) : Signature({})😭😭",
                        payer.pubkey(),
                        keypair.pubkey(),
                        lamports as f64 / LAMPORT as f64,
                        "Transfer failed".red()
                    );
                }
            }
        }

        Ok(())
    }
}
