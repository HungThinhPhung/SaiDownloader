mod command;

use crate::command::{Cli, Commands, EBCommand, HLSCommand};
use clap::Parser;
use saidl_ebook::{
    Config, EBConfig, EbookFlow, IterDownloader, NumDownloader, StandardEpub, TocDownloader,
    WriteBook,
};
use saidl_helper::{
    file::get_lines,
    http::{lines_to_header, HeaderMap},
};
use saidl_hls::download;
use std::path::PathBuf;

pub async fn run() {
    let cli: Cli = Cli::parse();
    let command = cli
        .command
        .expect("Invalid commands is already handled by clap");
    match command {
        Commands::HLS(hls) => {
            handle_hls(hls).await;
        }
        Commands::EB(eb) => {
            handle_eb(eb).await;
        }
    }
}

pub async fn handle_hls(hls: HLSCommand) {
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

            download(
                &links,
                hls.png,
                hls.h2,
                hls.multi_thread,
                hls.keep,
                &headers,
                hls.output,
                hls.delay,
                hls.retry,
            )
            .await;
        }
    }
}

pub async fn handle_eb(eb: EBCommand) {
    match eb.input {
        None => {
            println!("Input file is required");
        }
        Some(path) => {
            println!("{}", eb.h2);
            let contents = std::fs::read_to_string(path).unwrap();
            let config: Config = toml::from_str(&contents).unwrap();
            let Config {
                title_selector,
                content_selector,
                delay,
                retry,
                flow,
                name,
            } = config;
            let headers = extract_header(eb.headers);
            let cli_config = EBConfig {
                title_selector,
                content_selector,
                h2: eb.h2,
                headers: &headers,
                delay,
                retry,
            };
            let content = match flow {
                EbookFlow::Iter(f) => {
                    let downloader = IterDownloader::build(f);
                    downloader.download(cli_config).await
                }
                EbookFlow::Toc(f) => {
                    let downloader = TocDownloader::build(f);
                    downloader.download(cli_config).await
                }
                EbookFlow::Num(f) => {
                    let downloader = NumDownloader::build(f);
                    downloader.download(cli_config).await
                }
            };
            let writer = StandardEpub::build(name.clone(), content);
            writer.write(eb.chapter_num).unwrap();
        }
    }
}

fn link_filter(links: impl Iterator<Item = String>) -> Vec<String> {
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
