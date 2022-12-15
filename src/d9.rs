use std::collections::HashSet;

pub fn solve<S: AsRef<str>, const N: usize>(input: &[S]) -> u32 {
    let mut pos = vec![(0, 0); N];

    let mut visited = HashSet::new();

    visited.insert(pos[0]);

    for line in input {
        let mut iter = line.as_ref().split(' ');
        let dir = match iter.next().unwrap() {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => unreachable!(),
        };
        let steps: u32 = iter.next().unwrap().parse().unwrap();

        for _ in 0..steps {
            pos[N - 1].0 += dir.0;
            pos[N - 1].1 += dir.1;

            for k in (0..N - 1).rev() {
                if ((pos[k + 1].0 - pos[k].0) as i32).abs() <= 1
                    && ((pos[k + 1].1 - pos[k].1) as i32).abs() <= 1
                {
                    break;
                }

                if pos[k + 1].0 > pos[k].0 {
                    pos[k].0 += 1;
                }
                if pos[k + 1].0 < pos[k].0 {
                    pos[k].0 -= 1;
                }
                if pos[k + 1].1 > pos[k].1 {
                    pos[k].1 += 1;
                }
                if pos[k + 1].1 < pos[k].1 {
                    pos[k].1 -= 1;
                }

                if k == 0 {
                    visited.insert(pos[k]);
                }
            }
        }
    }

    visited.len() as u32
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    solve::<S, 2>(input)
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    solve::<S, 10>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &[&str] = &["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
    const SAMPLE_2: &[&str] = &["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE_1), 13);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE_1), 1);
        assert_eq!(part2(SAMPLE_2), 36);
    }
}
