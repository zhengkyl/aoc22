use std::{
    fs::File,
    io::{Read, Write},
};

pub fn get_input_for_day(day: u32) -> Vec<String> {
    let f = File::open(format!("inputs/d{}.txt", day));

    let mut contents = String::new();

    if let Ok(mut f) = f {
        f.read_to_string(&mut contents).unwrap();
    } else if let Err(_) = f {
        let client = reqwest::blocking::Client::new();
        contents = client
            .get(format!("https://adventofcode.com/2022/day/{}/input", day))
            .header(
                "Cookie",
                format!("session={}", std::env::var("SESSION").unwrap()),
            )
            .send()
            .unwrap()
            .text()
            .unwrap();

        File::create(format!("inputs/d{}.txt", day))
            .unwrap()
            .write_all(contents.as_bytes())
            .unwrap();
    }

    // Remove trailing newline so result doesn't have empty string
    contents.trim().split("\n").map(|s| s.into()).collect()
}
