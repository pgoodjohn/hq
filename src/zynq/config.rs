use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct ConfigCommand {
    #[clap(short, long, global(true))]
    debug: bool,

    #[clap(subcommand)]
    command: ConfigCommands
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set up authentication with the Zynq API
    Auth {
        #[clap(parse(try_from_str))]
        session_id: String,
    },
    /// Specify your preferred floor
    Floor,
    /// Specify your preferred desk
    Desk
}

pub fn command(config: &ConfigCommand) {

    match &config.command {
        ConfigCommands::Auth{ session_id } => {
            authenticate_command(&session_id);
        },
        ConfigCommands::Floor => {
            todo!("Build floor command");
        },
        ConfigCommands::Desk => {
            todo!("Build desk command");
        }
    }

}

fn authenticate_command(session_id: &String) {
    log::debug!("Saving session id {}", session_id);

    let config = super::configuration::Configuration::new(session_id);

    config.save();
}
