// src/error.rs
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Path does not exist: {0}")]
    PathNotFound(PathBuf),

    #[error("Not a file: {0}")]
    NotAFile(PathBuf),

    #[error("Not a directory: {0}")]
    NotADirectory(PathBuf),

    #[error("Permission denied: {0}")]
    PermissionDenied(PathBuf),

    #[error("Invalid UTF-8 in file: {0}")]
    InvalidUtf8(PathBuf),

    #[error("File too large: {path} ({size} bytes)")]
    FileTooLarge {
        path: PathBuf,
        size: u64,
        max_size: u64,
    },

    #[error("Empty query string")]
    EmptyQuery,

    #[error("Invalid query pattern: {0}")]
    InvalidPattern(String),

    #[error("Search timeout after {0} seconds")]
    SearchTimeout(u64),
}