use std::ops::ControlFlow;

fn solve<S: AsRef<str>, const W: usize>(input: &[S]) -> u32 {
    let x = input[0]
        .as_ref()
        .as_bytes()
        .windows(W)
        .enumerate()
        .try_for_each(|(i, val)| {
            let ones = val
                .into_iter()
                .map(|v| 1 << v - 'a' as u8)
                .fold(0u32, |acc, v| acc | v)
                .count_ones();
            if ones == W as u32 {
                return ControlFlow::Break(i + W);
            }

            ControlFlow::Continue(())
        });
    match x {
        ControlFlow::Break(index) => index as u32,
        _ => unreachable!(),
    }
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    solve::<S, 4>(input)
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    solve::<S, 14>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &[&str] = &["mjqjpqmgbljsphdztnvjfqwrcgsmlb"];
    const SAMPLE_2: &[&str] = &["bvwbjplbgvbhsrlpgdmjqwftvncz"];
    const SAMPLE_3: &[&str] = &["nppdvjthqldpwncqszvftbrmjlhg"];
    const SAMPLE_4: &[&str] = &["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"];
    const SAMPLE_5: &[&str] = &["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE_1), 7);
        assert_eq!(part1(SAMPLE_2), 5);
        assert_eq!(part1(SAMPLE_3), 6);
        assert_eq!(part1(SAMPLE_4), 10);
        assert_eq!(part1(SAMPLE_5), 11);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE_1), 19);
        assert_eq!(part2(SAMPLE_2), 23);
        assert_eq!(part2(SAMPLE_3), 23);
        assert_eq!(part2(SAMPLE_4), 29);
        assert_eq!(part2(SAMPLE_5), 26);
    }
}
