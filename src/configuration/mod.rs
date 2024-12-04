use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    db_configuration: DBConfiguration,
}

#[derive(Serialize, Deserialize)]
pub struct DBConfiguration {
    default_project: Option<String>,
    default_zone: Option<String>,
    filter_prefix: Option<String>
}

impl DBConfiguration {
    pub fn empty() -> DBConfiguration {
        return DBConfiguration {
            default_project: None,
            default_zone: None,
            filter_prefix: None,
        }
    }

    pub fn default() -> DBConfiguration {
        return DBConfiguration {
            default_project: None,
            default_zone: None,
            filter_prefix: Some(String::from("name~db-vm"))
        }
    }
}

impl Configuration {
    pub fn default() -> Configuration {
        return Configuration {
            db_configuration: DBConfiguration::default(),
        }
    }

    /// Try to load configuration, generate a default one if it's not present.
    pub fn initialize() -> Result<Configuration, _> {
        let         

    }

    fn load() -> Result<Configuration, _> {
        let file_path = Configuration::file_path();
    }

    fn save(&mut self) -> Result<Configuration, _> {
        let file_path = Configuration::file_path();
        let updated_config_string = toml::to_string_pretty(&self); // TODO: might need to clone config and use that intstead

        if let Some(parent) = file_path.parent() {
            std::fs::create_dir_all(parent).unwrap(); // Unwrap all the things, but will need to bubble up eventually
        }
        std::fs::write(&file_path, updated_config_string).unwrap();

        log::debug!("Configuration has been saved at {:?}", file_path);

        Ok(*self)
    }

    fn file_path() -> PathBuf {
        let mut file_path = PathBuf::new();

        if cfg!(debug_assertions) {
            file_path.push("/tmp/.hq/conf.toml");

            return file_path;
        }

        file_path.push(dirs::home_dir().unwrap()); // Why can this fail? How to handle??
        file_path.push(".hq/conf.toml");

        file_path
    }
}