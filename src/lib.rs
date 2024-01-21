use std::num::NonZeroU64;

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

pub fn steps(number: NonZeroU64) -> impl Iterator<Item = u64> {
    Collatz {
        number: Some(number),
    }
}
