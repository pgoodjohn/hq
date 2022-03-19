use clap::{Parser, Subcommand};

mod key_generator;
mod key_adder;
mod key_deleter;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct SshCommand {
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<SshCommands>,
}

#[derive(Subcommand)]
pub enum SshCommands {
    /// Generate a new SSH Key
    Generate {},
    /// Add a key to the running agent from ~/.ssh
    Add {},
    /// Delete a key from ~/.ssh
    Delete {},
}

pub fn command(ssh: &SshCommand) {
    match ssh.command {
        Some(SshCommands::Generate {}) => {
            key_generator::command().unwrap(); 
        }
        Some(SshCommands::Add {}) => {
            key_adder::command().unwrap();
        }
        Some(SshCommands::Delete {}) => {
            key_deleter::command().unwrap();
        }
        None => {}
    }
}
