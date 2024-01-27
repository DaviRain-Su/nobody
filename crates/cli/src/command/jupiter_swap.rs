use crate::errors::Error;
use crate::utils::get_config;
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signer::Signer;
use solana_sdk::{pubkey, transaction::VersionedTransaction};
use std::env;
use structopt::StructOpt;

// usdc
const USDC_MINT: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
// sol
const NATIVE_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

// pub const TEST_WALLET: Pubkey = pubkey!("2AQdpHJ2JpcEgPiATUXjQxA8QmafFegfQwSLWSprPicm"); // Coinbase 2 wallet

#[derive(Debug, StructOpt)]
pub struct JupyterSwap {}

impl JupyterSwap {
    pub async fn run(&self) -> anyhow::Result<()> {
        let config = get_config().map_err(|e| Error::from(e.to_string()))?;
        let (commitment, payer, rpc_enpoint) = config.read_global_config().map_err(|e| {
            let location = std::panic::Location::caller();
            Error::from(format!("Error({}): {})", location, e.to_string()))
        })?;

        let api_base_url = env::var("API_BASE_URL").unwrap_or("https://quote-api.jup.ag/v6".into());
        log::info!("Using base url: {}", api_base_url);

        let jupiter_swap_api_client = JupiterSwapApiClient::new(api_base_url);

        // swap sol(100_000_000/1_000_000_000 = 0.1) for usdc
        let sol_to_usdc_quote_request = QuoteRequest {
            amount: 10_000_000,
            input_mint: NATIVE_MINT,
            output_mint: USDC_MINT,
            slippage_bps: 50,
            ..QuoteRequest::default()
        };

        // swap usdc(10_000_000 / 1_000_000 = 10) for sol
        let usdc_to_sol_quote_request = QuoteRequest {
            amount: 10_000_000,
            input_mint: USDC_MINT,
            output_mint: NATIVE_MINT,
            slippage_bps: 50,
            ..QuoteRequest::default()
        };
        log::info!("{:#?}", sol_to_usdc_quote_request);

        // GET /quote
        let quote_response = jupiter_swap_api_client
            .quote(&usdc_to_sol_quote_request)
            .await
            .unwrap();
        log::info!("{quote_response:#?}");

        // POST /swap
        let swap_response = jupiter_swap_api_client
            .swap(&SwapRequest {
                user_public_key: payer.pubkey(),
                quote_response: quote_response.clone(),
                config: TransactionConfig::default(),
            })
            .await
            .unwrap();

        log::info!("Raw tx len: {}", swap_response.swap_transaction.len());

        let versioned_transaction: VersionedTransaction =
            bincode::deserialize(&swap_response.swap_transaction).unwrap();

        // Replace with a keypair or other struct implementing signer
        let signed_versioned_transaction =
            VersionedTransaction::try_new(versioned_transaction.message, &[&payer]).unwrap();

        // send with rpc client...
        let rpc_client = RpcClient::new_with_commitment(rpc_enpoint.to_string(), commitment);

        // This will fail with "Transaction signature verification failure" as we did not really sign

        // this why we need to sign the transaction with the wallet
        // how to resolve
        // 1. create a wallet
        // 2. fund the wallet
        // 3. sign the transaction with the wallet
        // 4. send the transaction
        // 5. check the balance
        // 6. check the transaction history
        let error = rpc_client
            .send_and_confirm_transaction(&signed_versioned_transaction)
            .await;
        println!("{error:?}");

        Ok(())
    }
}
