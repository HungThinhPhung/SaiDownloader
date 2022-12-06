mod command;

use std::path::PathBuf;
use crate::command::{Cli, Commands, EBCommand, HLSCommand};
use clap::Parser;
use saidl_hls::download;
use saidl_ebook::{Config, IterationConfig, TocConfig, EbookFlow, TocDownloader, StandardEpub, WriteBook};
use saidl_helper::{file::get_lines, http::{lines_to_header, HeaderMap}};

pub fn run() {
    let cli: Cli = Cli::parse();
    let command = cli.command.expect("Invalid commands is already handled by clap");
    match command {
        Commands::HLS(hls) => {
            handle_hls(hls);
        }
        Commands::EB(eb) => {
            handle_eb(eb);
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
            let content_lines = get_lines(path);
            let links = link_filter(content_lines.into_iter());

            // Extract headers from header file
            let headers = extract_header(hls.headers);

            download(&links, hls.png, hls.keep, &headers, hls.output);
        }
    }
}

pub fn handle_eb(eb: EBCommand) {
    match eb.input {
        None => { println!("Input file is required"); }
        Some(path) => {
            let contents = std::fs::read_to_string(path).unwrap();
            let config: Config = toml::from_str(&contents).unwrap();
            let y = config.flow;
            match y {
                EbookFlow::Iter(e) => {
                    let t = 1;
                    let y = e.iter;
                    let z = 1;
                }
                EbookFlow::Toc(t) => {
                    let downloader = TocDownloader::build(t, config.title_selector, config.content_selector);
                    let content = downloader.download();
                    let writer = StandardEpub::build(config.name, content);
                    writer.write().unwrap();
                }
            }
            let x = 1;
        }
    }
}

fn link_filter(links: impl Iterator<Item=String>) -> Vec<String> {
    links.filter(|i| i.starts_with("http")).collect()
}

fn extract_header(path: Option<PathBuf>) -> Option<HeaderMap> {
    match path {
        None => None,
        Some(p) => {
            let header_lines = get_lines(p);
            Some(lines_to_header(header_lines.into_iter()))
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
