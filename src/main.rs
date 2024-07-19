mod cli;
mod commands;
mod db;
mod error;
mod models;

use clap::Parser;
use cli::Cli;
use commands::execute_command;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = execute_command(cli.command) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}