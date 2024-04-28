use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
#[command(version = "0.1.0", about = "Simple cat like command", long_about = None)]
pub struct Cli {
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
