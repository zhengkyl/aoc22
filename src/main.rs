use std::env;

use aoc22::{d1, d2, d3, util::get_input_for_day};

fn main() -> Result<(), reqwest::Error> {
    dotenvy::dotenv().unwrap();

    let part1 = [d1::part1, d2::part1, d3::part1];
    let part2 = [d1::part2, d2::part2, d3::part2];

    let day: usize = env::args()
        .skip(1)
        .take(1)
        .map(|s| s.parse().unwrap())
        .next()
        .unwrap();

    let data = get_input_for_day(day.try_into().unwrap())?;

    println!("Day {}", day);
    if day <= part1.len() {
        println!("\tPart 1: {}", part1[day - 1](&data));
    }
    if day <= part2.len() {
        println!("\tPart 2: {}", part2[day - 1](&data));
    }

    Ok(())
}
