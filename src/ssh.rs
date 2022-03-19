use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, about)]
pub struct SshCommand {
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<SshCommands>,
}

#[derive(Subcommand)]
pub enum SshCommands {
    Generate {},
}

pub fn command(ssh: &SshCommand) {
    println!("SSH Command!");
    print_is_debug(&ssh.debug);

    match ssh.command {
        Some(SshCommands::Generate {}) => {
            println!("Generating new SSH Key");
        }
        None => {}
    }

}

fn print_is_debug(cli_debug: &bool) {
    match cli_debug {
        true => println!("Debug mode is on"),
        false => println!("Debug mode is off")
    }
}