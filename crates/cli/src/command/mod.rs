pub mod auto;
pub mod generator;
pub mod get_sol_balance;
pub mod jupiter_swap;

use crate::command::auto::Auto;
use generator::Generator;
use jupiter_swap::JupyterSwap;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// auto generate config.toml file to ~/.config/pomm/config.toml
    #[structopt(name = "auto")]
    Auto(Auto),
    /// get balance
    #[structopt(name = "get-balance")]
    GetBalance(get_sol_balance::GetBalance),
    /// jupyter swap
    #[structopt(name = "swap")]
    Swap(JupyterSwap),
    /// generate new keypair
    #[structopt(name = "generator")]
    Generator(Generator),
}

#[derive(Debug, StructOpt)]
#[structopt(name = "nobody-cli")]
pub struct NobodyCli {
    #[structopt(subcommand)]
    pub command: Command,
}

impl NobodyCli {
    pub async fn run(&self) -> anyhow::Result<()> {
        match &self.command {
            Command::Auto(auto) => {
                let config_path = auto.run();
                println!("ConfigPath: {:?}", config_path);
                Ok(())
            }
            Command::GetBalance(get_balance) => {
                get_balance.run().await?;
                Ok(())
            }
            Command::Swap(swap) => {
                swap.run().await?;
                Ok(())
            }
            Command::Generator(generator) => {
                generator.run()?;
                Ok(())
            }
        }
    }
}
