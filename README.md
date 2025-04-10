# Terminal Password Generator

A simple and secure terminal-based password generator written in Rust. This tool allows you to generate strong, random passwords directly from your terminal, making it easy to create secure credentials for your applications and services.

## Features

- **Customizable Password Length:** Specify the desired length of your password.
- **Character Set Options:** Choose which character types to include (uppercase, lowercase, numbers, symbols).
- **Command-line Interface:** Simple flags for quick password generation.
- **Interactive Mode:** Step-by-step prompts for more detailed customization.
- **Random and Secure:** Generates cryptographically secure random passwords.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.40 or later) installed on your system.

## Installation

### Local Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/zacblev1/password.generator.git
cd password.generator
```

Build the project using Cargo:

```bash
cargo build --release
```

This will compile the project and produce an executable in the `target/release` directory.

### Global Installation

To make the password generator available from anywhere on your system, you have two options:

#### Option 1: Install using Cargo

```bash
cargo install --path .
```

This will install the executable to your Cargo bin directory (usually `~/.cargo/bin/`), which should be in your PATH.

#### Option 2: Manual installation

After building the release version, create a symbolic link to the executable in a directory that's in your PATH:

For macOS/Linux:
```bash
sudo ln -s "$(pwd)/target/release/password-generator" /usr/local/bin/password-generator
```

For Windows, add the full path to the executable to your PATH environment variable.

## Usage

Run the password generator from the terminal:

If installed globally:
```bash
password-generator [OPTIONS]
```

If using the local build:
```bash
./target/release/password-generator [OPTIONS]
```

### Options

- `-l, --length <NUMBER>`: Specify the length of the password (default is 16).
- `-U, --uppercase`: Include uppercase letters (enabled by default).
- `-L, --lowercase`: Include lowercase letters (enabled by default).
- `-n, --numbers`: Include numbers (enabled by default).
- `-s, --symbols`: Include special characters (enabled by default).
- `-i, --interactive`: Run in interactive mode, with prompts for each option.
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

## Examples

Generate a password with the default settings (16 characters with all character types):

```bash
password-generator
```

Generate a password with a specific length:

```bash
password-generator --length 24
```

Generate a password with only lowercase letters and numbers:

```bash
password-generator --no-uppercase --no-symbols
```

Run in interactive mode:

```bash
password-generator --interactive
```

Note: If using the local build instead of the global installation, prefix commands with `./target/release/`

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests to help improve the project.

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Commit your changes (`git commit -m 'Add some feature'`).
4. Push to the branch (`git push origin feature-branch`).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- Built with the [Rust](https://www.rust-lang.org/) programming language.
- Uses [clap](https://github.com/clap-rs/clap) for command-line argument parsing.
- Uses [rand](https://github.com/rust-random/rand) for secure random number generation.
- Inspired by the need for simple and secure password generation tools in the terminal.