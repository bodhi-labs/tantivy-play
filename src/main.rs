mod cli;
mod commands;
mod errors;

use anyhow::Result;
use colored::*;


fn main() -> Result<()> {
    // Enable colors for Windows
    #[cfg(windows)]
    colored::control::set_virtual_terminal(true).unwrap_or(());

    let matches = cli::build_cli().get_matches();

    if let Err(err) = commands::handle_command(&matches) {
        eprintln!("{} {}", "Error:".red().bold(), err);
        std::process::exit(1);
    }
    
    // commands::handle_command(&matches)?;
    
    Ok(())
}