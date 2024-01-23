use std::{ops::Sub, str::FromStr};

use clap::{command, Args, Parser, Subcommand};
use collatz::Number;
use num::BigUint;

fn parse_number(s: &str) -> Result<Number, String> {
    BigUint::from_str(s)
        .ok()
        .and_then(|number| Number::new(number))
        .ok_or("Not a valid number, must be a positive integer".to_string())
}

#[derive(Debug, Parser)]
#[command(about = "Collatz conjecture cli", name = "collatz", author = "fabrlyn")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Count(Count),
    Sequence(Sequence),
}

#[derive(Debug, Args)]
#[command(about = "Count and print total number of steps in the sequence starting from <NUMBER>")]
struct Count {
    #[arg(help = "A positive integer", value_parser = parse_number)]
    number: Number,
}

#[derive(Debug, Args)]
#[command(about = "Print each step in the sequence starting from <NUMBER>")]
struct Sequence {
    #[arg(help = "Prefix each step with the step number", long = "enumerate")]
    enumerate: bool,

    #[arg(help = "A positive integer", value_parser = parse_number)]
    number: Number,
}

fn count(args: Count) {
    let count = collatz::sequence(args.number).count().sub(1);
    println!("{count}")
}

fn steps(args: Sequence) {
    let sequence = collatz::sequence(args.number);

    if args.enumerate {
        sequence.enumerate().for_each(|(step, number)| {
            println!("{step}: {number}");
        });
    } else {
        sequence.for_each(|number| {
            println!("{number}");
        });
    }
}

fn main() {
    match Cli::parse().command {
        Command::Count(args) => count(args),
        Command::Sequence(args) => steps(args),
    }
}
