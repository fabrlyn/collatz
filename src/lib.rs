use std::ops::{Div, Mul, Rem};

// TODO: Handle overflow of numbers
// TODO: Handle different archs
// TODO: cli command for count
// TODO: cli command for iterate
// TODO: extend cli command for iterate with enumeration
// TODO: Check of-by-one error regarding enumeration and counting - starting at 1 counts as 0 steps

/// A valid number within the Collatz conjecture - any positive integer
#[derive(Clone, Copy)]
pub struct Number(u64);

impl Number {
    /// Create a new [Number]. Must be a postive integer.
    pub fn new(number: u64) -> Option<Self> {
        if number == 0 {
            None
        } else {
            Some(Self(number))
        }
    }

    /// Get the value of the [Number]
    pub fn value(&self) -> u64 {
        self.0
    }
}

impl PartialEq<u64> for Number {
    fn eq(&self, other: &u64) -> bool {
        self.0 == *other
    }
}

impl Rem<u64> for Number {
    type Output = u64;

    fn rem(self, rhs: u64) -> Self::Output {
        self.0 % rhs
    }
}

impl Div<u64> for Number {
    type Output = u64;

    fn div(self, rhs: u64) -> Self::Output {
        self.0 / rhs
    }
}

impl Mul<u64> for Number {
    type Output = u64;

    fn mul(self, rhs: u64) -> Self::Output {
        self.0 * rhs
    }
}

impl TryFrom<u64> for Number {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Number::new(value).ok_or(())
    }
}

struct Collatz {
    number: Option<Number>,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.number?;
        self.number = next(current);

        Some(current.0)
    }
}

pub fn iterate(number: Number) -> impl Iterator<Item = u64> {
    Collatz {
        number: Some(number),
    }
}

pub fn next(number: Number) -> Option<Number> {
    if number == 1 {
        return None;
    }

    if number % 2 == 0 {
        Number::new(number / 2)
    } else {
        Number::new((number * 3) + 1)
    }
}

pub fn count(number: Number) -> u64 {
    let mut count = 0;
    let mut next_number = number;

    while let Some(number) = next(next_number) {
        next_number = number;
        count += 1;
    }

    count
}
