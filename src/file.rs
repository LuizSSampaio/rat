use core::fmt;
use std::{
    error::Error,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct File {
    file_path: PathBuf,
}

impl File {
    pub fn new(file_path: PathBuf) -> Result<Self, FileError> {
        if !Self::is_valid(file_path.as_path()) {
            return Err(FileError::FileNotFound);
        }

        Ok(Self {
            file_path: file_path.to_path_buf(),
        })
    }

    fn is_valid(file_path: &Path) -> bool {
        file_path.is_file()
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
