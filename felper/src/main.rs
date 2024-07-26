//! # felper
//!
//! ## Purpose
//! creating usefool tools for my flutter development work flow.
//!
//! ## Features
//! - felper modular {file_name}
//!
//! ## Usage
//! - felper help
//! - felper command /sub-commands
//! ## Dependencies
//! - clap
//!     - amazing command help
//! - prettier
//!     - colorful commands :)
//!
//! ## Build
//!
//! > installs felper, and runs it in the terminal
//!
//! ``` bash
//! cargo install --path . && felper
//! ```
//!
//!
//! ## Notes
//! Wanting to create a simple enough system for speeding up development time.
use clap::{Arg, Command};
use std::fs;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let matches = Command::new("felper")
        .version("1.0")
        .author("Jonas Smith")
        .about("Creating useful tools for Flutter development workflow")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("modular").about("Create a modular file").arg(
                Arg::new("file_name")
                    .help("The name of the file to create")
                    .required(true),
            ),
        )
        .subcommand(Command::new("list").about("List available commands"))
        .get_matches();

    match matches.subcommand() {
        Some(("modular", sub_matches)) => {
            let file_name = sub_matches
                .get_one::<String>("file_name")
                .expect("required");
            println!("Creating modular file: {}", file_name);

            // Create main directory
            if let Err(e) = fs::create_dir(file_name) {
                eprintln!("Error creating main directory: {}", e);
                return;
            }

            // Create bloc folder
            let bloc_path = Path::new(file_name).join("bloc");
            if let Err(e) = fs::create_dir(&bloc_path) {
                eprintln!("Error creating bloc directory: {}", e);
                return;
            }

            // Create and populate bloc.dart file
            let bloc_file_path = bloc_path.join("bloc.dart");
            match fs::File::create_new(&bloc_file_path) {
                 Ok(mut file) => {
                    if let Err(e) = file.write_all("/// export \"your_widget.dart\";".as_bytes()) {
                        eprintln!("Error writing to bloc.dart: {}", e);
                    }
                },
                Err(e) => eprintln!("Error creating bloc.dart: {}", e),
            }

            // Create widgets folder
            let widgets_path = Path::new(file_name).join("widgets");
            if let Err(e) = fs::create_dir(&widgets_path) {
                eprintln!("Error creating widgets directory: {}", e);
                return;
            }

            // Create widgets.dart file
            let widgets_file_path = widgets_path.join("widgets.dart");
            match fs::File::create_new(&widgets_file_path) {

                 Ok(mut file) => {
                    if let Err(e) = file.write_all("/// export \"your_widget.dart\";".as_bytes()) {
                        eprintln!("Error writing to bloc.dart: {}", e);
                    }
                },
                Err(e) => eprintln!("Error creating bloc.dart: {}", e),
            }

            println!("Modular structure created successfully!");
        }
        Some(("list", _)) => {
            println!("Available commands:");
            println!("  modular <file_name>    Create a modular file");
            println!("  list                   List available commands");
            println!("  help                   Print help information");
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}
