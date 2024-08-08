# Felper

Felper is a command-line tool designed to streamline Flutter development workflows by automating the creation of modular file structures and running common tasks.

## Features

- Create modular file structures for Flutter projects
- Generate BLoC files
- Create widget files
- Run build_runner automatically

## Installation

To install Felper, you need to have Rust and Cargo installed on your system. If you don't have them, you can install them from [rustup.rs](https://rustup.rs/).

Once you have Rust and Cargo installed, clone this repository and run:

```bash
cargo install --path .
```

This will install Felper on your system.

## Usage

Here are some example commands:

### Create a modular file structure

```bash
felper modular my_feature
```

### Create a modular file structure with BLoC files

```bash
felper modular my_feature --widgets
```

### List available commands

```bash
felper --help
```

## Dependencies

Felper requires the following tools to be installed and available in your PATH:

- Flutter
- Mason CLI

Make sure these are installed and properly configured before using Felper.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Jonas Smith
