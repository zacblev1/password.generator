use rand::Rng;
use std::io;

fn main() {
    println!("🗝️  Terminal Password Generator\n");

    let length = prompt_input("Enter password length: ")
        .trim()
        .parse::<usize>()
        .expect("Please enter a valid number.");

    let include_uppercase = confirm("Include uppercase letters? (y/n): ");
    let include_lowercase = confirm("Include lowercase letters? (y/n): ");
    let include_numbers = confirm("Include numbers? (y/n): ");
    let include_symbols = confirm("Include symbols? (y/n): ");

    let charset = build_charset(include_uppercase, include_lowercase, include_numbers, include_symbols);

    if charset.is_empty() {
        println!("You must include at least one character set.");
        return;
    }

    let password = generate_password(length, &charset);
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