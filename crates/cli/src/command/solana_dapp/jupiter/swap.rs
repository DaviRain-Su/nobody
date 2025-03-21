use super::token_list::TokenListType;
use super::token_list::Tokens;
use crate::errors::Error;
use crate::utils::get_config;
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::VersionedTransaction;
use std::env;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct JupyterSwap {
    /// input token name want to swap
    pub input_token_name: String,
    /// output token name want to swap
    pub output_token_name: String,
    /// input token amount
    pub input_amount: f64,
    /// slippage bps
    #[structopt(long, default_value = "50")]
    pub slippage_bps: u16,
}

impl JupyterSwap {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, payer, rpc_enpoint) = config
            .read_global_config()
            .map_err(|e| Error::from(format!("Error: {})", e.to_string())))?;
        let tokens = get_token_lists().map_err(|e| Error::from(e.to_string()))?;
        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);

        let input_token = tokens
            .address(&self.input_token_name)
            .map_err(|e| Error::from(format!("Error: {})", e.to_string())))?;
        let input_token_decimals = tokens
            .decimals(&self.input_token_name)
            .map_err(|e| Error::from(format!("Error: {})", e.to_string())))?;

        let output_token = tokens
            .address(&self.output_token_name)
            .map_err(|e| Error::from(format!("Error: {})", e.to_string())))?;

        let input_amount = (self.input_amount * 10f64.powi(input_token_decimals as i32)) as u64;
        let api_base_url = env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v6".into());

        let jupiter_swap_api_client = JupiterSwapApiClient::new(api_base_url);

        let quote_request = QuoteRequest {
            amount: input_amount,
            input_mint: input_token,
            output_mint: output_token,
            slippage_bps: self.slippage_bps,
            ..QuoteRequest::default()
        };

        // GET /quote
        let quote_response = jupiter_swap_api_client.quote(&quote_request).await.unwrap();

        // POST /swap
        let swap_response = jupiter_swap_api_client
            .swap(&SwapRequest {
                user_public_key: payer.pubkey(),
                quote_response: quote_response.clone(),
                config: TransactionConfig::default(),
            })
            .await
            .unwrap();

        let versioned_transaction: VersionedTransaction =
            bincode::deserialize(&swap_response.swap_transaction).unwrap();

        // Replace with a keypair or other struct implementing signer
        let signed_versioned_transaction =
            VersionedTransaction::try_new(versioned_transaction.message, &[&payer]).unwrap();

        // This will fail with "Transaction signature verification failure" as we did not really sign
        // this why we need to sign the transaction with the wallet
        let signature = rpc_client
            .send_and_confirm_transaction(&signed_versioned_transaction)
            .await;
        println!("🎉🎉🎉🎉{signature:?}🎉🎉🎉");

        let output_token_name = tokens
            .name(&output_token)
            .unwrap_or(output_token.to_string());
        let output_token_ata = spl_associated_token_account::get_associated_token_address(
            &payer.pubkey(),
            &output_token,
        );
        let output_token_balance = rpc_client
            .get_token_account_balance(&output_token_ata)
            .await
            .map_err(|e| Error::from(format!("Error: {}", e.to_string())))?;
        println!(
            "Token({}) Address({}) Decimals({})",
            output_token_name, output_token, output_token_balance.decimals
        );
        println!(
            "Address({}) have {} {}💰",
            payer.pubkey(),
            output_token_name,
            output_token_balance
                .amount
                .parse::<f64>()
                .map_err(|e| { Error::from(format!("Error :{}", e.to_string())) })?
                / 10f64.powi(output_token_balance.decimals as i32)
        );

        Ok(())
    }
}

pub fn get_token_lists() -> Result<Tokens, Error> {
    let current_dir =
        std::env::current_dir().map_err(|e| Error::from(format!("Error: {}", e.to_string())))?;
    log::info!("current_dir: {:?}", current_dir);
    let read_file_path = current_dir.join("token_list/solana-fm.csv");
    log::info!("read_file solana-fm.csv PATH {:?}", read_file_path);

    let mut token_list = vec![];
    let mut rdr = csv::Reader::from_path(read_file_path)
        .map_err(|e| Error::from(format!("Error : {}", e.to_string())))?;
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: TokenListType =
            result.map_err(|e| Error::from(format!("Error : {}", e.to_string())))?;
        token_list.push(record);
    }
    let tokens = token_list
        .into_iter()
        .filter(|t| !t.name.is_empty())
        .collect::<Vec<_>>();

    let tokens = Tokens::from_tokens(tokens);
    Ok(tokens)
}

#[test]
fn test_get_token_lists() {
    let tokens = get_token_lists();
    assert!(tokens.is_ok())
}
