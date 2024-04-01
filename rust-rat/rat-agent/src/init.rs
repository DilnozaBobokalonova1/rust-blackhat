use crate::{consts, Error};
use common::api;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

pub fn init(api_client: &ureq::Agent) -> Result<Uuid, Error> {

    let saved_agent_id = get_saved_agent_id()?;



}

pub fn register(api_client: &ureq::Agent) -> Result<Uuid, Error> {


    //replace with actual code
    let agent_id: Uuid = Uuid::new_v4();
    Ok(agent_id)
}

pub fn get_saved_agent_id() -> Result<Option<Uuid>, Error> {
    let agent_id_file = get_agent_id_file_path()?;

    if agent_id_file.exists() {
        let agent_file_content = fs::read(agent_id_file)?;
        let agent_id = Uuid::from_slice(&agent_file_content)?;
        Ok(Some(agent_id))
    } else {
        Ok(None)
    }
}
pub fn get_agent_id_file_path() -> Result<PathBuf, Error> {
    let mut home_dir = match dirs::home_dir() {
        Some(home_dir) => home_dir,
        None => return Err(Error::Internal("Error whole trying to get home directory.".to_string())),
    };

    home_dir.push(consts::AGENT_ID_FILE);

    Ok(home_dir)
}

pub fn save_agent_id(agent_id: Uuid) -> Result<(), Error> {
    let agent_id_file = get_agent_id_file_path()?;
    fs::write(agent_id_file, agent_id.as_bytes())?;

    Ok(())
}
