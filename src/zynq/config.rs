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
    /// Set up all the necessary values for the Zynq API to work
    Init,
    /// Specify your preferred desk
    Desk,
}

pub fn command(config: &ConfigCommand) -> Result<(), super::ZynqCommandError>{
    match &config.command {
        ConfigCommands::Auth { session_id } => {
            authenticate_command(&session_id);
            Ok(())
        }
        ConfigCommands::Floor => {
            floor_command();
            Ok(())
        },
        ConfigCommands::Init => {
            match init_command() {
                Ok(_) => {
                    log::info!("Configuration was successful, you are ready to book a desk with hq zynq book");
                    return Ok(())
                }
                Err(e) => {
                    log::error!("Something went wrong with configuring the Zynq CLI");
                    log::error!("{}", e);
                    log::error!("Please try again");

                    return Err(e);
                }
            }
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

fn floor_command() {
    log::debug!("Setting preferred floor");

    let mut config = super::configuration::Configuration::load_or_create()
        .expect("could not load / create configuration file");

    match config.floor() {
        Some(floor) => log::info!("Currently preferred floor is {}", floor.to_string()),
        None => log::info!("No preferred floor is currently set, let's set it up!"),
    }

    let preferred_floor_answer = ask_preferred_floor_question();

    match preferred_floor_answer {
        Ok(floor) => {
            config.preferred_floor_id = Some(floor.api_values());
        }
        Err(e) => {
            panic!("{:?}", e);
        }
    }

    config.save();

}

fn ask_preferred_floor_question() -> Result<Floors, super::ZynqCommandError> {
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
            return Ok(chosen_value);
        }
        Err(e) => {
            return Err(super::ZynqCommandError::new("wtf"));
        }
    }
}

fn init_command() -> Result<(), super::ZynqCommandError>{
    log::debug!("Running interactive configuration for the Zynq Command");

    let mut config = super::configuration::Configuration::load_or_create()
        .expect("could not load / create configuration file");

    let session_id_answer = ask_session_id_question();

    match session_id_answer {
        Ok(session_id) => {
            log::debug!("{:?}", session_id);
            config.session_id = Some(session_id);
        },
        Err(_e) => {
            return Err(super::ZynqCommandError::new("Please retry with a valid session id"))
        }
    }

    let preferred_floor_answer = ask_preferred_floor_question();

    match preferred_floor_answer {
        Ok(floor) => {
            config.preferred_floor_id = Some(floor.api_values());
        }
        Err(_e) => {
            return Err(super::ZynqCommandError::new("Something went wrong with setting up the preferred floor, try again"));
        }
    }

    config.save();

    Ok(())
}

fn ask_session_id_question() -> Result<String, String> {

    let session_id_question = Question::input("session_id")
        .message("What is your Session ID? To find it, you will need to inspect the cookies of one of the requests to the Zynq API made by your browser after having logged in.")
        .build();

    let session_id_answer = requestty::prompt_one(session_id_question);

    match session_id_answer {
        Ok(result) => {
            let session_id_string = String::from(result.as_string().unwrap());

            if session_id_string.len() == 0 {
                return Err(String::from("Please input a valid value"))
            }

            return Ok(String::from(result.as_string().unwrap()));
        },
        Err(_e) => return Err(String::from("Please input a valid value"))
    } 
}
