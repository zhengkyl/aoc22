pub fn get_values<S: AsRef<str>>(input: &[S]) -> Vec<i32> {
    let mut prev_value = 1;

    let mut values = vec![1];

    input.into_iter().for_each(|line| {
        let mut iter = line.as_ref().split(' ');
        match iter.next() {
            Some("addx") => {
                let delta: i32 = iter.next().unwrap().parse().unwrap();

                values.push(prev_value);

                prev_value += delta;

                values.push(prev_value);
            }
            Some("noop") => values.push(prev_value),
            _ => unreachable!(),
        }
    });

    values
}
pub fn part1<S: AsRef<str>>(input: &[S]) -> i32 {
    let values = get_values(input);

    [20, 60, 100, 140, 180, 220]
        .into_iter()
        .fold(0, |acc: i32, i_plus_1| {
            acc + values[i_plus_1 as usize - 1] * i_plus_1
        })
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> String {
    let values = get_values(input);

    let mut result = String::with_capacity(246);

    // values > 240 b/c we push all known future states
    values.into_iter().take(240).enumerate().for_each(|(i, v)| {
        let scan = (i % 40) as i32;
        if (v % 40 - scan).abs() <= 1 {
            result.push('#');
        } else {
            result.push('.');
        }

        if scan == 39 {
            result.push('\n');
        }
    });

    result
}
#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
        "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1",
        "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1", "addx 16",
        "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop", "addx -3", "addx 9",
        "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop", "noop", "noop", "noop",
        "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop", "addx 2", "addx 6", "noop",
        "noop", "noop", "noop", "noop", "addx 1", "noop", "noop", "addx 7", "addx 1", "noop",
        "addx -13", "addx 13", "addx 7", "noop", "addx 1", "addx -33", "noop", "noop", "noop",
        "addx 2", "noop", "noop", "noop", "addx 8", "noop", "addx -1", "addx 2", "addx 1", "noop",
        "addx 17", "addx -9", "addx 1", "addx 1", "addx -3", "addx 11", "noop", "noop", "addx 1",
        "noop", "addx 1", "noop", "noop", "addx -13", "addx -19", "addx 1", "addx 3", "addx 26",
        "addx -30", "addx 12", "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9",
        "addx 18", "addx 1", "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1",
        "addx 2", "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22",
        "addx -6", "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop",
        "addx 20", "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop ",
    ];

    const OUTPUT: &str = "##..##..##..##..##..##..##..##..##..##..\n\
                          ###...###...###...###...###...###...###.\n\
                          ####....####....####....####....####....\n\
                          #####.....#####.....#####.....#####.....\n\
                          ######......######......######......####\n\
                          #######.......#######.......#######.....\n\
";

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 13140);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), OUTPUT);
    }
}
