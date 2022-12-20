use std::cmp::{max, min};

fn min_max<T: Ord>(a: T, b: T) -> (T, T) {
    if a <= b {
        return (a, b);
    }
    (b, a)
}

pub fn solve<S: AsRef<str>, const PART_2: bool>(input: &[S]) -> u32 {
    // y increases with depth
    let mut max_y: usize = 0;
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;

    let paths = input
        .into_iter()
        .map(|line| {
            line.as_ref()
                .split(" -> ")
                .filter_map(|pair_str| {
                    let pair_str = pair_str.split_once(',')?;
                    let i = pair_str.0.parse::<usize>().ok()?;
                    let j = pair_str.1.parse::<usize>().ok()?;

                    Some((i, j))
                })
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>();

    paths.iter().for_each(|coords| {
        for coord in coords {
            max_y = max(max_y, coord.1);
            min_x = min(min_x, coord.0);
            max_x = max(max_x, coord.0);
        }
    });

    let walls_width = max_x - min_x + 1;
    let built_width = 2 * (max_y + 2) + 1;

    let (width, width_offset) = if PART_2 && built_width >= walls_width {
        (built_width, 500 - (built_width as i32 / 2))
    } else {
        (walls_width, min_x as i32)
    };

    let mut grid = vec![vec![false; max_y + 2 + 1]; width];

    // part 2
    if PART_2 {
        for i in 0..width {
            grid[i][max_y + 2] = true;
        }
    }

    paths.iter().for_each(|coords| {
        coords.windows(2).for_each(|window| {
            let &[(a, b), (c, d)] = window else {
                  panic!();
                };

            let (a, c) = min_max(
                (a as i32 - width_offset) as usize,
                (c as i32 - width_offset) as usize,
            );
            let (b, d) = min_max(b, d);

            for i in a..=c {
                for j in b..=d {
                    grid[i][j] = true;
                }
            }
        });
    });

    let mut count = 0;

    loop {
        let mut sand = ((500 - width_offset) as usize, 0);

        loop {
            // will always flow to left with nothing below
            if sand.0 == 0 {
                return count;
            }

            // no possible ledge below
            if !PART_2 && sand.1 == max_y {
                return count;
            }

            // move "down"
            if !grid[sand.0][sand.1 + 1] {
                sand.1 += 1;
                continue;
            }

            // move left
            if !grid[sand.0 - 1][sand.1 + 1] {
                sand.0 -= 1;
                sand.1 += 1;
                continue;
            }

            // flow off to right
            if sand.0 == width - 1 {
                return count;
            }

            // move right
            if !grid[sand.0 + 1][sand.1 + 1] {
                sand.0 += 1;
                sand.1 += 1;
                continue;
            }

            // stuck
            count += 1;
            grid[sand.0][sand.1] = true;

            // only applicable in part 2
            if PART_2 && sand.1 == 0 {
                return count;
            }

            break;
        }
    }
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    solve::<S, false>(input)
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    solve::<S, true>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 24);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), 93);
    }
}
