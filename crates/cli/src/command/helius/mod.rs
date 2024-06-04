use crate::errors::Error;
use crate::utils::get_config;
use chrono::{DateTime, Local, Utc};
use helius::types::*;
use helius::Helius;
use solana_client::rpc_client::GetConfirmedSignaturesForAddress2Config;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::Signature;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct NobodyHelius {
    /// 私钥文件的名字
    #[structopt(short, long)]
    pub file_name: Option<String>,
    /// display address
    #[structopt(short, long)]
    pub address: String,
}

impl NobodyHelius {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, _payer, rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;
        println!("rpc endpoint: {:?}", rpc_enpoint);
        let api_key = if let Some(index) = rpc_enpoint.find("api-key=") {
            let api_key = &rpc_enpoint[index + 8..];
            println!("截取的 API 密钥：{}", api_key);
            api_key
        } else {
            panic!("未找到 API 密钥");
        };

        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);
        let address = solana_sdk::pubkey::Pubkey::from_str(&self.address)?;
        let mut all_txs = Vec::new();
        let mut before = None;
        loop {
            let config = GetConfirmedSignaturesForAddress2Config {
                before,
                until: None,
                limit: Some(1000),
                commitment: Some(CommitmentConfig::confirmed()),
            };
            let mut result = rpc_client
                .get_signatures_for_address_with_config(&address, config)?
                .into_iter()
                .collect::<Vec<_>>();
            let last_signature = result.last();
            println!("last_signature: {:?}", last_signature);
            before = Some(Signature::from_str(
                &result
                    .last()
                    .ok_or(anyhow::anyhow!("get signatures is empty"))?
                    .signature
                    .clone(),
            )?);
            if result.len() < 1000 {
                all_txs.append(&mut result);
                break;
            } else {
                all_txs.append(&mut result);
                continue;
            }
        }
        println!("Address {} have {} transacition", address, all_txs.len());

        let all_txs = all_txs
            .into_iter()
            .filter(|tx| tx.err.is_none())
            .map(|item| item.signature)
            .collect::<Vec<_>>();

        println!(
            "Address {} have {} success transacition",
            address,
            all_txs.len()
        );

        let cluster: Cluster = Cluster::MainnetBeta;
        let helius: Helius = Helius::new(api_key, cluster).unwrap();

        let request: Vec<ParseTransactionsRequest> = ParseTransactionsRequest::from_slice(&all_txs);
        let mut counter = 0;
        for req in request {
            let response: Vec<EnhancedTransaction> = helius
                .parse_transactions(req)
                .await?
                .into_iter()
                .filter(|v| !v.description.is_empty()) // filter description is empty
                .filter(|v| !v.description.contains("nft")) // filter nft tx
                //.filter(|v| !v.description.contains("mint")) // filter mint tx
                .filter(|v| !v.description.contains("multiple")) // filter contain multiple tx
                .filter(|v| {
                    !v.description.contains("0.000000001 SOL")
                        && !v.description.contains("0 SOL")
                        && !v.description.contains("0.0001")
                }) // filet contain 0.000000001 SOL tx
                // .filter(|v| v.events.swap)
                .collect();

            counter += response.len();

            for (_idx, tx) in response.iter().enumerate() {
                let dt = DateTime::<Utc>::from_utc(
                    chrono::NaiveDateTime::from_timestamp(tx.timestamp as i64, 0),
                    Utc,
                );
                let local_dt = dt.with_timezone(&Local);
                println!("🌟{} 🌟🌟🌟 {}", local_dt, tx.description);
            }
        }
        println!("total have {} tx", counter);
        Ok(())
    }
}
