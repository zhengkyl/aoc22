use std::{collections::HashSet, env};

use aoc22::{
    d1::{part1, part2},
    util::get_input_for_day,
};

fn main() -> Result<(), reqwest::Error> {
    dotenvy::dotenv().unwrap();

    let args = env::args()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect::<HashSet<u32>>();

    if args.contains(&1) {
        let data = get_input_for_day(1)?;

        let ans1 = part1(&data);
        let ans2 = part2(&data);

        println!("Day 1");
        println!("\tPart 1: {}", ans1);
        println!("\tPart 2: {}", ans2);
    }

    Ok(())
}
