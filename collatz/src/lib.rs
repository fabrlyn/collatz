use std::ops::{Add, Div, Mul};

use num::{BigUint, Integer, One, Zero};

pub use num;

/// A valid number in the Collatz sequence.
///
/// A positive integer.
#[derive(Clone, Debug, PartialEq)]
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
pub fn sequence<N>(number: N) -> impl Iterator<Item = BigUint>
where
    N: Into<Number>,
{
    Sequence {
        number: Some(number.into()),
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::Number;

    #[rstest]
    #[case(Number::new(10u32).unwrap(), BigUint::from(10u32))]
    fn get_number_value(#[case] number: Number, #[case] expected: BigUint) {
        assert_eq!(expected, number.value());
    }

    #[rstest]
    #[case(0, None)]
    #[case(1, Some(Number(1u32.into())))]
    fn new_number(#[case] input: u32, #[case] expected: Option<Number>) {
        assert_eq!(Number::new(input), expected)
    }

    #[rstest]
    #[case(Number::new(1u32).unwrap(), None)]
    #[case(Number::new(2u32).unwrap(), Number::new(1u32))]
    #[case(Number::new(3u32).unwrap(), Number::new(10u32))]
    fn next(#[case] input: Number, #[case] expected: Option<Number>) {
        assert_eq!(expected, super::next(input))
    }

    #[rstest]
    #[case(Number::new(1u32).unwrap(), vec![BigUint::from(1u32)])]
    #[case(Number::new(4u32).unwrap(), vec![BigUint::from(4u32), BigUint::from(2u32), BigUint::from(1u32)])]
    fn sequence(#[case] input: Number, #[case] expected: Vec<BigUint>) {
        assert_eq!(expected, super::sequence(input).collect::<Vec<_>>())
    }
}
