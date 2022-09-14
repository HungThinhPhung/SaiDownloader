extern crate core;

pub mod file;

use std::fmt::Display;
use std::process::Command;

pub fn get_format_msg(base_msg: &str, format_obj: impl Display) -> String {
    return format!("\n{}\n{}", base_msg, format_obj);
}

pub fn run_os_command(command: &str) {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(["/C", command])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("failed to execute process")
    };
    println!("status: {}", output.status);
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
}