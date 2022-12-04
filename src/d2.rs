fn parse<S: AsRef<str>>(input: &[S]) -> impl Iterator<Item = (u8, u8)> + '_ {
    input.into_iter().filter_map(|line| {
        let mut i = line.as_ref().split_ascii_whitespace();
        Some((
            i.next()?.as_bytes().first()? - 'A' as u8,
            i.next()?.as_bytes().first()? - 'X' as u8,
        ))
    })
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    parse(input)
        .map(|(left, right)| {
            let bonus = u32::from(right) + 1;
            let score = match left {
                value if value == right => 3,
                value if value == (right + 2) % 3 => 6,
                _ => 0,
            };
            score + bonus
        })
        .sum()
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    parse(input)
        .map(|(left, right)| {
            u32::from(match right {
                0 => ((left + 2) % 3) + 1,     // lose
                1 => 3 + left + 1,             // draw
                2 => 6 + ((left + 1) % 3) + 1, // win
                _ => panic!("parse failed"),
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &["A Y", "B X", "C Z"];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 15);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), 12);
    }
}
