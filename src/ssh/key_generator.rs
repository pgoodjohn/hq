use requestty::{Question};
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::process::Command;
use std::path::PathBuf;
use std::{thread, time}; 
use chrono::prelude::*;

pub fn command() -> Result<(), &'static str> {
    println!("Generating new SSH Key");
    let encryption_type = ask_encryption_type().unwrap();
    let comment = ask_comment().unwrap();
    let path = ask_path(&encryption_type).unwrap();
    let password = ask_password().unwrap();

    println!("Running SSH Keygen: ssh-keygen -t {} -C \"{}\" -f {}", encryption_type, comment.text, path.value);

    let mut ssh_keygen_command = Command::new("ssh-keygen");

    ssh_keygen_command
        .arg("-t")
        .arg(format!("{}", encryption_type))
        .arg("-C")
        .arg(comment.text)
        .arg("-f")
        .arg(path.path_value.to_str().unwrap())
        .arg("-N")
        .arg(password.value);
    
    if encryption_type == EncryptionType::Rsa {
        ssh_keygen_command.arg("-b").arg("4096");
    }

    ssh_keygen_command.spawn().expect("ssh-keygen failed to runr");

    thread::sleep(time::Duration::from_millis(1000));
    Ok(())
}

fn ask_password() -> Result<Password, PasswordParsingError> {
    let question = Question::password("password")
        .message("Add passphrase (leave blank for none)")
        .mask('*')
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            Ok(Password{value: String::from(answer)})
        }
        Err (_) => {
            Err(PasswordParsingError{})
        }
    }
}

struct Password {
    value: String,
}

#[derive(Debug)]
struct PasswordParsingError {

}

fn ask_path(encryption_type: &EncryptionType) -> Result<Path, PathParsingError> {
    let default_ssh_path = format!("~/.ssh/id_{}_{}", encryption_type, Utc::now().format("%Y-%m-%d").to_string());
    let question = Question::input("path")
        .message("Path")
        .default(default_ssh_path)
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            let mut path = PathBuf::new();

            if answer.starts_with("~") {
                // Assume ~/ was at the start of the path and expand ~ to home_dir()
                path.push(dirs::home_dir().unwrap());
                path.push(&answer[2..answer.len()]);
            } else {
                path.push(answer)
            }


            Ok(Path{value: String::from(answer), path_value: path})
        }
        Err (_) => {
            Err(PathParsingError{})
        }
    }
}

struct Path {
    value: String,
    path_value: std::path::PathBuf,
}

#[derive(Debug)]
struct PathParsingError {

}


fn ask_comment() -> Result<Comment, CommentParsingError> {

    let question = Question::input("comment")
        .message("Add comment")
        .default("info@pietrobongiovanni.com")
        .build();

    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let answer = result.as_string().unwrap();
            Ok(Comment{text: String::from(answer)})
        }
        Err (_) => {
            Err(CommentParsingError{})
        }
    }

}

struct Comment {
    text: String
}

#[derive(Debug)]
struct CommentParsingError {

}


fn ask_encryption_type() -> Result<EncryptionType, EncryptionTypeParsingError> {
    let question = Question::select("encryption_type")
    .message("Select SSH Key encryption type")
    .choices(vec![
        format!("{}", EncryptionType::Ed25519), format!("{}", EncryptionType::Rsa)
    ])
    .build();
    let answer = requestty::prompt_one(question);

    match answer {
        Ok(result) => {
            let selected_type = EncryptionType::from_str(&result.as_list_item().unwrap().text);
            match selected_type {
                Ok(selected) => {
                    Ok(selected)
                }
                Err (_) => {
                    Err(EncryptionTypeParsingError{})
                }
            }
        }
        Err (_) => {
            Err(EncryptionTypeParsingError{})
        }
    }
}

#[derive(Debug)]
struct EncryptionTypeParsingError {

}

#[derive(Debug, PartialEq)]
enum EncryptionType {
    Rsa,
    Ed25519,
}

impl Display for EncryptionType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            EncryptionType::Rsa=> {
                write!(f, "rsa")
            }
            EncryptionType::Ed25519 => {
                write!(f, "ed25519")
            }
        }
    }
}

impl FromStr for EncryptionType {
    type Err = EncryptionTypeParsingError;
    fn from_str(input: &str) -> Result<EncryptionType, Self::Err> {
        match input {
            "ed25519"  => Ok(EncryptionType::Ed25519),
            "rsa"  => Ok(EncryptionType::Rsa),
            _      => Err(EncryptionTypeParsingError{}),
        }
    }
}