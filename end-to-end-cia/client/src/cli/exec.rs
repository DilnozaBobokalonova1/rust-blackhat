

use crate::api::Client;
use crate::config::Config;

pub fn run(api_client: &Client, agent_id: &str, command: &str, conf: Config) ->