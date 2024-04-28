use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version = "0.1.0", about = "Simple cat like command", long_about = None)]
pub struct Cli {
    /// Number all output lines, starting with 1.
    #[arg(short, long)]
    pub number: bool,

    file: Option<PathBuf>,
}

impl Cli {
    pub fn get_path(&self) -> &str {
        match &self.file {
            Some(path) => path.to_str().unwrap_or("./"),
            None => "./",
        }
    }
}
