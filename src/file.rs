use core::fmt;
use std::{error::Error, fs::read_to_string, path::PathBuf};

#[derive(Debug)]
pub struct File {
    file_path: String,
}

impl File {
    pub fn new(file_path: &str) -> Result<Self, FileError> {
        if !Self::is_valid(file_path) {
            return Err(FileError::FileNotFound);
        }

        Ok(Self {
            file_path: file_path.to_string(),
        })
    }

    fn is_valid(file_path: &str) -> bool {
        PathBuf::from(file_path).is_file()
    }

    pub fn print_file(&self, show_number: bool) {
        if show_number {
            let mut line_couter = 1;

            for line in read_to_string(&self.file_path).unwrap().lines() {
                let string_couter = line_couter.to_string();
                let line_number =
                    format!("{}{}", " ".repeat(6 - string_couter.len()), string_couter);

                println!("{}  {}", line_number, line);
                line_couter += 1;
            }
        } else {
            for line in read_to_string(&self.file_path).unwrap().lines() {
                println!("{}", line);
            }
        }
    }
}

#[derive(Debug)]
pub enum FileError {
    FileNotFound,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FileError::FileNotFound => write!(f, "File not found"),
        }
    }
}

impl Error for FileError {}
