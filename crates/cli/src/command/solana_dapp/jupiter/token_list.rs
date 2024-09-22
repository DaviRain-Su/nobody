use colored::*;
use solana_sdk::pubkey::Pubkey;
use std::fmt::Display;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct TokenList {}

#[derive(Debug, serde::Deserialize)]
pub struct TokenListType {
    #[serde(rename = "NAME")]
    pub name: String,
    #[serde(rename = "SYMBOL")]
    pub symbol: String,
    #[serde(rename = "ADDRESS")]
    pub address: String,
    #[serde(rename = "DECIMALS")]
    pub decimals: u64,
    #[serde(rename = "LOGOURI")]
    pub log_url: String,
    #[serde(rename = "VERIFIED")]
    pub verified: String,
}

impl Display for TokenListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "😼({}) 🤑({})", self.name.red(), self.symbol.blue())
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Tokens {
    pub tokens: Vec<TokenListType>,
}

impl Tokens {
    pub fn from_tokens(tokens: Vec<TokenListType>) -> Self {
        Self { tokens }
    }

    pub fn address(&self, token_symbol: &str) -> anyhow::Result<Pubkey> {
        log::info!("token_name: {}", token_symbol);
        let address = self
            .tokens
            .iter()
            .find(|t| t.symbol.to_lowercase() == token_symbol.to_lowercase())
            .map(|t| &t.address)
            .ok_or(anyhow::anyhow!("token not found"))?;

        Pubkey::from_str(&address).map_err(|e| anyhow::anyhow!(e))
    }

    pub fn decimals(&self, token_symbol: &str) -> anyhow::Result<u64> {
        let decimals = self
            .tokens
            .iter()
            .find(|t| t.symbol.to_lowercase() == token_symbol.to_lowercase())
            .map(|t| &t.decimals)
            .ok_or(anyhow::anyhow!("token not found"))?;

        Ok(*decimals)
    }

    pub fn name(&self, mint_token_addr: &Pubkey) -> anyhow::Result<String> {
        let name = self
            .tokens
            .iter()
            .find(|t| t.address == mint_token_addr.to_string())
            .map(|t| t.name.clone())
            .ok_or(anyhow::anyhow!("token not found"))?;

        Ok(name)
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for token in &self.tokens {
            writeln!(f, "{}", token)?;
        }
        Ok(())
    }
}

impl TokenList {
    pub async fn run(&self) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        log::info!("current_dir: {:?}", current_dir);
        let read_file_path = current_dir.join("token_list/solana-fm.csv");
        log::info!("read_file solana-fm.csv PATH {:?}", read_file_path);

        let mut token_list = vec![];
        let mut rdr = csv::Reader::from_path(read_file_path)?;
        for result in rdr.deserialize() {
            // Notice that we need to provide a type hint for automatic
            // deserialization.
            let record: TokenListType = result?;
            token_list.push(record);
        }
        let tokens = token_list
            .into_iter()
            .filter(|t| !t.name.is_empty())
            .collect::<Vec<_>>();

        let tokens = Tokens::from_tokens(tokens);
        println!("{}", tokens);
        Ok(())
    }
}
