use crate::command::generator::{KeyPairs, KeyPairsString};
use crate::config::Config;
use crate::errors::Error;

pub fn get_config() -> Result<Config, Error> {
    let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".to_string()))?;
    let pomm_config_path = home_path.join(".config").join("nobody");
    let config_path = pomm_config_path.join("config.toml");

    let config_str = std::fs::read_to_string(config_path).map_err(Error::ReadFileFailed)?;
    let phoneix_config: Config = toml::from_str(&config_str).map_err(Error::ParseTomlFileFailed)?;

    Ok(phoneix_config)
}

pub fn get_all_keypairs(file_name: &str) -> Result<KeyPairs, Error> {
    let home_path = dirs::home_dir().ok_or(Error::Custom("can't open home dir".to_string()))?;
    let pomm_config_path = home_path.join(".config").join("nobody");
    let config_path = pomm_config_path.join(format!("{}_keypairs.json", file_name));

    let keypairs_str = KeyPairsString::read(config_path)?;
    let keypairs = KeyPairs::from(keypairs_str);
    Ok(keypairs)
}
