use std::ops::{Add, Div, Mul};

use num::{BigUint, Integer, One, Zero};

/// A valid number in the Collatz sequence.
///
/// A positive integer.
#[derive(Debug, Clone)]
pub struct Number(BigUint);

impl Number {
    /// Create a new [Number].
    ///
    /// Returns [None] if `number` is zero(`0`).
    pub fn new<N: Into<BigUint>>(number: N) -> Option<Self> {
        let number = number.into();

        if number.is_zero() {
            None
        } else {
            Some(Self(number))
        }
    }

    /// Get the value of the [Number]
    pub fn value(self) -> BigUint {
        self.0
    }
}

/// Get the next number in the sequence.
///
/// Returns [None] if `number` is one(`1`).
/// The end of the sequence has been reached.
pub fn next(number: Number) -> Option<Number> {
    let number = number.value();

    if number.is_one() {
        return None;
    }

    let next = if number.is_even() {
        number.div(2u8)
    } else {
        (number.mul(3u8)).add(1u8)
    };

    Number::new(next)
}

struct Sequence {
    number: Option<Number>,
}

impl Iterator for Sequence {
    type Item = BigUint;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.number.take()?;
        self.number = next(current.clone());
        Some(current.value())
    }
}

/// Get the sequence starting from `number`.
pub fn sequence(number: Number) -> impl Iterator<Item = BigUint> {
    Sequence {
        number: Some(number),
    }
}
