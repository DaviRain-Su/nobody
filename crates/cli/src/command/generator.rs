use crate::config::Config;
use crate::constant::DEFAULT_CONFIG_FILE;
use crate::errors::Error;
use solana_sdk::signer::keypair::{write_keypair, Keypair};
use solana_sdk::signer::{EncodableKey, Signer};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Generator {
    /// generator wallet numbers
    #[structopt(name = "size", short, long)]
    pub size: usize,
    ///
    #[structopt(short, long)]
    pub config_path: Option<PathBuf>,
}

impl Generator {
    pub fn run(&self) -> anyhow::Result<()> {
        let keypairs = KeyPairs::from_keypairs(
            (0..self.size)
                .map(|_i| Keypair::new())
                .collect::<Vec<Keypair>>(),
        );
        log::info!(
            "keypairs: {:?}",
            keypairs
                .keypairs
                .iter()
                .map(|k| k.pubkey())
                .collect::<Vec<_>>()
        );

        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyPairs {
    pub keypairs: Vec<Keypair>,
}

impl KeyPairs {
    pub fn new() -> Self {
        Self { keypairs: vec![] }
    }

    pub fn from_keypairs(keypairs: Vec<Keypair>) -> Self {
        Self { keypairs }
    }

    pub fn push(&mut self, keypair: Keypair) {
        self.keypairs.push(keypair);
    }
}
