use std::path::PathBuf;

use clap::Parser;

use crate::file::*;

#[derive(Parser)]
#[command(version = "0.1.0", about = "Simple cat like command", long_about = None)]
pub struct Cli {
    file: Option<PathBuf>,
}

impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        let file_path = match cli.file {
            Some(path) => path,
            None => PathBuf::from("./"),
        };
        let file = match File::new(file_path) {
            Ok(result) => result,
            Err(e) => {
                println!("ERROR: {}", e);
                return;
            }
        };
        file.print_file();
    }
}
