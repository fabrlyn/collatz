use std::num::NonZeroU64;

// TODO: Handle overflow of numbers
// TODO: Handle different archs
// TODO: cli command for count
// TODO: cli command for iterate
// TODO: extend cli command for iterate with enumeration
// TODO: Check of-by-one error regarding enumeration and counting - starting at 1 counts as 0 steps

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

pub fn count(number: NonZeroU64) -> u64 {
    let mut count = 0;
    let mut next_number = number;

    while let Some(number) = next(next_number) {
        next_number = number;
        count += 1;
    }

    count
}
