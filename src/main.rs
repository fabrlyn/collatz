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
#[command(name = "collatz")]
#[command(about = "Computations surrounding Collatz conjecture")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Count(Count),
    Steps(Steps),
}

#[derive(Debug, Args)]
struct Count {
    #[arg(value_parser = parse_number)]
    number: Number,
}

#[derive(Debug, Args)]
struct Steps {
    #[arg(value_name = "enumerate", long = "enumerate")]
    enumerate: bool,
    #[arg(value_parser = parse_number)]
    number: Number,
}

fn count(args: Count) {
    let count = collatz::steps(args.number).count().sub(1);
    println!("{count}")
}

fn steps(args: Steps) {
    let steps = collatz::steps(args.number);

    if args.enumerate {
        steps.enumerate().for_each(|(step, number)| {
            println!("{step}: {number}");
        });
    } else {
        steps.for_each(|number| {
            println!("{number}");
        });
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Count(args) => count(args),
        Command::Steps(args) => steps(args),
    }
}
