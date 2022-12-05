use std::collections::{HashMap, HashSet};

fn score(c: char) -> u32 {
    u32::from(match c.is_ascii_lowercase() {
        true => c as u8 - 'a' as u8 + 1,
        false => c as u8 - 'A' as u8 + 27,
    })
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    input
        .into_iter()
        .map(|s| {
            let x = s.as_ref().len() / 2;
            let mut items = HashSet::new();
            s.as_ref().chars().take(x).for_each(|c| {
                items.insert(c);
            });
            let dupe = s
                .as_ref()
                .chars()
                .skip(x)
                .find(|c| items.contains(c))
                .unwrap();

            score(dupe)
        })
        .sum()
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    input
        .chunks(3)
        .map(|triple| {
            let mut items = HashMap::new();

            let s1 = triple.get(0).unwrap();
            let s2 = triple.get(1).unwrap();
            let s3 = triple.get(2).unwrap();

            s1.as_ref().chars().for_each(|c| {
                items.insert(c, 1);
            });
            s2.as_ref().chars().for_each(|c| {
                items.entry(c).and_modify(|e| *e += 1);
            });
            let c = s3
                .as_ref()
                .chars()
                .find(|c| {
                    let Some(count) = items.get(c) else {
                      return false;
                    };
                    *count > 1
                })
                .unwrap();

            score(c)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 157);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), 70);
    }
}
