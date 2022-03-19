use log::debug;
use requestty::Question;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::vec::Vec;
extern crate dirs;

pub fn command() -> Result<(), ()> {
    let key = ask_key().unwrap();
    debug!("Deleting {:?}", key);
    match delete_key(key) {
        Ok(_) => Ok(()),
        Err(_) => Ok(()),
    }
}

fn delete_key(key: Key) -> Result<(), KeyAddingError> {
    debug!("Deleting private key");
    Command::new("rm")
        .arg(String::from(&key.value))
        .spawn()
        .expect("ssh-add failed to run");

    let mut public_key_path = String::from(&key.value);
    let public_key_extension: &str = ".pub";
    public_key_path.push_str(public_key_extension);

    debug!("Deleting public key");
    Command::new("rm")
        .arg(public_key_path)
        .spawn()
        .expect("ssh-add failed to run");

    Ok(())
}

#[derive(Debug)]
struct KeyAddingError {}

fn ask_key() -> Result<Key, KeyParsingError> {
    let ssh_keys = list_ssh_keys().unwrap();
    let ssh_choices: Vec<String> = ssh_keys.into_iter().map(|x| x.value).collect();

    let question = Question::select("ssh_key")
        .message("Select key to delete")
        .choices(ssh_choices)
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let selected_key = &result.as_list_item().unwrap().text;

            let mut key: Vec<Key> = list_ssh_keys()
                .unwrap()
                .into_iter()
                .filter(|x| x.value.eq(selected_key))
                .collect();

            assert_eq!(key.len(), 1);

            match key.pop() {
                Some(result) => Ok(result),
                _ => Err(KeyParsingError {}),
            }
        }
        Err(_) => Err(KeyParsingError {}),
    }
}

#[derive(Debug)]
struct Key {
    value: String,
}

#[derive(Debug)]
struct KeyParsingError {}

fn list_ssh_keys() -> Result<Vec<Key>, io::Error> {
    let mut ssh_path = PathBuf::new();
    ssh_path.push(dirs::home_dir().unwrap());
    ssh_path.push(".ssh");

    Ok(fs::read_dir(ssh_path)
        .unwrap()
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| {
            let path = r.unwrap().path();
            let path_string = String::from(path.to_str().unwrap());
            Key { value: path_string }
        })
        .filter(|r| r.value.contains("id_") && r.value.contains(".pub") == false)
        .collect())
}
