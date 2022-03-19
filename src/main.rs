use clap::{Parser, Subcommand};
mod ssh;
mod utils;

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
    /// Check that everything is working
    Test {},
    /// Manage SSH keys and such
    Ssh(ssh::SshCommand)
}


fn main() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        true => println!("Enabling debug mode"),
        false => {}
    }

    match cli.command {
        Some(Commands::Test {}) => {
            test_cli_command(&cli);
        }
        Some(Commands::Ssh(command) )=> {
            ssh::command(&command);
        }
        None => {}
    }
}

fn test_cli_command(cli: &Cli) {
    utils::print_is_debug(&cli.debug)
}