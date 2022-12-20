use std::env;

use aoc22::{d1, d10, d11, d12, d13, d14, d2, d3, d4, d5, d6, d7, d8, d9, util::get_input_for_day};

enum Sol {
    U32(fn(&[String]) -> u32),
    I32(fn(&[String]) -> i32),
    U64(fn(&[String]) -> u64),
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
        Sol::U32(d7::part1),
        Sol::U32(d8::part1),
        Sol::U32(d9::part1),
        Sol::I32(d10::part1),
        Sol::U64(d11::part1),
        Sol::U32(d12::part1),
        Sol::U32(d13::part1),
        Sol::U32(d14::part1),
    ];
    let part2 = [
        Sol::U32(d1::part2),
        Sol::U32(d2::part2),
        Sol::U32(d3::part2),
        Sol::U32(d4::part2),
        Sol::STR(d5::part2),
        Sol::U32(d6::part2),
        Sol::U32(d7::part2),
        Sol::U32(d8::part2),
        Sol::U32(d9::part2),
        Sol::STR(d10::part2),
        Sol::U64(d11::part2),
        Sol::U32(d12::part2),
        Sol::U32(d13::part2),
        Sol::U32(d14::part2),
    ];

    let day: usize = env::args()
        .skip(1)
        .take(1)
        .map(|s| s.parse().unwrap())
        .next()
        .unwrap();

    let data = get_input_for_day(day.try_into().unwrap());

    println!("Day {}\n", day);
    if day <= part1.len() {
        let sol = match part1[day - 1] {
            Sol::U32(sol) => sol(&data).to_string(),
            Sol::U64(sol) => sol(&data).to_string(),
            Sol::I32(sol) => sol(&data).to_string(),
            Sol::STR(sol) => sol(&data),
        };
        println!("Part 1:\n{}\n", sol);
    }
    if day <= part2.len() {
        let sol = match part2[day - 1] {
            Sol::U32(sol) => sol(&data).to_string(),
            Sol::U64(sol) => sol(&data).to_string(),
            Sol::I32(sol) => sol(&data).to_string(),
            Sol::STR(sol) => sol(&data),
        };
        println!("Part 2:\n{}\n", sol);
    }
}
