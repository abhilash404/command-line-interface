use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Show {
        #[clap(long)]
        all: bool,
        #[clap(long)]
        completed: bool,
        #[clap(long)]
        incomplete: bool,
        list_name: Option<String>,
    },
    Add {
        list_name: String,
        item: String,
    },
    Complete {
        list_name: String,
        item_number: usize,
    },
    Incomplete {
        list_name: String,
        item_number: usize,
    },
    Remove {
        list_name: Option<String>,
        item_number: Option<usize>,
    },
}