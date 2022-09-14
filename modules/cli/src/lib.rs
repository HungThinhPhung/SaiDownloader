mod command;

use std::{env, path};
use crate::command::{Cli, Commands, HLSCommand};
use clap::Parser;

pub fn run() {
    let cli: Cli = Cli::parse();
    let command = cli.command.expect("None is already redirected to help");
    match command {
        Commands::HLS(hls) => {
            handle_hls(hls);
        }
    }
    // let x = cli
    // let args: Vec<String> = env::args().collect();
    // let file_path = &args[2];
    // let contents = std::fs::read_to_string(file_path).unwrap();
    // let content_lines = contents.split("\n");
    // let vec: Vec<&str> = content_lines.collect();
    // println!("With text:\n{contents}");
}

pub fn handle_hls(hls: HLSCommand) {
    match hls.input {
        None => {
            println!("Input file is required");
        }
        Some(path) => {
            let contents = std::fs::read_to_string(path).unwrap();
            let content_lines = contents.split("\n");
            let vec: Vec<&str> = content_lines.collect();
            println!("With text:\n{contents}");
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
