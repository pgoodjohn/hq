use serde::{Deserialize, Serialize};
use toml;

use super::config::Floors;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    pub session_id: Option<String>,
    pub preferred_seat_id: Option<String>,
    pub preferred_floor_id: Option<i32>,
}

impl Configuration {
    pub fn save(self: &Configuration) {
        let new_config_str = toml::to_string(self).expect("failed serialising config");

        std::fs::write(config_path(), new_config_str).expect("failed to write config");
    }

    pub fn load() -> Result<Self, String> {
        let config_path = config_path();
        match std::fs::read_to_string(config_path) {
            Ok(contents) => {
                let config: Configuration = toml::from_str(&contents).unwrap();

                return Ok(config);
            }
            Err(e) => {
                log::debug!("{}", e.to_string());
                return Err(
                    String::from("Could not load configuration. Please run hq zynq config auth to set up your authentication with the Zynq API")
                );
            }
        }
    }

    pub fn load_or_create() -> Result<Self, String> {
        let config_path = config_path();

        if config_path.exists() == false {
            let config = Configuration {
                session_id: None,
                preferred_seat_id: None,
                preferred_floor_id: None,
            };
            config.save();

            return Ok(config);
        }

        return Configuration::load();
    }

    pub fn floor(&self) -> Option<Floors> {
        match self.preferred_floor_id {
            Some(floor_api_value) => return Some(Floors::from_api_value(floor_api_value)),
            None => return None
        } 
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
