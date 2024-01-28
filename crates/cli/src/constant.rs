pub const LAMPORT: u64 = 1_000_000_000;

pub const DEFAULT_CONFIG_FILE: &str = r#"
# Optionally include your keypair path. Defaults to your Solana CLI config file.
keypair_path = "~/.config/solana/id.json"
# Optionally include your RPC endpoint. Use "local", "dev", "main" for default endpoints. Defaults to your Solana CLI config file.
rpc_endpoint = "https://api.mainnet-beta.solana.com"
# Optionally include a commitment level. Defaults to your Solana CLI config file.
commitment = "confirmed"
"#;

// raydium
pub const RAYDIUM_API_PAIRS: &str = "https://api.raydium.io/v2/main/pairs";
