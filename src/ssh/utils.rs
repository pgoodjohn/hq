use log::debug;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::vec::Vec;
extern crate dirs;

#[derive(Debug)]
pub struct SshKey {
    pub value: String,
}

pub fn list_private_keys() -> Result<Vec<SshKey>, io::Error> {
    let mut ssh_path = PathBuf::new();
    ssh_path.push(dirs::home_dir().unwrap());
    ssh_path.push(".ssh");

    debug!("Listing private keys in ~/.ssh");

    Ok(fs::read_dir(ssh_path)
        .unwrap()
        .into_iter()
        .filter(|r| r.is_ok()) // Get rid of Err variants for Result<DirEntry>
        .map(|r| {
            let path = r.unwrap().path();
            let path_string = String::from(path.to_str().unwrap());
            SshKey { value: path_string }
        })
        .filter(|r| r.value.contains("id_") && r.value.contains(".pub") == false)
        .collect())
}
