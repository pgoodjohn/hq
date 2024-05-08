use clap::{Parser, Subcommand};
use log::debug;

mod db;
mod logger;
mod ssh;

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
    /// Connect to Cloud VM datbases
    Db(db::DbCommand,)
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
        }
        Some(Commands::Db(command)) => {
            db::command(&command);
        }
        None => {}
    }
}
