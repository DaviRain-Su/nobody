pub const LAMPORT: f64 = 1_000_000_000.0;

pub const DEFAULT_CONFIG_FILE: &str = r#"
# Optionally include your keypair path. Defaults to your Solana CLI config file.
keypair_path = "~/.config/solana/id.json"
# Optionally include your RPC endpoint. Use "local", "dev", "main" for default endpoints. Defaults to your Solana CLI config file.
rpc_endpoint = "https://api.mainnet-beta.solana.com"
# Optionally include a commitment level. Defaults to your Solana CLI config file.
commitment = "confirmed"
"#;
