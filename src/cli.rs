use std::path::{Path, PathBuf};

use clap::Parser;

#[derive(Parser)]
#[command(version = "0.0.1", about = "Simple cat like command", long_about = None)]
pub struct Cli {
    file: Option<PathBuf>
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        println!("{}", cli.file_is_valid());
    }

    fn file_is_valid(&self) -> bool {
        match &self.file {
            Some(file_path) => {
                Path::new(file_path.to_str().unwrap()).is_file()
            },
            None => false
        }
    }
}