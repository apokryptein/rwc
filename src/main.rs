use clap::Parser;
use std::{fs,io};

#[derive(Parser, Debug)]
#[command(name="rwc", version, about="A small word count utility written in Rust")]
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
    file: Option<String>,
}

fn main() {
    // Parse Args
    let args = Args::parse();

    // Read file into string
    let contents = match &args.file {
        Some(filename) => fs::read_to_string(filename).unwrap_or_else(|_| {
            eprintln!("Error reading file: {}", filename);
            std::process::exit(1);
        }),
        None => {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read from stdin");
            input
        }
    };

    // Result Vec
    let mut result = Vec::new();


    // wc displays results in the following order:
    // Lines Words Bytes Filename

    // If lines flag add data to result
    if args.lines {
        result.push(contents.lines().count().to_string());
    }

    // If words flag add data to result
    if args.words {
        result.push(contents.split_whitespace().count().to_string());
    }

    // If bytes flag add data to result
    if args.bytes {
        result.push(contents.len().to_string());
    }

    // If no flag return all three: L W C Filename
    if !args.bytes && !args.lines && !args.words {
        // Lines
        result.push(contents.lines().count().to_string());

        // Words
        result.push(contents.split_whitespace().count().to_string());

        // Bytes
        result.push(contents.len().to_string());

    }
    
    // Print result to console
    if args.file.is_some() {
        println!("{} {}", result.join(" "), args.file.as_deref().unwrap_or("-"));
    } else {
        println!("{}", result.join(" "));
    }
}
