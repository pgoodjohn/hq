use serde::{Deserialize, Serialize};
use toml;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    pub session_id: String,
}

impl Configuration {
    pub fn new(session_id: &String) -> Self {
        Configuration {
            session_id: String::from(session_id),
        }
    }

    pub fn save(self: &Configuration) {
        let new_config_str = toml::to_string(self).expect("failed serialising config");

        std::fs::write(config_path(), new_config_str).expect("failed to write config");
    }

    pub fn load() -> Self {
        let config_path = config_path();
        let contents =
            std::fs::read_to_string(config_path).expect("Could not read configuration file");

        let config: Configuration =
            toml::from_str(&contents).expect("Failed to read configuration file contents");

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
