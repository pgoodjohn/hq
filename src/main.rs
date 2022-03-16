use clap::{Parser, Subcommand};

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
}

fn main() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        true => println!("Enabling debug mode"),
        false => {}
    }

    match &cli.command {
        Some(Commands::Test {}) => {
            test_cli_command(&cli);
        }
        None => {}
    }
}

fn test_cli_command(cli: &Cli) {
    match cli.debug {
        true => println!("Debug mode is on"),
        false => println!("Debug mode is off")
    }
}