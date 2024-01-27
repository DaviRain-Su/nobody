use crate::config::Config;
use crate::constant::DEFAULT_CONFIG_FILE;
use crate::errors::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Auto {}

impl Auto {
    pub fn run(&self) -> Result<(), Error> {
        // open  config file path is  ~/.config/pomm/config.toml
        let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".into()))?;
        let nobody_config_path = home_path.join(".config").join("nobody");

        if std::fs::read_to_string(nobody_config_path.join("config.toml")).is_ok()
            && std::fs::read_to_string(nobody_config_path.join("keypairs.json")).is_ok()
        {
            Ok(())
        } else {
            // create nobody_config_path
            std::fs::create_dir_all(nobody_config_path.clone()).map_err(|e| {
                Error::from(format!(
                    "failed create config path: Error({})",
                    e.to_string()
                ))
            })?;

            let config_path = nobody_config_path.join("config.toml");
            log::info!("config_path: {:?}", config_path);
            std::fs::write(config_path.clone(), DEFAULT_CONFIG_FILE).map_err(|e| {
                Error::from(format!(
                    "failed write config_path: Error({})",
                    e.to_string()
                ))
            })?;
            let keypairs_path = nobody_config_path.join("keypairs.json");
            std::fs::write(keypairs_path.clone(), "").map_err(|e| {
                Error::from(format!(
                    "failed write keypairs_path: Error({})",
                    e.to_string()
                ))
            })?;
            let config_str = std::fs::read_to_string(config_path.clone()).map_err(|e| {
                Error::from(format!(
                    "failed read config file from path: Error({})",
                    e.to_string()
                ))
            })?;
            assert!(toml::from_str::<Config>(&config_str).is_ok());
            Ok(())
        }
    }
}
