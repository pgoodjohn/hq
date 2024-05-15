use clap::{Parser, Subcommand};
use regex::Regex;

mod gcloud;

mod interactive;
use interactive::*;

mod types;
use types::Application;
use types::Zone;


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

    let database_instance = gcloud::list(Application::new(&application), &Zone::new(&zone), project);

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
    let expired_auth_regex = Regex::new(r" - Request had invalid authentication credentials").unwrap();


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

    if let Some(_caps) = expired_auth_regex.captures(error_message) {
        log::warn!("Your authentication credentials have expired.");
        log::debug!("Error: {}", error_message);
        log::warn!("Please run \"gcloud auth login\" and try again.");
        return Ok(("".to_string(), "".to_string()));
    }

    log::warn!("Could not parse gcloud error: \n{}", error_message);
        
    Ok(("".to_string(), "".to_string()))
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
            let mut database_instance = gcloud::list(Application::new(&application), &Zone::new(zone), project).unwrap();
            gcloud::connect(port, &database_instance.pop().unwrap(), Zone::new(zone), project).unwrap();
            return;
        }
    };

    gcloud::connect(port, database_instance, Zone::new(zone), project).unwrap();
}

fn connect_to_db_interactive() {
    log::info!("Connecting to database interactively");

    let zone = ask_zone().unwrap();
    let project = ask_project().unwrap();
    let application = ask_application().unwrap();

    log::info!("Looking for databases to connect to");

    let available_databases = gcloud::list(application, &zone, &project).unwrap();

    let chosen_db = ask_db(available_databases).unwrap();

    let host_port = ask_host_port().unwrap();

    log::debug!("Connecting to DB with data: Zone: {} Host port: {} Project: {}, Chosen DB: {}", zone.as_str(), host_port, project, chosen_db);
    gcloud::connect(host_port, &chosen_db, zone, &project).unwrap();
}
