use crate::errors::CliError;
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use colored::*;

const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB
const SEARCH_TIMEOUT: Duration = Duration::from_secs(30);

pub fn handle_command(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(run_matches) = matches.subcommand_matches("run") {
        match run_matches.subcommand() {
            Some(("index", index_matches)) => {
                if let Some(file_path) = index_matches.get_one::<String>("file") {
                    handle_index_file(file_path)
                        .with_context(|| format!("Failed to index file: {}", file_path))?;
                } else if let Some(dir_path) = index_matches.get_one::<String>("dir") {
                    handle_index_directory(dir_path)
                        .with_context(|| format!("Failed to index directory: {}", dir_path))?;
                }
            }
            Some(("search", search_matches)) => {
                let query = search_matches
                    .get_one::<String>("query")
                    .context("Search query is required")?;

                validate_search_query(query)?;

                if let Some(file_path) = search_matches.get_one::<String>("file") {
                    handle_search_file(query, file_path)
                        .with_context(|| format!("Failed to search in file: {}", file_path))?;
                } else if let Some(dir_path) = search_matches.get_one::<String>("dir") {
                    handle_search_directory(query, dir_path)
                        .with_context(|| format!("Failed to search in directory: {}", dir_path))?;
                }
            }
            _ => return Err(anyhow::anyhow!("Invalid command")),
        }
    }
    Ok(())
}

fn validate_search_query(query: &str) -> Result<()> {
    if query.trim().is_empty() {
        return Err(CliError::EmptyQuery.into());
    }

    // Check for valid regex pattern if the query is meant to be a regex
    if query.starts_with('/') && query.ends_with('/') {
        let pattern = &query[1..query.len() - 1];
        regex::Regex::new(pattern)
            .map_err(|_| CliError::InvalidPattern(pattern.to_string()))?;
    }

    Ok(())
}

fn validate_file_path(file_path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(file_path);
    
    if !path.exists() {
        return Err(CliError::PathNotFound(path).into());
    }
    if !path.is_file() {
        return Err(CliError::NotAFile(path).into());
    }

    // Check file size
    let metadata = fs::metadata(&path)
        .with_context(|| format!("Failed to read metadata for file: {}", file_path))?;
    
    if metadata.len() > MAX_FILE_SIZE {
        return Err(CliError::FileTooLarge {
            path,
            size: metadata.len(),
            max_size: MAX_FILE_SIZE,
        }
        .into());
    }

    // Check permissions
    let _file = fs::File::open(&path)
        .with_context(|| format!("Failed to open file: {}", file_path))?;
    
    if metadata.permissions().readonly() {
        return Err(CliError::PermissionDenied(path).into());
    }

    Ok(path)
}

fn validate_directory_path(dir_path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(dir_path);
    
    if !path.exists() {
        return Err(CliError::PathNotFound(path).into());
    }
    if !path.is_dir() {
        return Err(CliError::NotADirectory(path).into());
    }

    // Check permissions
    let dir = fs::read_dir(&path)
        .with_context(|| format!("Failed to read directory: {}", dir_path))?;

    Ok(path)
}

fn handle_index_file(file_path: &str) -> Result<()> {
    let path = validate_file_path(file_path)?;
    // println!("Indexing file: {}", path.display());
    println!("{} {}", "Indexing file:".green().bold(), path.display());
    
    // Read and validate file content
    let _content = fs::read_to_string(&path)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::InvalidData => CliError::InvalidUtf8(path.clone()),
            _ => CliError::Io(e),
        })?;

    // Add your file indexing logic here
    Ok(())
}

fn handle_index_directory(dir_path: &str) -> Result<()> {
    let path = validate_directory_path(dir_path)?;
    // println!("Indexing directory: {}", path.display());
    println!("{} {}", "Indexing directory:".green().bold(), path.display());

    // Recursively process directory with progress tracking
    let mut processed_files = 0;
    let mut failed_files = 0;

    for entry in walkdir::WalkDir::new(&path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        match handle_index_file(entry.path().to_str().unwrap()) {
            Ok(_) => processed_files += 1,
            Err(e) => {
                failed_files += 1;
                eprintln!("Warning: Failed to process {}: {}", entry.path().display(), e);
            }
        }
    }

    // println!(
    //     "Indexing completed: {} files processed, {} files failed",
    //     processed_files, failed_files
    // );
    println!(
        "{}\n{} {}\n{} {}",
        "Indexing completed:".green().bold(),
        "Files processed:".bright_white(), processed_files,
        "Files failed:".bright_white(), failed_files.to_string().red()
    );


    Ok(())
}

fn handle_search_file(query: &str, file_path: &str) -> Result<()> {
    let path = validate_file_path(file_path)?;
    // println!("Searching for '{}' in file: {}", query, path.display());
    println!("{} '{}' {}: {}", 
        "Searching for".green().bold(),
        query.yellow(),
        "in file".green().bold(),
        path.display()
    );


    let start_time = Instant::now();
    
    // Read file content with timeout
    let content = fs::read_to_string(&path)
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::InvalidData => CliError::InvalidUtf8(path.clone()),
            _ => CliError::Io(e),
        })?;

    // Perform search with timeout check
    let mut matches_found = 0;
    for (line_num, line) in content.lines().enumerate() {
        if start_time.elapsed() > SEARCH_TIMEOUT {
            return Err(CliError::SearchTimeout(SEARCH_TIMEOUT.as_secs()).into());
        }

        if line.contains(query) {
            println!("{}:{}: {}", path.display(), line_num + 1, line);
            matches_found += 1;
        }
    }

    println!("Found {} matches in {}", matches_found, path.display());
    Ok(())
}

fn handle_search_directory(query: &str, dir_path: &str) -> Result<()> {
    let path = validate_directory_path(dir_path)?;
    // println!("Searching for '{}' in directory: {}", query, path.display());
    println!("Searching for: {} {} in directory", query.green().bold(), path.display());

    let start_time = Instant::now();
    let mut total_matches = 0;
    let mut processed_files = 0;
    let mut failed_files = 0;

    for entry in walkdir::WalkDir::new(&path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if start_time.elapsed() > SEARCH_TIMEOUT {
            return Err(CliError::SearchTimeout(SEARCH_TIMEOUT.as_secs()).into());
        }

        match handle_search_file(query, entry.path().to_str().unwrap()) {
            Ok(_) => processed_files += 1,
            Err(e) => {
                failed_files += 1;
                eprintln!("Warning: Failed to search {}: {}", entry.path().display(), e);
            }
        }
    }

    println!(
        "Search completed: {} matches found in {} files ({} failed)",
        total_matches, processed_files, failed_files
    );

    Ok(())
}