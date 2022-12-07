fn parse<S: AsRef<str>>(input: &[S]) -> (Vec<String>, Vec<(usize, usize, usize)>) {
    let split_point = input.into_iter().position(|s| s.as_ref() == "").unwrap();

    // Exclude line with labels for stacks, we can figure that out
    let stacks = &input[..split_point - 1];
    let moves = &input[split_point + 1..];

    (parse_stacks(stacks), parse_moves(moves))
}

fn parse_stacks<S: AsRef<str>>(input: &[S]) -> Vec<String> {
    let cols = (input[0].as_ref().len() + 1) / 4;
    let mut stacks = vec![String::new(); cols];

    // Lower items are pushed to the end of the String
    input.into_iter().for_each(|line| {
        let chars = line.as_ref().as_bytes();
        for i in 0..cols {
            let Some(letter) = chars.get(i * 4 + 1) else {
                break;
            };

            let letter = *letter as char;

            if letter != ' ' {
                stacks.get_mut(i).unwrap().push(letter);
            }
        }
    });

    stacks
}

fn parse_moves<S: AsRef<str>>(input: &[S]) -> Vec<(usize, usize, usize)> {
    input
        .into_iter()
        .map(|line| {
            let v: Vec<u32> = line
                .as_ref()
                .split(' ')
                .skip(1)
                .step_by(2)
                .map(|s| s.parse().unwrap())
                .collect();
            v
        })
        .map(|v| match v[..] {
            [first, second, third] => (first as usize, second as usize - 1, third as usize - 1),
            _ => unreachable!(),
        })
        .collect()
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> String {
    let (mut stacks, moves) = parse(input);

    for m in moves {
        let from = stacks.get_mut(m.1).unwrap();
        let top: String = from.drain(..m.0).rev().collect();

        let to = stacks.get_mut(m.2).unwrap();
        *to = top + to;
    }

    stacks
        .into_iter()
        .filter_map(|stack| stack.chars().next())
        .collect()
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> String {
    let (mut stacks, moves) = parse(input);

    for m in moves {
        let from = stacks.get_mut(m.1).unwrap();
        let top: String = from.drain(..m.0).collect();

        let to = stacks.get_mut(m.2).unwrap();
        *to = top + to;
    }

    stacks
        .into_iter()
        .filter_map(|stack| stack.chars().next())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), "CMZ");
    }

    #[test]
    fn part2_sample() {
        assert_eq!(part2(SAMPLE), "MCD");
    }
}
