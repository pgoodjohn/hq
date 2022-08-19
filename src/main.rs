use clap::{Parser, Subcommand};
use log::debug;
mod logger;
mod ssh;
mod zynq;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,

    #[clap(short, long)]
    debug: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage SSH keys and such
    Ssh(ssh::SshCommand),
    /// Do things with Zynq
    Zynq(zynq::ZynqCommand),
}

fn main() {
    let cli = Cli::parse();

    logger::init(cli.debug);

    if cli.debug {
        debug!("Debug mode enabled");
    }

    match cli.command {
        Some(Commands::Ssh(command)) => {
            ssh::command(&command);
        },
        Some(Commands::Zynq(command)) => {
            zynq::command(&command)
        } 
        None => {}
    }
}
