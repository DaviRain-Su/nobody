pub mod auto;
pub mod balance;
pub mod blocks;
pub mod clone;
pub mod demo;
pub mod generator;
pub mod jupiter_swap;
pub mod print;
pub mod raydium;
pub mod solana_rpc;
pub mod transfer;

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
    #[structopt(name = "balance")]
    Balance(balance::Balance),
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
    /// transfer
    #[structopt(name = "transfer")]
    Transfer(transfer::Transfer),
    /// blocks
    #[structopt(name = "blocks")]
    Blocks(blocks::Blocks),
    /// print
    #[structopt(name = "print")]
    Print(print::Print),
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
            Command::Balance(balance) => {
                balance.run().await?;
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
            Command::Transfer(transfer) => {
                transfer.run().await?;
                Ok(())
            }
            Command::Blocks(blocks) => {
                blocks.run().await?;
                Ok(())
            }
            Command::Print(print) => {
                print.run().await?;
                Ok(())
            }
        }
    }
}
