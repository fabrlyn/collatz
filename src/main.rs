use std::{num::NonZeroU64, ops::Sub};

use clap::{command, Args, Parser, Subcommand};

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
    number: NonZeroU64,
}

#[derive(Debug, Args)]
struct Steps {
    number: NonZeroU64,
    #[arg(value_name = "enumerate", long = "enumerate")]
    enumerate: bool,
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
