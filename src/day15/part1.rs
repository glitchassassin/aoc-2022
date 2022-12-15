use std::ops::Range;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn parse_sensors(input: &str) -> Vec<(Point, Point)> {
    let mut points = vec![];
    for line in input.lines() {
        let re = Regex::new(r"Sensor at x=(?P<x1>\-?\d+), y=(?P<y1>\-?\d+): closest beacon is at x=(?P<x2>\-?\d+), y=(?P<y2>\-?\d+)").unwrap();
        let caps = re.captures(line).unwrap();
        points.push((
            Point {
                x: caps["x1"].parse().unwrap(),
                y: caps["y1"].parse().unwrap(),
            },
            Point {
                x: caps["x2"].parse().unwrap(),
                y: caps["y2"].parse().unwrap(),
            },
        ))
    }
    points
}

/**
 * Calculate Manhattan distance
 */
pub fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

pub fn run(input: &str, row: i32) -> i32 {
    let sensors = parse_sensors(input);
    let sensor_range = sensors
        .iter()
        .map(|(p1, p2)| (p1, manhattan_distance(p1, p2)))
        .collect::<Vec<_>>();

    // values in range per sensor for a given row
    let ranges = sensor_range
        .iter()
        .map(|(p1, range)| {
            let r1 = p1.x - (range - (p1.y - row).abs());
            let r2 = p1.x + (range - (p1.y - row).abs()) + 1;
            Range {
                start: r1.min(r2),
                end: r1.max(r2),
            }
        })
        .sorted_by_key(|r| r.start)
        .collect::<Vec<_>>();

    // combine overlapping ranges
    let mut no_beacons = 0;
    let mut combined_range = ranges[0].clone();
    for range in ranges.iter().skip(1) {
        if range.start <= combined_range.end {
            combined_range.end = combined_range.end.max(range.end);
        } else {
            no_beacons += combined_range.end - combined_range.start;
            combined_range = range.clone();
        }
    }
    no_beacons += combined_range.count() as i32;

    // subtract spaces that actually have beacons
    no_beacons -= sensors
        .iter()
        .map(|(_, beacon)| (beacon.x, beacon.y))
        .filter(|(_, y)| y == &row)
        .unique()
        .count() as i32;

    no_beacons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo, 10), 26i32);
    }
}
