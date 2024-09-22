use structopt::StructOpt;

pub mod swap;
pub mod swap_by_pubkey;
pub mod token_list;

#[derive(Debug, StructOpt)]
pub enum Jupyter {
    #[structopt(name = "token-list")]
    TokenList(token_list::TokenList),
    #[structopt(name = "swap")]
    Swap(swap::JupyterSwap),
    #[structopt(name = "swap-by-pubkey")]
    SwapByPubkey(swap_by_pubkey::JupyterSwapByPubkey),
}

impl Jupyter {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Jupyter::TokenList(token_list) => token_list.run().await?,
            Jupyter::Swap(swap) => swap.run().await?,
            Jupyter::SwapByPubkey(swap_by_pubkey) => swap_by_pubkey.run().await?,
        }

        Ok(())
    }
}
