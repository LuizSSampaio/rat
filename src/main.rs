mod cli;
mod file;

use clap::Parser;

use crate::{cli::*, file::File};

fn main() {
    let cli = Cli::parse();
    let file = match File::new(cli.get_path()) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    file.print_file();
}
