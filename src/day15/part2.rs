use std::ops::Range;

use itertools::Itertools;

use super::part1::*;

fn find_empty_space(sensor_range: &[(&Point, i32)], y: i32, max_x: i32) -> Option<Point> {
    // values in range per sensor for a given row
    let ranges = sensor_range
        .iter()
        .filter(|(p1, range)| (p1.y - y).abs() <= *range)
        .map(|(p1, range)| {
            let r1 = p1.x - (range - (p1.y - y).abs());
            let r2 = p1.x + (range - (p1.y - y).abs()) + 1;
            Range {
                start: r1.min(r2),
                end: r1.max(r2),
            }
        })
        .sorted_by_key(|r| r.start)
        .collect::<Vec<_>>();

    // combine overlapping ranges
    let mut combined_range = ranges[0].clone();
    for range in ranges.iter().skip(1) {
        if range.start <= combined_range.end {
            combined_range.end = combined_range.end.max(range.end);
        } else {
            if combined_range.end > 0 && combined_range.end <= max_x {
                return Some(Point {
                    x: combined_range.end,
                    y,
                });
            }
            combined_range = range.clone();
        }
    }
    None
}

pub fn run(input: &str, max_x: i32, max_y: i32) -> i64 {
    let sensors_beacons = parse_sensors(input);
    let sensors = sensors_beacons
        .iter()
        .map(|(p1, p2)| (p1, manhattan_distance(p1, p2)))
        .collect::<Vec<_>>();

    for y in 0..max_y {
        if let Some(point) = find_empty_space(&sensors, y, max_x) {
            return (point.x as i64) * 4000000 + (point.y as i64);
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo, 20, 20), 56000011i64);
    }
}
