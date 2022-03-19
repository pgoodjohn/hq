use super::utils::{list_private_keys, SshKey};
use log::debug;
use requestty::Question;
use std::process::Command;
use std::vec::Vec;
extern crate dirs;

pub fn command() -> Result<(), ()> {
    let ssh_key = ask_key().unwrap();
    debug!("Deleting {:?}", ssh_key);
    match delete_key(ssh_key) {
        Ok(_) => Ok(()),
        Err(_) => Ok(()),
    }
}

fn delete_key(key: SshKey) -> Result<(), KeyAddingError> {
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

fn ask_key() -> Result<SshKey, KeyParsingError> {
    let ssh_keys = list_private_keys().unwrap();
    let ssh_choices: Vec<String> = ssh_keys.into_iter().map(|x| x.value).collect();

    let question = Question::select("ssh_key")
        .message("Select key to delete")
        .choices(ssh_choices)
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let selected_key = &result.as_list_item().unwrap().text;

            let mut key: Vec<SshKey> = list_private_keys()
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
struct KeyParsingError {}
