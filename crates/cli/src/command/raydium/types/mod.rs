use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq, Clone)]
pub struct RaydiumPair {
    pub name: String,
    #[serde(rename = "ammId")]
    pub amm_id: String,
    #[serde(rename = "lpMint")]
    pub lp_mint: String,
    #[serde(rename = "baseMint")]
    pub base_mint: String,
    #[serde(rename = "quoteMint")]
    pub quote_mint: String,
    pub market: String,
    pub liquidity: Option<f64>,
    pub volume24h: Option<f64>,
    #[serde(rename = "volume24hQuote")]
    pub volume24h_quote: Option<f64>,
    pub fee24h: Option<f64>,
    #[serde(rename = "fee24hQuote")]
    pub fee24h_quote: Option<f64>,
    pub volume7d: Option<f64>,
    #[serde(rename = "volume7dQuote")]
    pub volume7d_quote: Option<f64>,
    pub fee7d: Option<f64>,
    #[serde(rename = "fee7dQuote")]
    pub fee7d_quote: Option<f64>,
    pub volume30d: Option<f64>,
    #[serde(rename = "volume30dQuote")]
    pub volume30d_quote: Option<f64>,
    pub fee30d: Option<f64>,
    #[serde(rename = "fee30dQuote")]
    pub fee30d_quote: Option<f64>,
    pub price: Option<f64>,
    #[serde(rename = "lpPrice")]
    pub lp_price: Option<f64>,
    #[serde(rename = "tokenAmountCoin")]
    pub token_amount_coin: Option<f64>,
    #[serde(rename = "tokenAmountPc")]
    pub token_amount_pc: Option<f64>,
    #[serde(rename = "tokenAmountLp")]
    pub token_amount_lp: Option<f64>,
    pub apr24h: Option<f64>,
    pub apr7d: Option<f64>,
    pub apr30d: Option<f64>,
}

impl Display for RaydiumPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "name: {}", self.name)?;
        // writeln!(f, "amm_id: {}", self.amm_id)?;
        // writeln!(f, "lp_mint: {}", self.lp_mint)?;
        // writeln!(f, "base_mint: {}", self.base_mint)?;
        // writeln!(f, "quote_mint: {}", self.quote_mint)?;
        // writeln!(f, "market: {}", self.market)?;
        writeln!(f, "liquidity: {:?}", self.liquidity)?;
        writeln!(f, "volume24h: {:?}", self.volume24h)?;
        // writeln!(f, "volume24h_quote: {:?}", self.volume24h_quote)?;
        writeln!(f, "fee24h: {:?}", self.fee24h)?;
        // writeln!(f, "fee24h_quote: {:?}", self.fee24h_quote)?;
        writeln!(f, "volume7d: {:?}", self.volume7d)?;
        // writeln!(f, "volume7d_quote: {:?}", self.volume7d_quote)?;
        writeln!(f, "fee7d: {:?}", self.fee7d)?;
        // writeln!(f, "fee7d_quote: {:?}", self.fee7d_quote)?;
        writeln!(f, "volume30d: {:?}", self.volume30d)?;
        // writeln!(f, "volume30d_quote: {:?}", self.volume30d_quote)?;
        writeln!(f, "fee30d: {:?}", self.fee30d)?;
        // writeln!(f, "fee30d_quote: {:?}", self.fee30d_quote)?;
        writeln!(f, "price: {:?}", self.price)?;
        // writeln!(f, "lp_price: {:?}", self.lp_price)?;
        // writeln!(f, "token_amount_coin: {:?}", self.token_amount_coin)?;
        // writeln!(f, "token_amount_pc: {:?}", self.token_amount_pc)?;
        // writeln!(f, "token_amount_lp: {:?}", self.token_amount_lp)?;
        writeln!(f, "apr24h: {:?}", self.apr24h)?;
        writeln!(f, "apr7d: {:?}", self.apr7d)?;
        writeln!(f, "apr30d: {:?}", self.apr30d)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RaydiumPairs {
    pub pairs: Vec<RaydiumPair>,
}

impl Display for RaydiumPairs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for pair in &self.pairs {
            writeln!(f, "{}", pair)?;
        }
        Ok(())
    }
}

impl RaydiumPairs {
    pub fn new() -> Self {
        RaydiumPairs { pairs: vec![] }
    }
    pub fn from_vec(pairs: Vec<RaydiumPair>) -> Self {
        RaydiumPairs { pairs }
    }
    pub fn len(&self) -> usize {
        self.pairs.len()
    }
}

#[test]
fn test_raydiumpairs() {
    // read fixed file
    let current_dir = std::env::current_dir().unwrap();
    println!("current_dir: {:?}", current_dir);
    let read_file_path = current_dir.join("../../fixed/raydium.json");
    println!("read_file_path: {:?}", read_file_path);
    let content = std::fs::read_to_string(read_file_path).unwrap();

    let tokens: Vec<RaydiumPair> =
        serde_json::from_str(&content).expect("JSON was not well-formatted");
    assert_eq!(tokens.len(), 1);
}
