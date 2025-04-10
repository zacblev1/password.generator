use clap::Parser;
use rand::Rng;
use std::io;

#[derive(Parser)]
#[command(author, version, about = "A simple terminal password generator")]
struct Args {
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    #[arg(short = 'U', long, default_value_t = true)]
    uppercase: bool,

    #[arg(short, long, default_value_t = true)]
    lowercase: bool,

    #[arg(short, long, default_value_t = true)]
    numbers: bool,

    #[arg(short, long, default_value_t = true)]
    symbols: bool,

    #[arg(short, long, default_value_t = false)]
    interactive: bool,
}

fn main() {
    println!("🗝️  Terminal Password Generator\n");

    let mut args = Args::parse();

    if args.interactive {
        args.length = prompt_input("Enter password length: ")
            .trim()
            .parse::<usize>()
            .unwrap_or(16);

        args.uppercase = confirm("Include uppercase letters? (y/n): ");
        args.lowercase = confirm("Include lowercase letters? (y/n): ");
        args.numbers = confirm("Include numbers? (y/n): ");
        args.symbols = confirm("Include symbols? (y/n): ");
    }

    let charset = build_charset(args.uppercase, args.lowercase, args.numbers, args.symbols);

    if charset.is_empty() {
        println!("You must include at least one character set.");
        return;
    }

    let password = generate_password(args.length, &charset);
    println!("\nGenerated password: \x1b[1;32m{}\x1b[0m", password); // green bold text
}

fn prompt_input(prompt: &str) -> String {
    use std::io::Write;
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

fn confirm(prompt: &str) -> bool {
    let response = prompt_input(prompt);
    matches!(response.trim().to_lowercase().as_str(), "y" | "yes")
}

fn build_charset(upper: bool, lower: bool, numbers: bool, symbols: bool) -> Vec<char> {
    let mut charset = Vec::new();

    if upper {
        charset.extend('A'..='Z');
    }
    if lower {
        charset.extend('a'..='z');
    }
    if numbers {
        charset.extend('0'..='9');
    }
    if symbols {
        charset.extend("!@#$%^&*()-_=+[]{}|;:,.<>?".chars());
    }

    charset
}

fn generate_password(length: usize, charset: &[char]) -> String {
    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| charset[rng.gen_range(0..charset.len())])
        .collect()
}