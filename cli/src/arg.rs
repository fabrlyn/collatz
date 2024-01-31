use std::{
    io::{self, stdin},
    str::FromStr,
};

use collatz::{num::BigUint, Number};

#[derive(Clone, Debug)]
pub struct NumberArg(Number);

impl FromStr for NumberArg {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BigUint::from_str(s.trim())
            .ok()
            .and_then(Number::new)
            .ok_or("Not a valid number, must be a positive integer.".to_string())
            .map(NumberArg)
    }
}

impl From<NumberArg> for Number {
    fn from(value: NumberArg) -> Self {
        value.0
    }
}

#[derive(Clone, Debug)]
pub enum StdinArg<A> {
    Arg(A),
    Stdin,
}

impl<A> StdinArg<A> {
    pub fn into_or_parse_line<P, E>(self, parser: P) -> Result<Result<A, E>, io::Error>
    where
        P: Fn(&str) -> Result<A, E>,
    {
        match self {
            StdinArg::Arg(arg) => Ok(Ok(arg)),
            StdinArg::Stdin => {
                let mut buffer = String::new();
                stdin().read_line(&mut buffer)?;
                Ok(parser(&buffer))
            }
        }
    }
}

impl<A, E> StdinArg<A>
where
    A: FromStr<Err = E>,
    E: Into<String>,
{
    pub fn parse(input: &str) -> Result<Self, String> {
        match input {
            "-" => Ok(Self::Stdin),
            input => A::from_str(input)
                .map(Self::Arg)
                .map_err(Into::into)
                .map_err(|e| format!("{e} Use '-' for piping from stdin.")),
        }
    }
}
