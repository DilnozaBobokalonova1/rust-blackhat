use crate::{config, Error};

use common::{api::{self, RegisterAgent}, crypto};

use ed25519_dalek::Signer;
use rand::RngCore;
use std::path::PathBuf;
use std::{convert::TryInto, fs};
use x25519_dalek::{x25519, X25519_BASEPOINT_BYTES};

pub fn init(api_client: &ureq::Agent) -> Result<config::Config, Error> {
    let saved_agent_id = get_saved_agent_config()?;

    let conf = match saved_agent_id {
        Some(agent_id) => agent_id,
        None => {
            let conf = register(api_client)?;
            save_agent_config(&conf)?;
            conf
        }
    };

    Ok(conf)
}

pub fn register(api_client: &ureq::Agent) -> Result<config::Config, Error> {

}

fn get_saved_agent_config() -> Result<Option<config::Config>, Error> {
    let agent_id_file = get_agent_config_file_path()?;

    if agent_id_file.exists() {
        let agent_file_content = fs::read(agent_id_file)?;

        let serialized_conf: config::SerializedConfig = serde_json::from_slice(&agent_file_content)?;
        let conf = serialized_conf.try_into()?;
        Ok(Some(conf))
    } else {
        Ok(None)
    }
}

pub fn get_agent_config_file_path() -> Result<PathBuf, Error> {
    let mut home_dir = match dirs::home_dir() {
        Some(home_dir) => home_dir,
        None => return Err(Error::Internal("Error getting home directory".to_string())),
    };

    home_dir.push(config::AGENT_ID_FILE);

    Ok(home_dir)
}

fn save_agent_config(conf: &config::Config) -> Result<(), Error> {
    let agent_config_file = get_agent_config_file_path()?;
    let serialized_config: config::SerializedConfig = conf.into(); // thanks to into from Conf
    let config_json = serde_json::to_string(&serialized_config)?;

    fs::write(agent_config_file, config_json.as_bytes())?;

    Ok(())
}