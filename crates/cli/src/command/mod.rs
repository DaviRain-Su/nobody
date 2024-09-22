pub mod auto;
pub mod balance;
pub mod clone;
pub mod generator;
pub mod print;
pub mod solana_dapp;
pub mod transfer;

use crate::command::auto::Auto;
use generator::Generator;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Command {
    /// 生成默认的配置文件 config.toml 在 ~/.config/nobody/config.toml
    #[structopt(name = "auto")]
    Auto(Auto),
    /// 获取账户的余额
    #[structopt(name = "balance")]
    Balance(balance::Balance),
    /// about solana dapp
    #[structopt(name = "dapp")]
    SolanaDapp(solana_dapp::SolanaDapp),
    /// 生成新的Solana Keypair
    #[structopt(name = "generator")]
    Generator(Generator),
    /// 支持一对多，多对一 Sol 代币转移
    #[structopt(name = "transfer")]
    Transfer(transfer::Transfer),
    /// 打印账户的Pubkey和Private Key
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
            Command::Auto(auto) => auto.run().map_err(Into::into),
            Command::Balance(balance) => balance.run().await,
            Command::SolanaDapp(dapp) => dapp.run().await,
            Command::Generator(generator) => generator.run(),
            Command::Transfer(transfer) => transfer.run().await,
            Command::Print(print) => print.run().await,
        }
    }
}
