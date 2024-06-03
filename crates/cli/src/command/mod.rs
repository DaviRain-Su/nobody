pub mod auto;
pub mod balance;
pub mod clone;
pub mod generator;
pub mod jupiter_swap;
pub mod print;
pub mod raydium;
pub mod transfer;

use crate::command::auto::Auto;
use generator::Generator;
use jupiter_swap::Jupyter;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// auto generate config.toml file to ~/.config/nobody/config.toml
    #[structopt(name = "auto")]
    Auto(Auto),
    /// 获取账户的余额
    #[structopt(name = "balance")]
    Balance(balance::Balance),
    /// 使用jupyter 进行swap
    #[structopt(name = "jupyter")]
    Jupyter(Jupyter),
    /// 生成新的Solana Keypair
    #[structopt(name = "generator")]
    Generator(Generator),
    /// clone token list csv
    #[structopt(name = "clone")]
    Clone(clone::Clone),
    /// raydium
    #[structopt(name = "raydium")]
    Raydium(raydium::Raydium),
    /// transfer
    #[structopt(name = "transfer")]
    Transfer(transfer::Transfer),
    /// display config: Commitment, Keypair sercret key, RpcEndpoint
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
            Command::Raydium(raydium) => {
                raydium.run().await?;
                Ok(())
            }
            Command::Transfer(transfer) => {
                transfer.run().await?;
                Ok(())
            }
            Command::Print(print) => {
                print.run().await?;
                Ok(())
            }
        }
    }
}
