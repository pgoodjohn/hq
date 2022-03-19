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
    /// Manage SSH keys and such
    Ssh(SshCommand)
}

#[derive(Parser)]
#[clap(version, about)]
struct SshCommand {
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<SshCommands>,
}

#[derive(Subcommand)]
enum SshCommands {
    Generate {},
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
            ssh_command(&command);
        }
        None => {}
    }
}

fn ssh_command(ssh: &SshCommand) {
    println!("SSH Command!");
    print_is_debug(&ssh.debug);

    match ssh.command {
        Some(SshCommands::Generate {}) => {
            println!("Generating new SSH Key");
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