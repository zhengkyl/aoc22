fn parse<S: AsRef<str>>(input: &[S]) -> impl Iterator<Item = ((u32, u32), (u32, u32))> + '_ {
    input.into_iter().map(|line| {
        let values: Vec<u32> = line
            .as_ref()
            .split(|c| c == '-' || c == ',')
            .map(|s| s.parse().unwrap())
            .collect();
        ((values[0], values[1]), (values[2], values[3]))
    })
}
pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    let mut sum = 0;
    parse(input).for_each(|(x, y)| {
        if (x.0 <= y.0 && y.1 <= x.1) || (y.0 <= x.0 && x.1 <= y.1) {
            sum += 1;
        }
    });

    sum
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    let mut sum = 0;
    parse(input).for_each(|(x, y)| {
        if (x.0 < y.0 && x.1 < y.0) || (y.0 < x.0 && y.1 < x.0) {
            return;
        }
        sum += 1;
    });

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "2-4,6-8", "5-7,7-9", "2-8,3-7", "2-3,4-5", "6-6,4-6", "2-6,4-8",
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), 4);
    }
}
