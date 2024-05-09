use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};
use regex::Regex;

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
    log::info!("Listing databases in zone {} for project {}", zone, project);

    let database_instance = find_database_instance(application, zone, project);

    match database_instance {
        Ok(instance) => log::info!("Database instance: {}", instance),
        Err(e) => {
            log::debug!("Failed to find database instance: {}", e);
            parse_gcloud_error(&e).unwrap();
        }
    }
}

fn parse_gcloud_error(error_message: &str) -> Result<(String, String), &'static str> {
    // Regex pattern to find the project name and missing permission
    let permissions_regex = Regex::new(r" - Required '(.+)' permission for 'projects/(.+)'").unwrap();
    let auth_error_pattern = Regex::new(
        r"You do not currently have an active account selected.|Please run:\s+\$ gcloud auth login"
    ).unwrap();

    if let Some(caps) = permissions_regex.captures(error_message) {
        let permission = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let project_name = caps.get(2).map_or("", |m| m.as_str()).to_string();

        if !permission.is_empty() && !project_name.is_empty() {
            log::warn!("You are missing some permissions to run this command.");
            log::warn!("Permission needed: {}", permission);
            log::warn!("Project name: {}", project_name);
            return Ok((project_name, permission));
        } else {
            return Err("Failed to parse necessary details from the error message.");
        }
    } 
    if let Some(_caps) = auth_error_pattern.captures(error_message) {
        log::warn!("You are not authenticated with the google cloud sdk");
        log::warn!("Please run \"gcloud auth login\" and try again.");
        return Ok(("".to_string(), "".to_string()));
    }
        
    Err("No matching error pattern found.")
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

    // Keep all the otuput in memory
    command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());

    // Spawn the command
    let spawned_command = match command.spawn() {
        Ok(process) => process,
        Err(e) => return Err(format!("Failed to spawn gcloud command: {}", e)),
    };

    // Capture output
    let output = match spawned_command.wait_with_output() {
        Ok(output) => {
            log::debug!("Output: {:?}", output);
            output
        },
        Err(e) => return Err(format!("Failed to read output: {}", e)),
    };

     if output.status.success() == false {
        log::debug!("{:?}", output);

        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("gcloud command failed:\n{}", err_msg));
    }

    // Successful output
    let output_str = String::from_utf8(output.stdout).unwrap_or_else(|_| String::new());

    return Ok(output_str.trim_end().to_string())
}

fn connect_to_database_command(application: &String, port: &Option<u16>, instance: &Option<String>, zone: &String, project: &String) {
    log::info!("Connecting you to a database");

    let port = match port {
        Some(port) => *port,
        None => 13306,
    };

    let database_instance = match instance {
        Some(ref inst) => inst,
        None => {
            let database_instance = find_database_instance(application, zone, project).unwrap();
            connect_via_gcloud(port, &database_instance, zone, project).unwrap();
            return;
        }
    };

    connect_via_gcloud(port, database_instance, zone, project).unwrap();
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
        return Err(format!("Command failed:\n{}", err_msg));
    }

    let _output_str = String::from_utf8(output.stdout).unwrap_or_else(|_| String::new());
    // Ok(output_str);

    Ok(())
}
