use structopt::StructOpt;

pub mod swap;
pub mod token_list;

#[derive(Debug, StructOpt)]
pub enum Jupyter {
    #[structopt(name = "token-list")]
    TokenList(token_list::TokenList),
    #[structopt(name = "swap")]
    Swap(swap::JupyterSwap),
}

impl Jupyter {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            Jupyter::TokenList(token_list) => token_list.run().await?,
            Jupyter::Swap(swap) => swap.run().await?,
        }

        Ok(())
    }
}
