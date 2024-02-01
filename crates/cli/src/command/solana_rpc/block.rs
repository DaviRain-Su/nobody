use super::NobodyClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::clock::{Slot, UnixTimestamp};
use solana_transaction_status::EncodedTransactionWithStatusMeta;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncodedConfirmedBlock {
    pub previous_blockhash: String,
    pub blockhash: String,
    pub parent_slot: Slot,
    pub transactions: Vec<EncodedTransactionWithStatusMeta>,
    pub block_time: Option<UnixTimestamp>,
    pub block_height: Option<u64>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub id: usize,
    pub jsonrpc: String,
    pub result: EncodedConfirmedBlock,
}

impl NobodyClient {
    pub async fn get_block(&self, slot: Slot) -> anyhow::Result<Block> {
        log::info!("get block");
        let request_body = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getBlock",
            "params": [slot, {
                "encoding": "jsonParsed",
                "maxSupportedTransactionVersion": 0,
                "transactionDetails": "full",
                "rewards": false
            }]
        });

        // 发送 POST 请求
        let response = self
            .client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let response = response.json::<Block>().await?;

        Ok(response)
    }
}
