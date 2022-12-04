use reqwest::Error;

pub fn get_input_for_day(day: u32) -> Result<Vec<String>, Error> {
    let client = reqwest::blocking::Client::new();
    let full_res = client
        .get(format!("https://adventofcode.com/2022/day/{}/input", day))
        .header(
            "Cookie",
            format!("session={}", std::env::var("SESSION").unwrap()),
        )
        .send()?
        .text()?;

    Ok(full_res.split("\n").map(|s| s.into()).collect())
}
