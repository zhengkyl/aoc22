fn get_counts<S: AsRef<str>>(input: &[S]) -> Vec<u32> {
    let mut sum = 0;

    let mut counts = vec![];

    for line in input {
        let Ok(num) = line.as_ref().parse::<u32>() else {
          counts.push(sum);
          sum = 0;
          continue;
        };

        sum += num;
    }

    if sum > 0 {
        counts.push(sum);
    }

    counts
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    let counts = get_counts(input);
    counts.into_iter().max().unwrap_or(0)
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    let mut counts = get_counts(input);
    counts.sort_by(|a, b| b.cmp(a));

    counts.into_iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "1000", "2000", "3000", "", "4000", "", "5000", "6000", "", "7000", "8000", "9000", "",
        "10000",
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 24000);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), 45000);
    }
}
