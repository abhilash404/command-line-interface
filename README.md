# Rust Command-Line To-Do List Manager

This project is a command-line to-do list manager written in Rust. It allows you to create, manage, and update to-do lists directly from the terminal. The project is modular, making use of several Rust libraries to handle different aspects of functionality.

## Features

- Create and manage multiple to-do lists.
- Add, complete, mark as working, mark as incomplete, and remove tasks from lists.
- Display tasks with filtering options (all, completed, incomplete).

## Modules

- `cli`: Handles command-line argument parsing.
- `commands`: Executes commands based on user input.
- `db`: Manages data storage and retrieval.
- `error`: Defines custom error types for the application.
- `models`: Defines data structures for the to-do list.

## Getting Started

### Prerequisites

- Rust (https://www.rust-lang.org/tools/install)

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/yourusername/rust-todo-cli.git
    cd rust-todo-cli
    ```

2. Build the project:
    ```sh
    cargo build
    ```

### Usage

Run the application using `cargo run` followed by a command:

```sh
cargo run -- [COMMAND]
Commands
Show tasks:

sh
cargo run -- show [OPTIONS]
Options:
--all: Show all tasks.
--completed: Show only completed tasks.
--incomplete: Show only incomplete tasks.
--list-name <LIST_NAME>: Show tasks from a specific list.
Add a task:

sh
cargo run -- add --list-name <LIST_NAME> --item <ITEM_DESCRIPTION>
Complete a task:

sh
cargo run -- complete --list-name <LIST_NAME> --item-number <ITEM_NUMBER>
Mark a task as working:

sh
cargo run -- working --list-name <LIST_NAME> --item-number <ITEM_NUMBER>
Mark a task as incomplete:

sh
cargo run -- incomplete --list-name <LIST_NAME> --item-number <ITEM_NUMBER>
Remove a task:

sh
cargo run -- remove --list-name <LIST_NAME> --item-number <ITEM_NUMBER>
Example
Adding a new task:

sh
cargo run -- add --list-name "Work" --item "Finish report"
Completing a task:

sh
cargo run -- complete --list-name "Work" --item-number 1
Showing all tasks in a list:

sh
cargo run -- show --list-name "Work"
Project Structure
main.rs: The entry point of the application.
cli.rs: Defines the command-line interface structure and subcommands.
commands.rs: Contains functions to execute various commands based on user input.
db.rs: Manages data storage and retrieval.
error.rs: Defines custom error types for the application.
models.rs: Defines data structures used in the application.
Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

License
This project is licensed under the MIT License. See the LICENSE file for more details.

Acknowledgements
clap - Command Line Argument Parser for Rust.
serde - Serialization framework.
serde_json - JSON support for Serde.
thiserror - Simplified error handling.


Made with ❤️ by Your Name
Replace `yourusername` and `[Your Name]` with your GitHub username and your name, respective
