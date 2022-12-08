use std::collections::HashMap;

fn get_size_map<S: AsRef<str>>(input: &[S]) -> HashMap<String, u32> {
    let mut size_map = HashMap::new();

    let mut cwd = String::new();

    for line in input {
        let mut file_iter = line.as_ref().split(' ');
        if line.as_ref().starts_with("$ cd") {
            let dir = file_iter.nth(2).unwrap();
            match dir {
                "/" => cwd = "/".into(),
                ".." => cwd = cwd.rsplit_once('/').unwrap().0.into(),
                _ => {
                    cwd += "/";
                    cwd += dir;
                }
            }
        } else if line.as_ref().starts_with("$ ls") {
            // Do nothing?
        } else if line.as_ref().starts_with('d') {
            // Do nothing?
        } else {
            let size: u32 = file_iter.next().unwrap().parse().unwrap();

            let mut temp_cwd = cwd.to_owned();

            while !temp_cwd.is_empty() {
                size_map
                    .entry(temp_cwd.to_owned())
                    .and_modify(|e| *e += size)
                    .or_insert(size);

                temp_cwd.drain(temp_cwd.rfind('/').unwrap_or(0)..);
            }
        }
    }
    size_map
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> u32 {
    get_size_map(input)
        .into_values()
        .filter(|v| v <= &100000)
        .sum()
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u32 {
    let mut map = get_size_map(input);
    let required = 30000000 - (70000000 - map.remove("/").unwrap());

    map.into_values().filter(|v| v >= &required).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "$ cd /",
        "$ ls",
        "dir a",
        "14848514 b.txt",
        "8504156 c.dat",
        "dir d",
        "$ cd a",
        "$ ls",
        "dir e",
        "29116 f",
        "2557 g",
        "62596 h.lst",
        "$ cd e",
        "$ ls",
        "584 i",
        "$ cd ..",
        "$ cd ..",
        "$ cd d",
        "$ ls",
        "4060174 j",
        "8033020 d.log",
        "5626152 d.ext",
        "7214296 k",
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(part1(SAMPLE), 95437);
    }

    // #[test]
    // fn part2_sample() {
    //     assert_eq!(part2(SAMPLE), 19);
    // }
}
