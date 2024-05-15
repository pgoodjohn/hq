use std::process::{Command, Stdio};
use std::net::TcpListener;

use super::types::*;

pub fn connect(port: u16, instance: &String,  zone: Zone, project: &String) -> Result<(), String> {

    validate_port(port).unwrap();

    let args = [
        "compute",
        "start-iap-tunnel",
        &format!("{}", instance.trim_end()),
        "3306",
        &format!("--local-host-port=localhost:{}", port),
        &format!("--project={}", project),
        &format!("--zone={}", zone.as_str()),
    ];

    log::debug!("Running: gcloud {}", args.join(" "));

    let mut command = Command::new("gcloud");
    for arg in args.iter() {
        command.arg(arg);
    }

    // Spawn the command
    let command = match command.spawn() {
        Ok(process) => process,
        Err(e) => return Err(format!("Failed to spawn gcloud command: {}", e)),
    };

    // Wait for the command to complete and capture the output
    let output = match command.wait_with_output() {
        Ok(output) => output,
        Err(e) => return Err(format!("Failed to read output: {}", e)),
    };

    if !output.status.success() {
        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Command failed:\n{}", err_msg));
    }

    let _output_str = String::from_utf8(output.stdout).unwrap_or_else(|_| String::new());
    // Ok(output_str);

    Ok(())
}


fn validate_port(port: u16) -> Result<(), String> {
    if is_port_in_use(port) {
        let process_details = get_process_details(port);
        match process_details {
            Some(details) => {
                log::warn!("Port {} is already in use by another process, run with -d flag to see more what process is taking it up.", port);
                log::debug!("Details:\n{}", details);
            }
            None => {
                log::warn!("Port {} is already in use by another process", port);
            }
        }
        return Err("Port is already in use".to_string());
    }

    Ok(())

}

fn is_port_in_use(port: u16) -> bool {
    log::debug!("Checking if port {} is in use", port);
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(_listener) => false, // If we can bind, the port is not in use.
        Err(_) => true, // If we can't bind, the port is in use.
    }
}

fn get_process_details(port: u16) -> Option<String> {
    #[cfg(target_os = "linux")]
    {
        // Using netstat to find the process details on Linux
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("sudo netstat -tuln | grep :{}", port))
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            Some(str::from_utf8(&output.stdout).unwrap().to_string())
        } else {
            None
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Using lsof to find the process details on macOS
        let output = Command::new("sh")
            .arg("-c")
            .arg(format!("lsof -i :{}", port))
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            Some(std::str::from_utf8(&output.stdout).unwrap().to_string())
        } else {
            None
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        None // Unsupported platform
    }
}


pub fn list(application: Application, zone: &Zone, project: &String) -> Result<Vec<String>, String> {
    let args = [
        "compute",
        "instances",
        "list",
        &format!("--filter=name~db-vm-{}", application.as_str()),
        // "--limit=1",
        &format!("--zones={}", zone.as_str()),
        &format!("--project={}", project),
        "--format=value(name)",
    ];

    log::debug!("Running: gcloud {}", args.join(" "));

    let mut command = Command::new("gcloud");
    for arg in args.iter() {
        command.arg(arg);
    }

    // Keep all the otuput in memory
    command.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped());

    // Spawn the command
    let spawned_command = match command.spawn() {
        Ok(process) => process,
        Err(e) => return Err(format!("Failed to spawn gcloud command: {}", e)),
    };

    // Capture output
    let output = match spawned_command.wait_with_output() {
        Ok(output) => {
            log::debug!("Output: {:?}", output);
            output
        },
        Err(e) => return Err(format!("Failed to read output: {}", e)),
    };

     if output.status.success() == false {
        log::debug!("{:?}", output);

        let err_msg = String::from_utf8_lossy(&output.stderr);
        return Err(format!("gcloud command failed:\n{}", err_msg));
    }

    // Successful output
    let output_str = split_multiline_to_vector(&String::from_utf8(output.stdout).unwrap_or_else(|_| String::new()));

    return Ok(output_str);
}

fn split_multiline_to_vector(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_string()).collect()
}
