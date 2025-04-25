use clap::Parser;
use std::fs;

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
    file: String,
}

fn main() {
    // Parse Args
    let args = Args::parse();

    // Read file into string
    let contents = match fs::read_to_string(&args.file) {
        Ok(text) => text,
        Err(_) => {
            eprintln!("Error reading file: {}", args.file);
            std::process::exit(1);
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
    println!("{} {}", result.join(" "), args.file);
}
