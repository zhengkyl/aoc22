use std::{cmp::max, collections::HashSet};

use regex::Regex;

fn parse<S: AsRef<str>>(input: &[S]) -> (HashSet<(i32, i32)>, Vec<((i32, i32), i32)>) {
    let re =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();

    let mut beacons = HashSet::new();

    let sensor_dists = input
        .into_iter()
        .map(|line| {
            let x = re.captures(line.as_ref()).unwrap();
            let sensor = (x[1].parse::<i32>().unwrap(), x[2].parse::<i32>().unwrap());
            let beacon = (x[3].parse::<i32>().unwrap(), x[4].parse::<i32>().unwrap());

            beacons.insert(beacon);

            let max_dist = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();

            (sensor, max_dist)
        })
        .collect::<Vec<((i32, i32), i32)>>();

    (beacons, sensor_dists)
}

fn get_ranges_at_row(sensor_dists: &Vec<((i32, i32), i32)>, row: i32) -> Vec<(i32, i32)> {
    let mut ranges = sensor_dists
        .iter()
        .filter_map(|(sensor, max_dist)| {
            let radius = max_dist - (row - sensor.1).abs();

            if radius < 0 {
                None
            } else {
                Some((sensor.0 - radius, sensor.0 + radius))
            }
        })
        .collect::<Vec<(i32, i32)>>();

    ranges.sort();

    let mut collaped_ranges = Vec::new();
    let mut cur_range = ranges[0];

    for i in 0..ranges.len() - 1 {
        let next = ranges[i + 1];

        if next.0 <= cur_range.1 {
            cur_range = (cur_range.0, max(cur_range.1, next.1));
            continue;
        }

        collaped_ranges.push(cur_range);
        cur_range = next;
    }
    collaped_ranges.push(cur_range);

    collaped_ranges
}

fn solve_part1<S: AsRef<str>, const ROW: i32>(input: &[S]) -> i32 {
    let (beacons, sensor_dists) = parse(input);

    let mut count: i32 = 0;

    for beacon in beacons {
        if beacon.1 == ROW {
            count -= 1;
        }
    }

    for range in get_ranges_at_row(&sensor_dists, ROW) {
        count += range.1 - range.0 + 1;
    }

    count
}

pub fn part1<S: AsRef<str>>(input: &[S]) -> i32 {
    solve_part1::<S, 2000000>(input)
}

fn solve_part2<S: AsRef<str>, const MAX: i32>(input: &[S]) -> u64 {
    let (_, sensor_dists) = parse(input);

    for i in 0..=MAX {
        for range in get_ranges_at_row(&sensor_dists, i) {
            if range.0 > 0 {
                return (range.0 - 1) as u64 * 4000000 + i as u64;
            }

            if range.1 < MAX {
                return (range.1 + 1) as u64 * 4000000 + i as u64;
            }
        }
    }

    unreachable!()
}

pub fn part2<S: AsRef<str>>(input: &[S]) -> u64 {
    solve_part2::<S, 4000000>(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[&str] = &[
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15",
        "Sensor at x=9, y=16: closest beacon is at x=10, y=16",
        "Sensor at x=13, y=2: closest beacon is at x=15, y=3",
        "Sensor at x=12, y=14: closest beacon is at x=10, y=16",
        "Sensor at x=10, y=20: closest beacon is at x=10, y=16",
        "Sensor at x=14, y=17: closest beacon is at x=10, y=16",
        "Sensor at x=8, y=7: closest beacon is at x=2, y=10",
        "Sensor at x=2, y=0: closest beacon is at x=2, y=10",
        "Sensor at x=0, y=11: closest beacon is at x=2, y=10",
        "Sensor at x=20, y=14: closest beacon is at x=25, y=17",
        "Sensor at x=17, y=20: closest beacon is at x=21, y=22",
        "Sensor at x=16, y=7: closest beacon is at x=15, y=3",
        "Sensor at x=14, y=3: closest beacon is at x=15, y=3",
        "Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    ];

    #[test]
    fn part1_sample() {
        assert_eq!(solve_part1::<&str, 10>(SAMPLE), 26);
    }

    #[test]
    fn part2_sample() {
        assert_eq!(solve_part2::<&str, 20>(SAMPLE), 56000011);
    }
}
