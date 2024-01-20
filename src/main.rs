use std::str::FromStr;

use clap::{command, Args, Parser, Subcommand};
use collatz::Number;

fn parse_number(s: &str) -> Result<Number, String> {
    u64::from_str(s)
        .ok()
        .and_then(|number| Number::new(number))
        .ok_or("Not a valid number, must be a positive integer".to_string())
}

#[derive(Debug, Parser)]
#[command(name = "collatz")]
#[command(about = "Computations surrounding Collatz conjecture")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Iterate(Iterate),
}

#[derive(Debug, Args)]
struct Iterate {
    #[arg(value_parser = parse_number)]
    number: Number,
}

fn iterate(args: Iterate) {
    collatz::iterate(args.number).for_each(|n| {
        println!("{n}");
    });
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Iterate(a) => iterate(a),
    }
}
