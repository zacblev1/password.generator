# Terminal Password Generator

A simple and secure terminal-based password generator written in Rust. This tool allows you to generate strong, random passwords directly from your terminal, making it easy to create secure credentials for your applications and services.

## Features

- **Customizable Password Length:** Specify the desired length of your password.
- **Random and Secure:** Generates cryptographically secure random passwords.
- **Easy to Use:** Simple command-line interface with clear instructions and options.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (version 1.40 or later) installed on your system.

## Installation

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/yourusername/password.generator.git
cd password.generator
```

Build the project using Cargo:

```bash
cargo build --release
```

This will compile the project and produce an executable in the `target/release` directory.

## Usage

Run the password generator from the terminal:

```bash
./target/release/password.generator [OPTIONS]
```

### Options

- `-l, --length <NUMBER>`: Specify the length of the password (default is 16).
- `-h, --help`: Print help information.

For example, to generate a 20-character password:

```bash
./target/release/password.generator --length 20
```

## Examples

Generate a password with the default settings:

```bash
./target/release/password.generator
```

Generate a password with a specific length:

```bash
./target/release/password.generator --length 24
```

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
- Inspired by the need for simple and secure password generation tools in the terminal.
