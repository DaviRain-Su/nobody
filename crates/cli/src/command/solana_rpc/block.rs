use super::NobodyClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::clock::{Slot, UnixTimestamp};
use solana_transaction_status::option_serializer::OptionSerializer;
use solana_transaction_status::{
    EncodedTransactionWithStatusMeta, UiInstruction, UiParsedInstruction,
};

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

impl Block {
    pub fn find_new_token(&self) {
        for transaction in &self.result.transactions {
            if let Some(meta) = &transaction.meta {
                if let OptionSerializer::Some(inner_ins) = &meta.inner_instructions {
                    for inner_in in inner_ins {
                        for ins in &inner_in.instructions {
                            if let UiInstruction::Parsed(UiParsedInstruction::Parsed(item)) = ins {
                                if item.program_id == "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
                                {
                                    log::info!(" Metaplex Token Metadata: Create Metadata Account V3: {:?}", item);
                                } else if item.program_id
                                    == "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
                                {
                                    continue;
                                } else if item.program_id == "11111111111111111111111111111111" {
                                    continue;
                                } else if item.program_id
                                    == "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
                                {
                                    continue;
                                } else if item.program_id
                                    == "Stake11111111111111111111111111111111111111"
                                {
                                    continue;
                                } else if item.program_id
                                    == "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"
                                {
                                    // log::info!("Token 2022 Program: {:?}", item);
                                    continue;
                                } else {
                                    log::info!("program_id: {}", item.program_id);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
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

    pub async fn check_block(&self, slot: Slot) -> anyhow::Result<String> {
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

        let text = response.text().await?;
        // log::info!("text: {}", text);

        Ok(text)
    }
}
