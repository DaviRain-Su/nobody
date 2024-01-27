use crate::config::Config;
use crate::constant::DEFAULT_CONFIG_FILE;
use crate::errors::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Auto {
    /// config path for Phoenix onchain Maket Maker
    #[structopt(short, long)]
    config_path: Option<PathBuf>,
}

impl Auto {
    pub fn run(&self) -> anyhow::Result<PathBuf> {
        if let Some(config_path) = self.config_path.clone() {
            println!("enpter input config file");
            let config_str = std::fs::read_to_string(config_path.clone())
                .map_err(|e| Error::from(e.to_string()))?;
            assert!(toml::from_str::<Config>(&config_str).is_ok());
            Ok(config_path)
        } else {
            // open  config file path is  ~/.config/pomm/config.toml
            let home_path = dirs::home_dir().ok_or(anyhow::anyhow!("can't open home dir"))?;
            let pomm_config_path = home_path.join(".config").join("pomm");
            let config_path = pomm_config_path.join("config.toml");
            if std::fs::read_to_string(config_path.clone()).is_ok() {
                Ok(config_path)
            } else {
                std::fs::create_dir_all(pomm_config_path.clone())?;
                let config_path = pomm_config_path.join("config.toml");
                std::fs::write(config_path.clone(), DEFAULT_CONFIG_FILE)?;
                let config_str = std::fs::read_to_string(config_path.clone())
                    .map_err(|e| Error::from(e.to_string()))?;
                assert!(toml::from_str::<Config>(&config_str).is_ok());
                Ok(config_path)
            }
        }
    }
}
