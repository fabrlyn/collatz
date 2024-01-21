use std::num::NonZeroU64;

// TODO: Handle overflow of numbers
// TODO: Handle different archs
// TODO: extend cli command for iterate with enumeration
// TODO: Check of-by-one error regarding enumeration and counting - starting at 1 counts as 0 steps

pub fn next(number: NonZeroU64) -> Option<NonZeroU64> {
    if number.get() == 1 {
        return None;
    }

    if number.get() % 2 == 0 {
        NonZeroU64::new(number.get() / 2)
    } else {
        NonZeroU64::new((number.get() * 3) + 1)
    }
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

pub fn iterate(number: NonZeroU64) -> impl Iterator<Item = u64> {
    Collatz {
        number: Some(number),
    }
}
