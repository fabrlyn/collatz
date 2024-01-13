struct Collatz {
    number: Option<Number>,
}

#[derive(Clone, Copy)]
struct Number(u64);

impl Number {
    fn new(number: u64) -> Option<Self> {
        if number == 0 {
            None
        } else {
            Some(Self(number))
        }
    }
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.number?.0;
        self.number = next(current).and_then(Number::new);

        Some(current)
    }
}

pub fn iterate(number: u64) -> Option<impl Iterator<Item = u64>> {
    Some(Collatz {
        number: Some(Number::new(number)?),
    })
}

pub fn next(number: u64) -> Option<u64> {
    let number = Number::new(number)?;

    if number.0 == 1 {
        return None;
    }

    if number.0 % 2 == 0 {
        Some(number.0 / 2)
    } else {
        Some((number.0 * 3) + 1)
    }
}

pub fn count(number: u64) -> u64 {
    let mut count = 0;
    let mut next_number = number;

    while let Some(number) = next(next_number) {
        next_number = number;
        count += 1;
    }

    count
}
