pub mod command;
pub mod config;
pub mod constant;
pub mod errors;
pub mod utils;

use command::NobodyCli;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let opt = NobodyCli::from_args();
    opt.run().await?;
    Ok(())
}
