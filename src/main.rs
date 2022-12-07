use std::env;

use aoc22::{d1, d2, d3, d4, d5, d6, util::get_input_for_day};

enum Sol {
    U32(fn(&[String]) -> u32),
    STR(fn(&[String]) -> String),
}

fn main() {
    dotenvy::dotenv().unwrap();

    let part1 = [
        Sol::U32(d1::part1),
        Sol::U32(d2::part1),
        Sol::U32(d3::part1),
        Sol::U32(d4::part1),
        Sol::STR(d5::part1),
        Sol::U32(d6::part1),
    ];
    let part2 = [
        Sol::U32(d1::part2),
        Sol::U32(d2::part2),
        Sol::U32(d3::part2),
        Sol::U32(d4::part2),
        Sol::STR(d5::part2),
        Sol::U32(d6::part2),
    ];

    let day: usize = env::args()
        .skip(1)
        .take(1)
        .map(|s| s.parse().unwrap())
        .next()
        .unwrap();

    let data = get_input_for_day(day.try_into().unwrap());

    println!("Day {}", day);
    if day <= part1.len() {
        let sol = match part1[day - 1] {
            Sol::U32(sol) => sol(&data).to_string(),
            Sol::STR(sol) => sol(&data),
        };
        println!("\tPart 1: {}", sol);
    }
    if day <= part2.len() {
        let sol = match part2[day - 1] {
            Sol::U32(sol) => sol(&data).to_string(),
            Sol::STR(sol) => sol(&data),
        };
        println!("\tPart 2: {}", sol);
    }
}
