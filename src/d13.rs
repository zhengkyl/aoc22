use std::{cmp::Ordering, iter, str::FromStr};

#[derive(PartialEq, Eq, Debug)]
enum Pack {
    List(Vec<Pack>),
    Num(u8),
}

impl FromStr for Pack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = s.chars().peekable();

        let mut stack: Vec<Vec<Pack>> = Vec::new();

        while left.peek() != None {
            match left.next().unwrap() {
                '[' => {
                    stack.push(Vec::new());
                }
                ']' => {
                    let list = stack.pop().unwrap();
                    if let Some(last) = stack.last_mut() {
                        last.push(Pack::List(list));
                    } else {
                        return Ok(Pack::List(list));
                    }
                }
                num @ '0'..='9' => {
                    let mut built = num as u8 - '0' as u8;
                    while let Some(nc) = left.peek() {
                        match nc {
                            '0'..='9' => {
                                let num = left.next().unwrap();
                                built *= 10;
                                built += num as u8 - '0' as u8;
                            }
                            _ => break,
                        }
                    }
                    stack.last_mut().unwrap().push(Pack::Num(built));
                }
                _ => {}
            }
        }

        Err(())
    }
}

impl Ord for Pack {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Pack::List(x), Pack::List(y)) => cmp_packet(x.iter(), y.iter()),
            (Pack::Num(x), Pack::Num(y)) => x.cmp(y),
            (Pack::List(x), y) => cmp_packet(x.iter(), iter::once(y)),
            (x, Pack::List(y)) => cmp_packet(iter::once(x), y.iter()),
        }
    }
}
impl PartialOrd for Pack {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn cmp_packet<Item: Ord, A: Iterator<Item = Item>, B: Iterator<Item = Item>>(
    mut a: A,
    mut b: B,
) -> Ordering {
    loop {
        return match (a.next(), b.next()) {
            (None, None) => Ordering::Equal,
            (None, Some(_)) => Ordering::Less,
            (Some(_), None) => Ordering::Greater,
            (Some(x), Some(y)) => match x.cmp(&y) {
                // Keep comparing until end or decision
                Ordering::Equal => continue,
                x => x,
            },
        };
    }
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    input
        .chunks(3)
        .enumerate()
        .filter_map(|(i, chunk)| {
            let left = chunk[0].as_ref().parse::<Pack>().ok()?;
            let right = chunk[1].as_ref().parse::<Pack>().ok()?;

            if left <= right {
                Some((i + 1) as u32)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    let two = Pack::List(vec![Pack::List(vec![Pack::Num(2)])]);
    let six = Pack::List(vec![Pack::List(vec![Pack::Num(6)])]);

    // x is at least 1, y is greater than x
    let mut x = 1;
    let mut y = 2;

    input
        .into_iter()
        .filter(|line| !line.as_ref().is_empty())
        .for_each(|packet| {
            let packet = packet.as_ref().parse::<Pack>().unwrap();
            if two > packet {
                x += 1;
                y += 1;
            } else if six > packet {
                y += 1;
            }
        });

    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 13);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), 140);
    }
}
