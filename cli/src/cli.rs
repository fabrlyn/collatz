use std::{
    io::{self, stdin},
    ops::Sub,
    str::FromStr,
};

use clap::{command, Args, Parser, Subcommand};
use collatz::{num::BigUint, Number};

use crate::{
    arg::{NumberArg, StdinArg},
    util::GetOrExit,
};

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
#[command(about = "Count and print total number of steps in the sequence starting from <NUMBER>.")]
struct Count {
    #[arg(help = "A positive integer. Use '-' for piping from stdin.", value_parser = StdinArg::<NumberArg>::parse)]
    number: StdinArg<NumberArg>,
}

#[derive(Debug, Args)]
#[command(about = "Print each step in the sequence starting from <NUMBER>.")]
struct Sequence {
    #[arg(help = "Prefix each step with the step number.", long = "enumerate")]
    enumerate: bool,

    #[arg(help = "A positive integer. Use '-' for piping from stdin.", value_parser = StdinArg::<NumberArg>::parse)]
    number: StdinArg<NumberArg>,
}

fn count(args: Count) {
    let number = args
        .number
        .into_or_parse_line(NumberArg::from_str)
        .get_or_exit();

    let count = collatz::sequence(number).count().sub(1);
    println!("{count}")
}

fn steps(args: Sequence) {
    let number = args
        .number
        .into_or_parse_line(NumberArg::from_str)
        .get_or_exit();

    let sequence = collatz::sequence(number);

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

pub fn run() {
    match Cli::parse().command {
        Command::Count(args) => count(args),
        Command::Sequence(args) => steps(args),
    }
}
