use std::path::PathBuf;
use clap::{Subcommand, Parser};

#[derive(Parser)]
#[clap(about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// HLS Downloader
    HLS(HLSCommand)
}

#[derive(Parser)]
#[clap(arg_required_else_help(true))]
pub struct HLSCommand {
    /// Input m3u8 text file
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Header file
    #[clap(short='H', long, value_parser, value_name = "FILE")]
    pub headers: Option<PathBuf>,

    /// Enable png mode
    #[clap(short, long, value_parser, default_value_t = false)]
    pub png: bool,

    /// keep temp folder after download
    #[clap(short, long, value_parser, default_value_t = false)]
    pub keep: bool,

    /// Output file name
    #[clap(short, long, value_parser)]
    pub output: Option<String>,
}
