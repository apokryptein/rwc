use clap::Parser;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(
    name = "rwc",
    version,
    about = "A small word count utility written in Rust"
)]
struct Args {
    /// Count lines
    #[arg(short = 'l', long)]
    lines: bool,

    /// Count words
    #[arg(short = 'w', long)]
    words: bool,

    /// Count bytes
    #[arg(short = 'c', long)]
    bytes: bool,

    /// File
    #[arg()]
    files: Vec<String>,
}

fn main() {
    // Parse Args
    let args = Args::parse();

    let mut input = String::new();

    if args.files.is_empty() {
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read from stdin");
        parse_input(input.as_str(), "-", args.lines, args.words, args.bytes)
    } else {
        for file in &args.files {
            match fs::read_to_string(file) {
                Ok(content) => parse_input(&content, file, args.lines, args.words, args.bytes),
                Err(_) => eprintln!("Error reading file: {}", file),
            }
        }
    }
}

fn parse_input(input: &str, file: &str, lines: bool, words: bool, bytes: bool) {
    // Result Vec
    let mut result = Vec::new();

    // wc displays results in the following order:
    // Lines Words Bytes Filename
    // If lines flag add data to result
    if lines {
        result.push(input.lines().count().to_string());
    }

    // If words flag add data to result
    if words {
        result.push(input.split_whitespace().count().to_string());
    }

    // If bytes flag add data to result
    if bytes {
        result.push(input.len().to_string());
    }

    // If no flag return all three: L W C Filename
    if !bytes && !lines && !words {
        // Lines
        result.push(input.lines().count().to_string());

        // Words
        result.push(input.split_whitespace().count().to_string());

        // Bytes
        result.push(input.len().to_string());
    }

    // Print result to console
    if file != "-" {
        println!("{} {}", result.join(" "), file);
    } else {
        println!("{}", result.join(" "));
    }
}
