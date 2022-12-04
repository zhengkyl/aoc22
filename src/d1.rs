fn get_counts(input: &Vec<String>) -> Vec<u32> {
    let mut sum = 0;

    let mut counts = vec![];

    for line in input {
        let Ok(num) = line.parse::<u32>() else {
          counts.push(sum);
          sum = 0;
          continue;
        };

        sum += num;
    }

    counts
}

pub fn part1(input: &Vec<String>) -> u32 {
    let counts = get_counts(input);
    counts.into_iter().max().unwrap_or(0)
}

pub fn part2(input: &Vec<String>) -> u32 {
    let mut counts = get_counts(input);
    counts.sort_by(|a, b| b.cmp(a));

    counts.into_iter().take(3).sum()
}
