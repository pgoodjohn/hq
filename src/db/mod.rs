use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct DbCommand {
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<DbCommands>,
}

#[derive(Subcommand)]
pub enum DbCommands {
    /// List available databases for project
    List {
        #[clap(long)]
        application: String,

        #[clap(long)]
        zone: String,

        #[clap(long)]
        project: String,
    },
    /// Connect to a specific database
    Connect {
        #[clap(long)]
        application: String,

        #[clap(short='p', long="port")]
        port: Option<u16>,

        #[clap(short, long)]
        instance: Option<String>,
        
        #[clap(long)]
        zone: String,

        #[clap(long)]
        project: String,
    },
}

pub fn command(command: &DbCommand) {
    match &command.command {
        Some(DbCommands::List {application, zone, project}) => {
            list_databases_command(application, zone, project);
        }
        Some(DbCommands::Connect {application, port, instance, zone, project}) => {
            connect_to_database_command(application, port, instance, zone, project);
        }
        None => {}
    }
}

fn list_databases_command(application: &String, zone: &String, project: &String) {
    println!("Listing databases in zone {} for project {}", zone, project);

    let database_instance = find_database_instance(application, zone, project).unwrap();
    log::info!("Database instance: {}", database_instance);
}

fn find_database_instance(application: &String, zone: &String, project: &String) -> Result<String, String> {
    let args = [
        "compute",
        "instances",
        "list",
        &format!("--filter=name~db-vm-{}", application),
        "--limit=1",
        &format!("--zones={}", zone),
        &format!("--project={}", project),
        "--format=value(name)",
    ];

    log::debug!("Running: gcloud {}", args.join(" "));

    let mut command = Command::new("gcloud");
    for arg in args.iter() {
        command.arg(arg);
    }

    command.stdin(Stdio::piped()).stdout(Stdio::piped());

    // Spawn the command
    let command = match command.spawn() {
        Ok(process) => process,
        Err(e) => return Err(format!("Failed to spawn gcloud command: {}", e)),
    };

    // Wait for the command to complete and capture the output
    let output = match command.wait_with_output() {
        Ok(output) => output,
        Err(e) => return Err(format!("Failed to read output: {}", e)),
    };

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed: {}", err_msg));
    }

    let output_str = String::from_utf8(output.stdout).unwrap_or_else(|_| String::new());

    Ok(output_str)
}

fn connect_to_database_command(application: &String, port: &Option<u16>, instance: &Option<String>, zone: &String, project: &String) {
    log::info!("Connecting you to a database");

    connect_via_gcloud(13306 as u16, &find_database_instance(application, zone, project).unwrap(), zone, project).unwrap();
}

fn connect_via_gcloud(port: u16, instance: &String,  zone: &String, project: &String) -> Result<(), String> {
    let args = [
        "compute",
        "start-iap-tunnel",
        &format!("{}", instance.trim_end()),
        "3306",
        &format!("--local-host-port=localhost:{}", port),
        &format!("--project={}", project),
        &format!("--zone={}", zone),
    ];

    log::debug!("Running: gcloud {}", args.join(" "));

    let mut command = Command::new("gcloud");
    for arg in args.iter() {
        command.arg(arg);
    }

    // Spawn the command
    let command = match command.spawn() {
        Ok(process) => process,
        Err(e) => return Err(format!("Failed to spawn gcloud command: {}", e)),
    };

    // Wait for the command to complete and capture the output
    let output = match command.wait_with_output() {
        Ok(output) => output,
        Err(e) => return Err(format!("Failed to read output: {}", e)),
    };

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed: {}", err_msg));
    }

    let output_str = String::from_utf8(output.stdout).unwrap_or_else(|_| String::new());
    // Ok(output_str);

    Ok(())
}
