mod arg;
mod cli;
mod util;

use std::{
    io::{self, stdin},
    ops::Sub,
    str::FromStr,
};

use clap::{command, Args, Parser, Subcommand};
use collatz::{num::BigUint, Number};

fn main() {
    cli::run();
}
