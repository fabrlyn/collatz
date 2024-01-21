use std::{
    num::NonZeroU64,
    ops::{Div, Mul, Add},
};

use num::{Integer, One};

pub fn next(number: NonZeroU64) -> Option<NonZeroU64> {
    if number.get().is_one() {
        return None;
    }

    let next = if number.get().is_even() {
        number.get().div(2)
    } else {
        (number.get().mul(3)).add(1)
    };

    NonZeroU64::new(next)
}

struct Collatz {
    number: Option<NonZeroU64>,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.number?;
        self.number = next(current);

        Some(current.into())
    }
}

pub fn steps(number: NonZeroU64) -> impl Iterator<Item = u64> {
    Collatz {
        number: Some(number),
    }
}
