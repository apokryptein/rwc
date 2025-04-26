use clap::Parser;
use std::io::Read;
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

// Stores filename and counts for each supplied file
// or piped input from stdin
struct Result {
    filename: String,
    lines: usize,
    words: usize,
    bytes: usize,
}

fn main() {
    // Parse Args
    let args = Args::parse();
    let results: Vec<_> = get_input(&args).collect();

    // Variables to store max count values
    // Used to provide totals when multiple files are provided
    // and to determine appropriate column width for output
    let (mut word_total, mut line_total, mut byte_total) = (0, 0, 0);

    // Get largest number to determine column width
    let largest_value = results
        .iter()
        .flat_map(|res| vec![res.lines, res.words, res.bytes])
        .max()
        .unwrap_or(0);

    // Get number of digits in largest number
    let column_width = largest_value.to_string().chars().count();

    // Iterate over results, sum values, and output data
    for result in results {
        line_total += result.lines;
        word_total += result.words;
        byte_total += result.bytes;
        print_file_data(&result, args.lines, args.words, args.bytes, column_width);
    }

    // Print totals if more than one file has been provided
    if args.files.len() > 1 {
        println!(
            "{0: >column_width$} {1: >column_width$} {2: >column_width$} total",
            line_total, word_total, byte_total
        );
    }
}

// Parses argument to retrieve input and store in Result
// or Vec<Result> then returns an iterator
fn get_input(args: &Args) -> impl Iterator<Item = Result> {
    // Vec of Result to store all result from provided files
    let mut results: Vec<Result> = Vec::new();

    // If files were provided read and parse them
    if !args.files.is_empty() {
        for file in &args.files {
            match fs::read_to_string(file) {
                Ok(content) => {
                    results.push(parse_input(&content, file));
                }
                Err(_) => continue,
            };
        }
    // If piped input from stdin or no args
    } else {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read from stdin");
        results.push(parse_input(input.as_str(), "-"));
    }

    results.into_iter()
}

// Parses input according to selected flags
fn parse_input(input: &str, file: &str) -> Result {
    // Result Vec
    Result {
        filename: String::from(file),
        lines: input.lines().count(),
        words: input.split_whitespace().count(),
        bytes: input.len(),
    }
}

// Prints data for one processed file
fn print_file_data(result: &Result, lines: bool, words: bool, bytes: bool, width: usize) {
    // Vec to build output string
    let mut output = Vec::new();

    // wc displays results in the following order:
    // Lines Words Bytes Filename
    // If lines flag add data to result
    if lines {
        output.push(result.lines.to_string());
    }

    // If words flag add data to result
    if words {
        output.push(result.words.to_string());
    }

    // If bytes flag add data to result
    if bytes {
        output.push(result.bytes.to_string());
    }

    // If no flag return all three: L W C Filename
    if !bytes && !lines && !words {
        // Lines
        output.push(result.lines.to_string());

        // Words
        output.push(result.words.to_string());

        // Bytes
        output.push(result.bytes.to_string());
    }

    // Print result to console
    if result.filename == "-" {
        println!("{}", output.join("\t"));
    } else {
        println!(
            "{0: >width$} {1: >width$} {2: >width$} {3:}",
            output[0], output[1], output[2], result.filename
        );
    }
}
