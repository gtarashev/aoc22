use aoc22::*;
use clap::Parser;

/// Choose which day to run and which file to use for the data
#[derive(Parser)]
struct Cli {
    /// The day to be ran
    day: String,
    /// The path to the data file
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    match &args.day[..] {
        "1" => day1(&args.path),
        _ => println!("Either invalid input or day isn't available yet"),
    }
}
