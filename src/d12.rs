pub fn solve<S: AsRef<str>, const MULTI: bool>(input: &[S]) -> u32 {
    let rows = input.len();
    let cols = input[0].as_ref().len();

    let mut min_dist = vec![vec![u32::MAX; cols]; rows];

    let mut height = vec![vec![0; cols]; rows];

    let mut e = (0, 0);

    let mut stack = Vec::new();

    for i in 0..rows {
        for j in 0..cols {
            let mut c = input[i].as_ref().as_bytes()[j];
            if c == 'S' as u8 {
                c = 'a' as u8;

                stack.push((i, j));
            } else if c == 'E' as u8 {
                e = (i, j);

                c = 'z' as u8;
            } else if MULTI && c == 'a' as u8 {
                stack.push((i, j));
            }
            c -= 'a' as u8;
            height[i][j] = c;
        }
    }

    for pos in &stack {
        min_dist[pos.0][pos.1] = 0;
    }

    while stack.len() > 0 {
        let (i, j) = stack.pop().unwrap();

        let ii = i as i32;
        let jj = j as i32;

        let dir = [(ii - 1, jj), (ii + 1, jj), (ii, jj - 1), (ii, jj + 1)];

        for (x, y) in dir {
            if x < 0 || y < 0 {
                continue;
            }

            let x = x as usize;
            let y = y as usize;

            if x >= rows || y >= cols {
                continue;
            }

            if height[x][y] > height[i][j] + 1 {
                continue;
            }

            if min_dist[i][j] + 1 < min_dist[x][y] {
                min_dist[x][y] = min_dist[i][j] + 1;
                stack.push((x, y));
            }
        }
    }

    min_dist[e.0][e.1]
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

    const SAMPLE: &[&str] = &["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 31);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), 29);
    }
}
