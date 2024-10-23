use std::path::PathBuf;
use clap::Parser;
use crate::verify_path;

#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short, long, value_parser = verify_path, default_value = ".")]
    pub dir: PathBuf,

    /// Path to the directory to serve
    #[arg(long, default_value = "8080")]
    pub port: u16,

}