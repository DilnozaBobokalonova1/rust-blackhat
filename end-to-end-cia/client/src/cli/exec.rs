use std::time::Duration;

use common::api;
use uuid::Uuid;

use crate::{api::Client, config::Config, Error};

pub fn run(api_client: &Client, agent_id: &str, command: &str, conf: Config) -> Result<(), Error> {
    let agent_id = Uuid::parse_str(agent_id)?;
    let sleep_for = Duration::from_millis(550);

    let mut command_with_args: Vec<String> = command.split_whitespace()
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    if command_with_args.is_empty() {
        return Err(Error::Internal("Command is not valid".to_string()));
    }
    let command = command_with_args.remove(0);
    let args = command_with_args;
    let agent: api::Agent = api_client.get_agent(agent_id)?;
    let agent_identity_public_key = ed25519_dalek::PublicKey::from_bytes(&agent.identity_public_key)?;

    Ok(())
}
