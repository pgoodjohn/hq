use super::utils::{list_private_keys, SshKey};
use log::debug;
use requestty::Question;
use std::process::{Command, Stdio};
use std::vec::Vec;
extern crate dirs;

pub fn command() -> Result<(), ()> {
    let key = ask_key().unwrap();
    debug!("Adding key {:?}", key);
    match add_key(key) {
        Ok(_) => Ok(()),
        Err(_) => Ok(()),
    }
}

fn add_key(key: SshKey) -> Result<(), KeyAddingError> {
    let command = Command::new("ssh-add")
        .arg("-K")
        .arg(key.value)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("ssh-add failed to run");

    command.wait_with_output().unwrap();

    Ok(())
}

#[derive(Debug)]
struct KeyAddingError {}

fn ask_key() -> Result<SshKey, KeyParsingError> {
    let ssh_keys = list_private_keys().unwrap();
    let ssh_choices: Vec<String> = ssh_keys.into_iter().map(|x| x.value).collect();

    let question = Question::select("ssh_key")
        .message("Select key to add")
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
