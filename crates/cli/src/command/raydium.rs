use crate::errors::Error;
use structopt::StructOpt;

pub mod display;
pub mod types;
pub mod update;

#[derive(Debug, StructOpt)]
pub enum Raydium {
    /// display raydium pairs keys
    #[structopt(name = "display")]
    Display(display::Display),
    /// update raydium pairs
    #[structopt(name = "update-pairs")]
    UpdatePairs(update::Update),
}

impl Raydium {
    pub async fn run(&self) -> Result<(), Error> {
        match &self {
            Raydium::Display(display) => {
                display.run().await?;
            }
            Raydium::UpdatePairs(update) => {
                update.run().await?;
            }
        }
        Ok(())
    }
}
