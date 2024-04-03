use prettytable::{Cell, Row, Table};
use crate::{api, Error};

pub fn run(api_client: &api::Client) -> Result<(), Error> {
    let agents = api_client.list_agents()?;

    Ok(())
}