use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(about, long_about = None, arg_required_else_help(true))]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// HLS Downloader
    HLS(HLSCommand),

    /// Ebook downloader
    EB(EBCommand),
}

#[derive(Parser)]
#[clap(arg_required_else_help(true))]
pub struct EBCommand {
    /// Load input config file
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Enable http2 mode
    #[clap(long, value_parser, default_value_t = false)]
    pub h2: bool,

    /// Header file
    #[clap(short = 'H', long, value_parser, value_name = "FILE")]
    pub headers: Option<PathBuf>,

    /// Add chapter index number at title
    #[clap(long, short, value_parser, default_value_t = false)]
    pub chapter_num: bool,
}

#[derive(Parser)]
#[clap(arg_required_else_help(true))]
pub struct HLSCommand {
    /// Input m3u8 text file
    #[clap(short, long, value_parser, value_name = "FILE")]
    pub input: Option<PathBuf>,

    /// Header file
    #[clap(short = 'H', long, value_parser, value_name = "FILE")]
    pub headers: Option<PathBuf>,

    /// Enable png mode
    #[clap(short, long, value_parser, default_value_t = false)]
    pub png: bool,

    /// Keep temp folder after download
    #[clap(short, long, value_parser, default_value_t = false)]
    pub keep: bool,

    /// Output file name
    #[clap(short, long, value_parser)]
    pub output: Option<String>,

    /// Enable http2 mode
    #[clap(long, value_parser, default_value_t = false)]
    pub h2: bool,

    /// Enable multi thread mode
    #[clap(short, long, value_parser, default_value_t = false)]
    pub multi_thread: bool,

    #[clap(short, long, value_parser)]
    pub delay: Option<u64>,

    #[clap(short, long, value_parser)]
    pub retry: Option<u8>,
}
