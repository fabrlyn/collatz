use std::num::NonZeroU64;

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
    Iterate(Iterate),
}

#[derive(Debug, Args)]
struct Count {
    number: NonZeroU64,
}

#[derive(Debug, Args)]
struct Iterate {
    number: NonZeroU64,
}

fn count(args: Count) {
    let count = collatz::iterate(args.number).count();
    println!("{count}")
}

fn iterate(args: Iterate) {
    collatz::iterate(args.number).for_each(|n| {
        println!("{n}");
    });
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Count(args) => count(args),
        Command::Iterate(args) => iterate(args),
    }
}
