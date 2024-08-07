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
use colored::Colorize;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command as comp_Command;
extern crate dirs;
mod error;
mod prelude;
use crate::error::Error;
use crate::prelude::*;

fn main() -> Result<()> {
    let matches = Command::new("felper")
        .version("1.0")
        .author("Jonas Smith")
        .about("Creating useful tools for Flutter development workflow")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("modular")
                .about("Create a modular file")
                .arg(
                    Arg::new("file_name")
                        .help("The name of the file to create")
                        .required(true),
                )
                .arg(
                    Arg::new("bloc")
                        .long("bloc")
                        .help("Include bloc files")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("widgets")
                        .long("widgets")
                        .help("Include widgets files")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("run")
                        .long("run")
                        .help("Run build_runner after creating files")
                        .action(clap::ArgAction::SetTrue),
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

            let include_bloc_tag = sub_matches.get_flag("bloc");
            let include_widgets_tag = sub_matches.get_flag("widgets");
            let run_build_runner_tag = sub_matches.get_flag("run");

            // Create main directory
            let main_path = Path::new(file_name);
            if let Err(e) = fs::create_dir_all(main_path) {
                eprintln!("Error creating main directory: {}", e);
                return Err(Error::IO(e));
            }

            if include_bloc_tag {
                if let Err(e) = create_bloc_files(main_path, file_name) {
                    eprintln!("Error creating bloc files: {}", e);
                    return Err(e);
                }
            }

            if include_widgets_tag {
                if let Err(e) = create_widgets_files(main_path, file_name) {
                    eprintln!("Error creating widgets files: {}", e);
                    return Err(e);
                }
            }

            // Create the custom page file
            let page_file_path = main_path.join(format!("{}_page.dart", file_name));
            let page_content = generate_page_content(file_name);
            if let Err(e) = create_file_if_not_exists(&page_file_path, &page_content) {
                eprintln!("Error creating {}_page.dart: {}", file_name, e);
                return Err(Error::IO(e));
            }

            // create the module file
            let module_file_path = main_path.join(format!("{}_module.dart", file_name));
            let module_content = generate_module_content(file_name);
            if let Err(e) = create_file_if_not_exists(&module_file_path, &module_content) {
                eprintln!("Error creating {}_module.dart: {}", file_name, e);
                return Err(Error::IO(e));
            }

            // Create and populate {file_name}.dart file
            let export_file_path = main_path.join(format!("{}.dart", file_name));
            // Option 1: Using a vector of strings
            // Option 2: Using a multi-line string literal
            let export_content = format!(
                r#"export 'bloc/{0}_bloc.dart';
                export 'widgets/widgets.dart';
                export '{0}_page.dart';
                export '{0}_module.dart';
                // Add more export lines as needed
                "#,
                file_name
            );
            if let Err(e) = create_file_if_not_exists(&export_file_path, &export_content) {
                eprintln!("Error creating {}.dart: {}", file_name, e);
                return Err(Error::IO(e));
            }

            println!("Modular structure created successfully!");

            // Check and augment parent file
            if let Err(e) = check_and_augment_parent_file(file_name) {
                eprintln!("Error checking/augmenting parent file: {}", e);
            }

            // Run build_runner
            if let Err(e) = run_build_runner(&main_path) {
                return Err(Error::BuildRunnerError(e.to_string()));
            }

            // Run build_runner if the flag is set
            if run_build_runner_tag {
                if let Err(e) = run_build_runner(&main_path) {
                    return Err(Error::BuildRunnerError(e.to_string()));
                }
                println!("Build runner completed successfully!");
            }

            println!("Modular structure and build process completed successfully!");
        }
        Some(("list", _)) => {
            print_available_commands();
        }
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }

    Ok(())
}

fn run_mason_command(dir: &Path, args: &[&str]) -> Result<()> {
    let output = comp_Command::new("mason")
        .current_dir(dir)
        .args(args)
        .output()?;

    if output.status.success() {
        println!(
            "Mason command succeeded: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        Ok(())
    } else {
        Err(Error::MasonError(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

fn create_file_if_not_exists(path: &Path, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path);

    match file {
        Ok(ref mut f) => {
            let file_path = format!("{:?}", path);
            f.write_all(content.as_bytes())?;
            println!("file : {} created and written to", file_path.green());
        }
        Err(ref e) if e.kind() == io::ErrorKind::AlreadyExists => {
            println!("File already exists: {:?}", path);
            // If you want to update existing file, uncomment the following lines:
            // file = OpenOptions::new().write(true).open(path);
            // file?.write_all(content.as_bytes())?;
            // println!("Existing file updated with new content.");
        }
        Err(e) => {
            eprintln!("Error creating file: {:?}", e);
            return Err(e);
        }
    }

    Ok(())
}

fn generate_page_content(file_name: &str) -> String {
    let page_name = format!("{}Page", to_pascal_case(file_name));
    format!(
        r#"import 'package:flutter/material.dart';
import 'package:flutter_modular/flutter_modular.dart';

/// [{0}] the display page for this feature
class {0} extends StatelessWidget {{
  /// [{0}] constructor.
  const {0}({{super.key}});

  /// [routeName] the route name for this page
  static const routeName = '/{1}';

  /// our route, this should generally use the modular route, and
  /// our basic route callable item
  static void route() {{
    Modular.to.pushNamed(routeName);
  }}

  @override
  Widget build(BuildContext context) {{
    return const Scaffold(
      body: Text(
        routeName,
      ),
    );
  }}
}}"#,
        page_name, file_name
    )
}

fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect()
}

fn generate_module_content(file_name: &str) -> String {
    let file_path = format!("{}", to_pascal_case(file_name));
    let page_name = format!("{}Page", to_pascal_case(file_name));
    let module_name = format!("{}Module", to_pascal_case(file_name));

    format!(
        r#"import 'package:flutter_modular/flutter_modular.dart';
import '{1}.dart';

/// [{3}] is a [Module] that provides the application's dependencies.
class {3} extends Module {{
  @override
  void binds(Injector i) {{
    i.addLazySingleton<{2}Bloc>(() => {2}Bloc()..add(const {2}Event.started()));
  }}

  @override
  void routes(RouteManager r) {{
    r.child(
      '/',
      child: (context) => const {0}(),
    );
  }}
}}"#,
        page_name, file_name, file_path, module_name,
    )
}

fn run_build_runner(main_path: &Path) -> io::Result<()> {
    println!("Running build_runner...");

    let output = comp_Command::new("flutter")
        .current_dir(main_path)
        .args(&[
            "pub",
            "run",
            "build_runner",
            "build",
            "--delete-conflicting-outputs",
        ])
        .output()?;

    if output.status.success() {
        println!("{}", "build_runner completed successfully.".green());
        println!("Output: {}", String::from_utf8_lossy(&output.stdout));
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error running build_runner: {}", error);
        Err(io::Error::new(io::ErrorKind::Other, "build_runner failed"))
    }
}

fn check_and_augment_parent_file(inserted_file_name: &str) -> io::Result<()> {
    let current_dir = std::env::current_dir()?;

    let last_path = get_last_path(&current_dir);

    let target_file = format!("{}.dart", last_path);
    let paths = fs::read_dir(&current_dir)?;

    for path_result in paths {
        match path_result {
            Ok(entry) => {
                let file_path = entry.path();
                let file_name = get_last_path(&file_path);

                if file_name == target_file {
                    add_export_to_file(&file_path, &inserted_file_name)?;
                }
            }
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }

    Ok(())
}

fn get_last_path(dir: &PathBuf) -> String {
    let path_string = dir.to_string_lossy().into_owned();
    let components: Vec<_> = path_string.split('/').collect();

    if let Some(last_component) = components.last() {
        return last_component.to_string();
    }

    "".to_string()
}

fn add_export_to_file(file_path: &Path, file_name: &str) -> std::io::Result<()> {
    let mut content = String::new();
    let mut file = File::open(file_path)?;
    file.read_to_string(&mut content)?; // Changed from read_to_end to read_to_string
    let export_statement = format!("export '{0}/{0}.dart';\n", file_name);
    if !content.starts_with(&export_statement) {
        let new_content = format!("{}{}", export_statement, content);
        let mut file = File::create(file_path)?;
        file.write_all(new_content.as_bytes())?;
        println!("Added export statement to {}", file_name);
    } else {
        println!("Export statement already exists in {}", file_name);
    }
    Ok(())
}

fn create_bloc_files(main_path: &Path, file_name: &str) -> Result<()> {
    let bloc_path = main_path.join("bloc");
    fs::create_dir(&bloc_path)?;

    // Run Mason commands for bloc
    run_mason_command(&bloc_path, &["init"])?;
    run_mason_command(&bloc_path, &["add", "bloc"])?;
    run_mason_command(
        &bloc_path,
        &["make", "bloc", "--name", file_name, "--style", "freezed"],
    )?;

    println!("Bloc files created successfully");
    Ok(())
}

fn create_widgets_files(main_path: &Path, _: &str) -> Result<()> {
    let widgets_path = main_path.join("widgets");
    fs::create_dir(&widgets_path)?;

    let widgets_file_path = widgets_path.join("widgets.dart");
    create_file_if_not_exists(&widgets_file_path, "/// export \"your_widget.dart\";")?;

    println!("Widgets files created successfully");
    Ok(())
}

fn print_available_commands() {
    println!("Available commands:");
    println!("  modular <file_name> [--bloc] [--widgets] [--run]    Create a modular file");
    println!("    --bloc      Include bloc files");
    println!("    --widgets   Include widgets files");
    println!("    --run       Run build_runner after creating files");
    println!("  list                                                List available commands");
    println!("  help                                                Print help information");
}

#[cfg(test)]
mod tests {
    use assert_cmd::Command;
    use predicates::prelude::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_help_command() {
        let mut cmd = Command::cargo_bin("felper").unwrap();
        cmd.arg("--help");
        cmd.assert().success().stdout(predicate::str::contains(
            "Creating useful tools for Flutter development workflow",
        ));
    }

    #[test]
    fn test_list_command() {
        let mut cmd = Command::cargo_bin("felper").unwrap();
        cmd.arg("list");
        cmd.assert()
            .success()
            .stdout(predicate::str::contains("Available commands:"))
            .stdout(predicate::str::contains("modular <file_name>"))
            .stdout(predicate::str::contains("list"));
    }

    #[test]
    fn test_modular_command() {
        let temp_dir = tempdir().unwrap();
        let file_name = "test_feature";

        let mut cmd = Command::cargo_bin("felper").unwrap();
        cmd.arg("modular")
            .arg(file_name)
            .arg("--bloc")
            .arg("--widgets")
            .current_dir(&temp_dir);

        cmd.assert().success();

        // Check if directories and files were created
        assert!(temp_dir.path().join(file_name).exists());
        assert!(temp_dir.path().join(file_name).join("bloc").exists());
        assert!(temp_dir.path().join(file_name).join("widgets").exists());
        assert!(temp_dir
            .path()
            .join(file_name)
            .join(format!("{}_page.dart", file_name))
            .exists());
        assert!(temp_dir
            .path()
            .join(file_name)
            .join(format!("{}_module.dart", file_name))
            .exists());
        assert!(temp_dir
            .path()
            .join(file_name)
            .join(format!("{}.dart", file_name))
            .exists());

        // Check content of the main export file
        let export_content = fs::read_to_string(
            temp_dir
                .path()
                .join(file_name)
                .join(format!("{}.dart", file_name)),
        )
        .unwrap();
        assert!(export_content.contains(&format!("export 'bloc/{}_bloc.dart';", file_name)));
        assert!(export_content.contains("export 'widgets/widgets.dart';"));
        assert!(export_content.contains(&format!("export '{}_page.dart';", file_name)));
        assert!(export_content.contains(&format!("export '{}_module.dart';", file_name)));
    }

    #[test]
    fn test_modular_command_without_flags() {
        let temp_dir = tempdir().unwrap();
        let file_name = "test_feature_no_flags";

        let mut cmd = Command::cargo_bin("felper").unwrap();
        cmd.arg("modular").arg(file_name).current_dir(&temp_dir);

        cmd.assert().success();

        // Check if directories and files were created
        assert!(temp_dir.path().join(file_name).exists());
        assert!(!temp_dir.path().join(file_name).join("bloc").exists());
        assert!(!temp_dir.path().join(file_name).join("widgets").exists());
        assert!(temp_dir
            .path()
            .join(file_name)
            .join(format!("{}_page.dart", file_name))
            .exists());
        assert!(temp_dir
            .path()
            .join(file_name)
            .join(format!("{}_module.dart", file_name))
            .exists());
        assert!(temp_dir
            .path()
            .join(file_name)
            .join(format!("{}.dart", file_name))
            .exists());
    }
}
