use std::{
    io::{self, stdin},
    ops::Sub,
    str::FromStr,
};

use clap::{command, Args, Parser, Subcommand};
use collatz::Number;
use num::BigUint;

#[derive(Clone, Debug)]
struct NumberArg(Number);

impl FromStr for NumberArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigUint::from_str(s.trim())
            .ok()
            .and_then(|number| Number::new(number))
            .ok_or("Not a valid number, must be a positive integer".to_string())
            .map(NumberArg)
    }
}

#[derive(Clone, Debug)]
enum ArgOrStdin<A> {
    Arg(A),
    Stdin,
}

impl<A> ArgOrStdin<A> {
    fn into_or_parse_line<P, E>(self, parser: P) -> Result<Result<A, E>, io::Error>
    where
        P: Fn(&str) -> Result<A, E>,
    {
        match self {
            ArgOrStdin::Arg(arg) => Ok(Ok(arg)),
            ArgOrStdin::Stdin => {
                let mut buffer = String::new();
                stdin().read_line(&mut buffer)?;
                Ok(parser(&buffer))
            }
        }
    }
}

impl<A, E> ArgOrStdin<A>
where
    A: FromStr<Err = E>,
    E: Into<String>,
{
    fn parse(input: &str) -> Result<Self, String> {
        match input {
            "-" => Ok(Self::Stdin),
            input => A::from_str(input)
                .map(Self::Arg)
                .map_err(Into::into)
                .map_err(|e| format!("{e}. Use '-' for piping from stdin.")),
        }
    }
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
    #[arg(help = "A positive integer", value_parser = ArgOrStdin::<NumberArg>::parse)]
    number: ArgOrStdin<NumberArg>,
}

#[derive(Debug, Args)]
#[command(about = "Print each step in the sequence starting from <NUMBER>")]
struct Sequence {
    #[arg(help = "Prefix each step with the step number", long = "enumerate")]
    enumerate: bool,

    #[arg(help = "A positive integer", value_parser = ArgOrStdin::<NumberArg>::parse)]
    number: ArgOrStdin<NumberArg>,
}

fn count(args: Count) {
    let number = match args.number.into_or_parse_line(NumberArg::from_str) {
        Ok(Ok(number)) => number.0,
        Ok(Err(error)) => clap::Error::raw(clap::error::ErrorKind::Format, error).exit(),
        Err(error) => clap::Error::raw(clap::error::ErrorKind::Io, error).exit(),
    };

    let count = collatz::sequence(number).count().sub(1);
    println!("{count}")
}

fn steps(args: Sequence) {
    let number = match args.number {
        ArgOrStdin::Arg(number) => number.0,
        ArgOrStdin::Stdin => todo!(),
    };

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

fn main() {
    match Cli::parse().command {
        Command::Count(args) => count(args),
        Command::Sequence(args) => steps(args),
    }
}
