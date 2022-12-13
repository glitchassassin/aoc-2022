use itertools::Itertools;
use serde_json::{json, Value};
use std::cmp::Ordering;

use super::part1::*;

pub fn run(input: &str) -> i32 {
    let divider1 = json![[[2]]];
    let divider2 = json![[[6]]];
    // parse lines as JSON using serde
    let sets = input
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .chain([divider1.clone(), divider2.clone()])
        .sorted_by(compare)
        .collect::<Vec<Value>>();

    let (pos1, _) = sets
        .iter()
        .find_position(|&x| compare(x, &divider1) == Ordering::Equal)
        .unwrap();
    let (pos2, _) = sets
        .iter()
        .find_position(|&x| compare(x, &divider2) == Ordering::Equal)
        .unwrap();

    ((pos1 + 1) * (pos2 + 1)) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 140i32);
    }
}
