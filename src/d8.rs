use std::cmp;

#[derive(Debug, Clone)]
struct Tree {
    val: u8,
    adj: [u8; 4],
}

#[derive(Debug, Clone)]
struct Tree2 {
    val: u8,
    adj: [usize; 4],
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    let n = input.len();

    let mut grid = vec![
        vec![
            Tree {
                val: 0,
                adj: [0, 0, 0, 0],
            };
            n + 2
        ];
        n + 2
    ];

    input.into_iter().enumerate().for_each(|(row, line)| {
        line.as_ref().bytes().enumerate().for_each(|(i, b)| {
            let val = b - '0' as u8;

            // Offset indices by 1 to avoid bound checking when checking adj trees
            // Offset value by 1 so 0 is less than shortest tree
            grid[row + 1][i + 1].val = val + 1;
        })
    });

    let mut calc_max = |i: usize, j: usize, offset, vert, horz| {
        let x = (i as i32 + offset) as usize;
        let y = (j as i32 + offset) as usize;
        // Max top, right or bottom, left values so far
        grid[i][j].adj[vert] = cmp::max(grid[x][j].adj[vert], grid[x][j].val);
        grid[i][j].adj[horz] = cmp::max(grid[i][y].adj[horz], grid[i][y].val);
    };

    for i in 1..=n {
        for j in 1..=n {
            calc_max(i, j, -1, 0, 1);
        }
    }
    for i in (1..=n).rev() {
        for j in (1..=n).rev() {
            calc_max(i, j, 1, 2, 3);
        }
    }

    (1..=n)
        .map(|i| {
            (1..=n)
                .filter(|&j| grid[i][j].val > grid[i][j].adj.into_iter().min().unwrap())
                .count() as u32
        })
        .sum()
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    let n = input.len();

    let mut grid = vec![
        vec![
            Tree2 {
                val: 10,
                adj: [0, 0, n + 1, n + 1],
            };
            n + 2
        ];
        n + 2
    ];

    input.into_iter().enumerate().for_each(|(row, line)| {
        line.as_ref().bytes().enumerate().for_each(|(i, b)| {
            let val = b - '0' as u8;

            // Offset indices by 1 to avoid bound checking when checking adj trees
            grid[row + 1][i + 1].val = val;
        })
    });

    let mut calc_max = |i: usize, j: usize, offset, vert, horz| {
        let mut x = (i as i32 + offset) as usize;
        let mut y = (j as i32 + offset) as usize;

        while grid[i][j].val > grid[x][j].val {
            x = grid[x][j].adj[vert];
        }
        grid[i][j].adj[vert] = x;

        while grid[i][j].val > grid[i][y].val {
            y = grid[i][y].adj[horz];
        }
        grid[i][j].adj[horz] = y;
    };

    for i in 1..=n {
        for j in 1..=n {
            calc_max(i, j, -1, 0, 1);
        }
    }
    for i in (1..=n).rev() {
        for j in (1..=n).rev() {
            calc_max(i, j, 1, 2, 3);
        }
    }
    (1..=n)
        .map(|i| {
            (1..=n)
                .map(|j| {
                    let mut top = i - grid[i][j].adj[0];
                    let mut left = j - grid[i][j].adj[1];
                    if top == i {
                        top -= 1;
                    }
                    if left == j {
                        left -= 1;
                    }

                    let mut bot = grid[i][j].adj[2];
                    let mut right = grid[i][j].adj[3];

                    if bot == n + 1 {
                        bot -= 1;
                    }
                    bot -= i;
                    if right == n + 1 {
                        right -= 1;
                    }
                    right -= j;

                    (top * left * bot * right) as u32
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &["30373", "25512", "65332", "33549", "35390"];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 21);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), 8);
    }
}
