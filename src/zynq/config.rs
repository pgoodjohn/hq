use clap::{Parser, Subcommand};
use requestty::Question;
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Parser)]
#[clap(version, about, arg_required_else_help(true))]
pub struct ConfigCommand {
    #[clap(short, long, global(true))]
    debug: bool,

    #[clap(subcommand)]
    command: ConfigCommands,
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set up authentication with the Zynq API
    Auth {
        #[clap(parse(try_from_str))]
        session_id: String,
    },
    /// Specify your preferred floor
    Floor,
    /// Specify your preferred desk
    Desk,
}

pub fn command(config: &ConfigCommand) {
    match &config.command {
        ConfigCommands::Auth { session_id } => {
            authenticate_command(&session_id);
        }
        ConfigCommands::Floor => {
            floor_command();
        }
        ConfigCommands::Desk => {
            todo!("Build desk command");
        }
    }
}

fn authenticate_command(session_id: &String) {
    log::debug!("Saving session id {}", session_id);

    let mut config = super::configuration::Configuration::load_or_create()
        .expect("could not load / create config file");

    config.session_id = Some(String::from(session_id));

    config.save();
}

#[derive(Debug, PartialEq, EnumString, strum_macros::ToString)]
pub enum Floors {
    #[strum(serialize = "Ground Floor")]
    GroundFloor,
    #[strum(serialize = "First Floor")]
    FirstFloor,
    #[strum(serialize = "Second Floor")]
    SecondFloor,
    #[strum(serialize = "Third Floor")]
    ThirdFloor,
}

impl Floors {
    pub fn api_values(&self) -> i32 {
        match self {
            Floors::GroundFloor => 561,
            Floors::FirstFloor => 560,
            Floors::SecondFloor => 559,
            Floors::ThirdFloor => 558,
        }
    }

    pub fn from_api_value(api_value: i32) -> Self {
        match api_value {
            561 => Floors::GroundFloor,
            560 => Floors::FirstFloor,
            559 => Floors::SecondFloor,
            558 => Floors::ThirdFloor,
            _ => panic!("Unrecognized floor api value {}", api_value),
        }
    }
}

/*
impl std::fmt::Display for Floors {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.to_string())
        }
}
*/

fn floor_command() {
    log::debug!("Setting preferred floor");

    let mut config = super::configuration::Configuration::load_or_create()
        .expect("could not load / create configuration file");

    match config.floor() {
        Some(floor) => log::info!("Currently preferred floor is {}", floor.to_string()),
        None => log::info!("No preferred floor is currently set, let's set it up!"),
    }

    let question = Question::select("preferred_floor")
        .message("Select your preferred floor")
        .choices(vec![
            Floors::GroundFloor.to_string(),
            Floors::FirstFloor.to_string(),
            Floors::SecondFloor.to_string(),
            Floors::ThirdFloor.to_string(),
        ])
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let chosen_value = Floors::from_str(&result.as_list_item().unwrap().text).unwrap();
            log::debug!("{:?}", &chosen_value);
            config.preferred_floor_id = Some(chosen_value.api_values());
            config.save();
        }
        Err(e) => {
            panic!("{}", e)
        }
    }
}
