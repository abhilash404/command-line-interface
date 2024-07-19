mod cli;
mod commands;
mod db;
mod error;
mod models;

use clap::Parser;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    commands::execute_command(cli.command)?;
    Ok(())
}