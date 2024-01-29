use crate::errors::Error;
use serde::{Deserialize, Deserializer};
use solana_cli_config::{Config as SolanaConfig, ConfigInput, CONFIG_FILE};
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use std::str::FromStr;

/// This is what we're going to decode into. Each field is optional, meaning
/// that it doesn't have to be present in TOML.
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Optionally include your keypair path. Defaults to your Solana CLI config file.
    pub keypair_path: Option<String>,
    /// Optionally include your RPC endpoint. Use "local", "dev", "main" for default endpoints. Defaults to your Solana CLI config file.
    pub rpc_endpoint: Option<String>,
    /// Optionally include a commitment level. Defaults to your Solana CLI config file.
    pub commitment: Option<String>,
}

impl Config {
    pub fn read_global_config(&self) -> anyhow::Result<(CommitmentConfig, Keypair, String)> {
        let (commitment, keypair_path, rpc_enpoint) =
            if let (Some(commitment), Some(keypair_path), Some(rpc_endpoint)) = (
                self.commitment.clone(),
                self.keypair_path.clone(),
                self.rpc_endpoint.clone(),
            ) {
                (commitment, keypair_path, rpc_endpoint)
            } else {
                let config = match CONFIG_FILE.as_ref() {
                    Some(config_file) => SolanaConfig::load(config_file).unwrap_or_else(|_| {
                        log::warn!("Failed to load config file: {}", config_file);
                        SolanaConfig::default()
                    }),
                    None => SolanaConfig::default(),
                };
                (config.commitment, config.keypair_path, config.json_rpc_url)
            };
        let commitment = ConfigInput::compute_commitment_config("", &commitment).1;
        let payer = get_payer_keypair_from_path(&keypair_path)?;
        let network_url = get_network(&rpc_enpoint).to_string();

        log::info!(
            "Commitment: {:?}, Payer: {}, Network URL: {}",
            commitment,
            payer.pubkey(),
            network_url
        );
        Ok((commitment, payer, network_url))
    }
}

pub fn parse_pubkey<'de, D>(deserializer: D) -> Result<Pubkey, D::Error>
where
    D: Deserializer<'de>,
{
    let pubkey_str = String::deserialize(deserializer)?;
    Pubkey::from_str(&pubkey_str).map_err(serde::de::Error::custom)
}

pub fn get_network(network_str: &str) -> &str {
    match network_str {
        "devnet" | "dev" | "d" => "https://api.devnet.solana.com",
        "mainnet" | "main" | "m" | "mainnet-beta" => "https://api.mainnet-beta.solana.com",
        "localnet" | "localhost" | "l" | "local" => "http://localhost:8899",
        _ => network_str,
    }
}

pub fn get_payer_keypair_from_path(path: &str) -> Result<Keypair, Error> {
    let path = &*shellexpand::tilde(path);
    read_keypair_file(path)
        .map_err(|e| Error::ReadKeypairFailed(format!("Failed to read keypair file: {:?}", e)))
}

#[test]
fn test_read_config() {
    let home_path = dirs::home_dir()
        .ok_or(anyhow::anyhow!("can't open home dir"))
        .unwrap();
    let config_path = home_path.join("/phoenix-onchain-mm/config.toml");
    let config_str = std::fs::read_to_string(config_path).unwrap();
    let config: Config = toml::from_str(&config_str).unwrap();

    println!("{:#?}", config);
}
