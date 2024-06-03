use crate::errors::Error;
use serde::{Deserialize, Serialize};
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Generator {
    /// generator wallet numbers
    #[structopt(short, long)]
    pub wallet_num: usize,
    /// keypair file name
    #[structopt(short, long)]
    pub file_name: String,
}

impl Generator {
    pub fn run(&self) -> anyhow::Result<()> {
        let keypairs = KeyPairs::from_keypairs(
            (0..self.wallet_num)
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

        let keypairs_str = KeyPairsString::from(keypairs);

        let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
        let nobody_config_path = home_path.join(".config").join("nobody");
        let keypairs_path = nobody_config_path.join(format!("{}_keypairs.json", self.file_name));
        keypairs_str.write(keypairs_path.clone())?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct KeyPairs {
    pub keypairs: Vec<Keypair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyPairsString {
    pub keypairs: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub pubkey: String,
    pub secret: String,
}

impl KeyPairsString {
    pub fn write(&self, path: PathBuf) -> Result<(), Error> {
        let temp_keypairs_str =
            serde_json::to_string(&self).map_err(|e| Error::SerializeFailed(e.to_string()))?;
        std::fs::write(path, temp_keypairs_str)
            .map_err(|e| Error::WriteFileFailed(e.to_string()))?;
        Ok(())
    }

    pub fn read(path: PathBuf) -> Result<Self, Error> {
        let temp_keypairs_str = std::fs::read_to_string(path).map_err(Error::ReadFileFailed)?;
        let keypairs_str =
            serde_json::from_str(&temp_keypairs_str).map_err(Error::DeserializeFailed)?;
        Ok(keypairs_str)
    }
}

impl From<KeyPairs> for KeyPairsString {
    fn from(keypairs: KeyPairs) -> Self {
        let keypairs = keypairs
            .keypairs
            .iter()
            .map(|k| {
                let raw_keypairs = k.to_bytes();
                Item {
                    pubkey: k.pubkey().to_string(),
                    secret: serde_json::to_string(&raw_keypairs.to_vec())
                        .expect("serde keypairs error"),
                }
            })
            .collect::<Vec<_>>();
        Self { keypairs }
    }
}

impl From<KeyPairsString> for KeyPairs {
    fn from(value: KeyPairsString) -> Self {
        let keypairs = value
            .keypairs
            .iter()
            .map(|k| {
                let raw_keypairs =
                    serde_json::from_str::<Vec<u8>>(&k.secret).expect("serde keypairs error");
                Keypair::from_bytes(&raw_keypairs).expect("keypairs from bytes error")
            })
            .collect::<Vec<_>>();
        Self { keypairs }
    }
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
