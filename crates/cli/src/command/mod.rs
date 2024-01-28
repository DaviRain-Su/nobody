pub mod auto;
pub mod clone;
pub mod demo;
pub mod generator;
pub mod get_sol_balance;
pub mod jupiter_swap;
pub mod raydium;

use crate::command::auto::Auto;
use generator::Generator;
use jupiter_swap::Jupyter;
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
    #[structopt(name = "jupyter")]
    Jupyter(Jupyter),
    /// generate new keypair
    #[structopt(name = "generator")]
    Generator(Generator),
    /// clone token list csv
    #[structopt(name = "clone")]
    Clone(clone::Clone),
    /// demo rpc
    #[structopt(name = "demo")]
    Demo(demo::Demo),
    /// raydium
    #[structopt(name = "raydium")]
    Raydium(raydium::Raydium),
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
            Command::Jupyter(swap) => {
                swap.run().await?;
                Ok(())
            }
            Command::Generator(generator) => {
                generator.run()?;
                Ok(())
            }
            Command::Clone(clone) => {
                clone.run().await?;
                Ok(())
            }
            Command::Demo(demo) => {
                demo.run().await?;
                Ok(())
            }
            Command::Raydium(raydium) => {
                raydium.run().await?;
                Ok(())
            }
        }
    }
}
