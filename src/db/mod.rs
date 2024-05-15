use clap::{Parser, Subcommand};
use std::process::{Command, Stdio};
use regex::Regex;
use requestty::Question;

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
    /// Let H.Q.! guide you in finding the database.
    #[clap(alias = "i")]
    Interactive {},
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
        Some(DbCommands::Interactive {}) => {
            connect_to_db_interactive();
        }
        None => {}
    }
}

fn list_databases_command(application: &String, zone: &String, project: &String) {
    log::info!("Listing databases in zone {} for project {}", zone, project);

    let database_instance = find_database_instance(application, zone, project);

    match database_instance {
        Ok(instance) => log::info!("Database instance: {:?}", instance),
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

    log::warn!("Could not parse gcloud erro: \n{}", error_message);
        
    Ok(("".to_string(), "".to_string()))
}

fn find_database_instance(application: &String, zone: &String, project: &String) -> Result<Vec<String>, String> {
    let args = [
        "compute",
        "instances",
        "list",
        &format!("--filter=name~db-vm-{}", application),
        // "--limit=1",
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
    let output_str = split_multiline_to_vector(&String::from_utf8(output.stdout).unwrap_or_else(|_| String::new()));

    return Ok(output_str);
}

fn split_multiline_to_vector(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
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
            let mut database_instance = find_database_instance(application, zone, project).unwrap();
            connect_via_gcloud(port, &database_instance.pop().unwrap(), zone, project).unwrap();
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

fn connect_to_db_interactive() {
    log::info!("Connecting to database interactively");

    let region = ask_region().unwrap();
    let project = ask_project().unwrap();
    let application = ask_application().unwrap();

    log::info!("Looking for databases to connect to");

    let available_databases = find_database_instance(&application, &region, &project).unwrap();

    let chosen_db = ask_db(available_databases).unwrap();

    let host_port = ask_host_port().unwrap();

    log::debug!("Connecting to DB with data: Region: {} Host port: {} Project: {}, Chosen DB: {}", region, host_port, project, chosen_db);
    connect_to_database_command(&application, &Some(host_port), &Some(chosen_db), &region, &project)
}

fn ask_db(available_databases: Vec<String>) -> Result<String, String> {
    let question = Question::select("db")
        .message("Select key db to connect to")
        .choices(available_databases)
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = &result.as_list_item().unwrap().text;
            Ok(String::from(answer))
        }
        Err(_) => Err("Failed to get db from user".to_string()),
    }

}

fn ask_region() -> Result<String, String> {
    let question = Question::input("region")
        .message("What region is the DB you are looking for in?")
        .default("europe-west1-c")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            Ok(String::from(answer))
        }
        Err(_) => Err( "Failed to get region from user".to_string()),
    }
}

fn ask_host_port() -> Result<u16, String> {
    let question = Question::input("port")
        .message("On what port do you want the DB to be available on your host?")
        .default("13306")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            match answer.parse::<u16>() {
                Ok(port) => Ok(port),
                Err(_) => Err("Failed to parse port".to_string()),
            }
        }
        Err(_) => Err("Failed to get port from user".to_string()),
    }
}

fn ask_project() -> Result<String, String> {
    let question = Question::input("project")
        .message("What is the name of the project?")
        .default("mol-platform-prod")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            Ok(String::from(answer))
        }
        Err(_) => Err("Failed to get project name from user".to_string()),
    }
}

fn ask_application() -> Result<String, String> {
    let question = Question::input("application")
        .message("What is the name of the application?")
        .default("mollie")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            Ok(String::from(answer))
        }
        Err(_) => Err("Failed to get application name from user".to_string()),
    }
}