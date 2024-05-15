use requestty::Question;

use super::types::*;

pub fn ask_db(available_databases: Vec<String>) -> Result<String, String> {
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

pub fn ask_zone() -> Result<Zone, String> {
    let question = Question::input("region")
        .message("What region is the DB you are looking for in?")
        .default("europe-west1-c")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            Ok(Zone::new(answer))
        }
        Err(_) => Err("Failed to get region from user".to_string()),
    }
}

pub fn ask_host_port() -> Result<u16, String> {
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

pub fn ask_project() -> Result<String, String> {
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

pub fn ask_application() -> Result<Application, String> {
    let question = Question::input("application")
        .message("What is the name of the application?")
        .default("mollie")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            Ok(Application::new(answer))
        }
        Err(_) => Err("Failed to get application name from user".to_string()),
    }
}
