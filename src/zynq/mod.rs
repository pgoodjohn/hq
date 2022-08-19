use clap::{Parser, Subcommand};
use serde::{Serialize, Deserialize};
use reqwest::{cookie::Jar, Url};
use toml;
extern crate dirs;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct ZynqCommand {
    #[clap(short, long)]
    debug: bool,

    #[clap(subcommand)]
    command: Option<ZynqCommands>,
}

#[derive(Subcommand)]
pub enum ZynqCommands {
    /// Book a desk
    Book {},
    // See bookable desks
    // List {},
    // Cancel a day's booking
    // Cancel {},
    /// Configurate authentication with the Zynq API
    Auth {
        #[clap(short, long)]
        session_id: String,
    },
}

pub fn command(zynq: &ZynqCommand) {
    match zynq.command.as_ref() {
        Some(ZynqCommands::Book {}) => {
            book_desk_command();      
        }
        Some(ZynqCommands::Auth { session_id }) => {
            authenticate_command(&session_id);
        }
        None => {}
    }

}

#[derive(Debug, Serialize)]
struct MultidayBookRequest {
    floorplanID: i32,
    days: Vec<String>,
    preferredSeatID: i32,
    resourceType: String,
    start: String,
    end: String,
    bookingLengthDays: i32,
}

fn book_desk_command() {
    log::debug!("running book desk command");

    let mut days = Vec::new();
    days.push(String::from("2022-09-16"));
    days.push(String::from("2022-09-17"));

    let request_body = MultidayBookRequest {
        floorplanID: 558,
        days: days,
        preferredSeatID: 20606,
        resourceType: String::from("DESK"),
        start: String::from("08:00"),
        end: String::from("17:00"),
        bookingLengthDays: 0,
    };

    log::debug!("Making request to the Zynq API with body: {:?}", request_body);

    let config = Config::from_file(&config_path());

    let cookie = format!("sessionid={}; Domain=zynq.io", config.session_id);
    let url = "https://zynq.io".parse::<Url>().unwrap();

    let jar = Jar::default();
    jar.add_cookie_str(&cookie, &url);

    let cookie_store = std::sync::Arc::new(jar);

    let client = reqwest::blocking::Client::builder()
        .cookie_provider(cookie_store)
        .build().expect("failed to build http client");
    let res = client.post("https://zynq.io/seating/api/book_multiday")
        .json(&request_body)
        .send();

    log::debug!("Got response: {:?}", res.expect("could not unwrap").text());
    
}

fn authenticate_command(session_id: &String) {
    log::debug!("saving session id");
   let config = Config { session_id: String::from(session_id) };

   config.save();

}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Config {
    session_id: String
}

impl Config {
    fn save(self: &Config) {
        let new_config_str =
            toml::to_string(self).expect("failed serialising config");

        std::fs::write(config_path(), new_config_str).expect("failed to write config");
    }

    fn from_file(config_path: &std::path::Path) -> Self {
        let contents = std::fs::read_to_string(config_path).expect("could not read config file");

        let config: Config = toml::from_str(&contents).expect("Could not parse config");

        config
    }
}

fn config_path() -> std::path::PathBuf {
    let mut config_path = std::path::PathBuf::new();

    if cfg!(debug_assertions) {
        config_path.push("/tmp/.hq/config/");
    } else {
        config_path.push(dirs::home_dir().unwrap());
        config_path.push(".hq/config/");
    }

    std::fs::create_dir_all(&config_path).expect("could not create config directory");

    config_path.push("zynq.toml");

    config_path
}
