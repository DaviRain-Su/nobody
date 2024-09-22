use structopt::StructOpt;

pub mod jupiter;

#[derive(Debug, StructOpt)]
pub enum SolanaDapp {
    Jupiter(jupiter::Jupyter),
}

impl SolanaDapp {
    pub async fn run(&self) -> anyhow::Result<()> {
        match self {
            SolanaDapp::Jupiter(jupiter) => jupiter.run().await,
        }
    }
}
