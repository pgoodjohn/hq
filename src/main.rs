use clap::{Parser, Subcommand};
mod ssh;

#[derive(Parser)]
#[clap(version, about)]
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
    print_is_debug(&cli.debug)
}

fn print_is_debug(cli_debug: &bool) {
    match cli_debug {
        true => println!("Debug mode is on"),
        false => println!("Debug mode is off")
    }
}