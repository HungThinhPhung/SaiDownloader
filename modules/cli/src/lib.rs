mod command;

use std::path::PathBuf;
use crate::command::{Cli, Commands, HLSCommand};
use clap::Parser;
use saidl_hls::download;
use saidl_helper::{file::text_to_lines, http::{lines_to_header, HeaderMap}};

pub fn run() {
    let cli: Cli = Cli::parse();
    let command = cli.command.expect("None is already redirected to help");
    match command {
        Commands::HLS(hls) => {
            handle_hls(hls);
        }
    }
}

pub fn handle_hls(hls: HLSCommand) {
    match hls.input {
        None => {
            println!("Input file is required");
        }
        Some(path) => {
            // Extract links from input
            let mut content_lines = text_to_lines(path);
            let links = link_filter(&mut content_lines);

            // Extract headers from header file
            let headers = extract_header(hls.headers);

            download(links, hls.png, hls.keep, &headers, hls.output);
        }
    }
}

fn link_filter(links: &mut Vec<String>) -> &mut Vec<String> {
    links.retain(|i| i.starts_with("http"));
    return links;
}

fn extract_header(path: Option<PathBuf>) -> Option<HeaderMap> {
    match path {
        None => None,
        Some(p) => {
            let header_lines = text_to_lines(p);
            Some(lines_to_header(&header_lines))
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
