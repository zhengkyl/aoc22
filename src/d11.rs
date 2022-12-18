use std::{iter, str::FromStr};

struct Monkey {
    items: Vec<u64>,
    op: Operation,
    div: u64,
    if_true: usize,
    if_false: usize,
}

#[derive(Clone, Copy)]
enum Operation {
    Old(),
    Mul(u64),
    Add(u64),
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(add) = s.strip_prefix("new = old + ") {
            Ok(Operation::Add(add.parse::<u64>().unwrap()))
        } else if let Some(_) = s.strip_prefix("new = old * old") {
            Ok(Operation::Old())
        } else if let Some(mul) = s.strip_prefix("new = old * ") {
            Ok(Operation::Mul(mul.parse::<u64>().unwrap()))
        } else {
            Err(())
        }
    }
}

impl Operation {
    fn apply(&self, x: u64) -> u64 {
        match self {
            Self::Add(y) => x + y,
            Self::Mul(y) => x * y,
            Self::Old() => x * x,
        }
    }
}

fn get_monkeys<S: AsRef<str>>(input: &[S]) -> Vec<Monkey> {
    let mut mon_iter = input.chunks(7);

    iter::from_fn(|| {
        let chunk = mon_iter.next()?;

        let items = chunk[1]
            .as_ref()
            .rsplit_once(':')?
            .1
            .split(',')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let op = chunk[2]
            .as_ref()
            .strip_prefix("  Operation: ")?
            .parse::<Operation>()
            .ok()?;

        let div = chunk[3].as_ref().rsplit_once(' ')?.1.parse::<u64>().ok()?;

        let if_true = chunk[4]
            .as_ref()
            .rsplit_once(' ')?
            .1
            .parse::<usize>()
            .ok()?;
        let if_false = chunk[5]
            .as_ref()
            .rsplit_once(' ')?
            .1
            .parse::<usize>()
            .ok()?;

        Some(Monkey {
            items,
            op,
            div,
            if_true,
            if_false,
        })
    })
    .collect::<Vec<Monkey>>()
}

pub fn solve<S: AsRef<str>, const D: u64, const N: u64>(input: &[S]) -> u64 {
    let mut monkeys = get_monkeys(input);
    let shared_mod = monkeys.iter().fold(1, |acc, mon| mon.div * acc);

    let mut counts = vec![0; monkeys.len()];

    for _ in 0..N {
        for j in 0..monkeys.len() {
            let mon = &mut monkeys[j];

            let items: Vec<u64> = mon.items.drain(..).collect();

            counts[j] += items.len();

            let Monkey {
                op,
                div,
                if_true,
                if_false,
                ..
            } = *mon;

            for item in items {
                let value = op.apply(item) / D % shared_mod;
                let throw = if value % div == 0 { if_true } else { if_false };

                monkeys[throw].items.push(value);
            }
        }
    }

    counts.sort();
    counts.reverse();
    (counts[0] * counts[1]) as u64
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u64 {
    solve::<S, 3, 20>(input)
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u64 {
    solve::<S, 1, 10000>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "Monkey 0:",
        "  Starting items: 79, 98",
        "  Operation: new = old * 19",
        "  Test: divisible by 23",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 3",
        "",
        "Monkey 1:",
        "  Starting items: 54, 65, 75, 74",
        "  Operation: new = old + 6",
        "  Test: divisible by 19",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 0",
        "",
        "Monkey 2:",
        "  Starting items: 79, 60, 97",
        "  Operation: new = old * old",
        "  Test: divisible by 13",
        "    If true: throw to monkey 1",
        "    If false: throw to monkey 3",
        "",
        "Monkey 3:",
        "  Starting items: 74",
        "  Operation: new = old + 3",
        "  Test: divisible by 17",
        "    If true: throw to monkey 0",
        "    If false: throw to monkey 1",
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 10605);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), 2713310158);
    }
}
