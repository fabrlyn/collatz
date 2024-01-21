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
    Iterate(Iterate),
}

#[derive(Debug, Args)]
struct Iterate {
    number: NonZeroU64,
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
