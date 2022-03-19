use clap::{Parser, Subcommand};

mod key_generator;
mod key_adder;

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
    Add {},
}

pub fn command(ssh: &SshCommand) {
    println!("SSH Command!");
    super::utils::print_is_debug(&ssh.debug);

    match ssh.command {
        Some(SshCommands::Generate {}) => {
            key_generator::command().unwrap(); 
        }
        Some(SshCommands::Add {}) => {
            key_adder::command().unwrap();
        }
        None => {}
    }
}