use std::cmp::Ordering;

use itertools::{EitherOrBoth::*, Itertools};
use serde_json::{json, Value};

pub fn compare(v1: &Value, v2: &Value) -> Ordering {
    if v1.is_array() && v2.is_array() {
        let v1 = v1.as_array().unwrap();
        let v2 = v2.as_array().unwrap();
        for pair in v1.iter().zip_longest(v2.iter()) {
            match pair {
                Both(v1, v2) => match compare(v1, v2) {
                    Ordering::Equal => continue,
                    ord => return ord,
                },
                Left(_) => return Ordering::Greater,
                Right(_) => return Ordering::Less,
            }
        }
        Ordering::Equal
    } else if v1.is_number() && v2.is_number() {
        v1.as_f64()
            .unwrap()
            .partial_cmp(&v2.as_f64().unwrap())
            .unwrap()
    } else if v1.is_array() && v2.is_number() {
        compare(v1, &json!([v2]))
    } else if v1.is_number() && v2.is_array() {
        compare(&json!([v1]), v2)
    } else {
        panic!("Invalid JSON");
    }
}

pub fn run(input: &str) -> i32 {
    // parse lines as JSON using serde
    let sets = input
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect::<Vec<Value>>();

    let mut sum_of_indices = 0;
    for (index, set) in sets.windows(2).step_by(2).enumerate() {
        if compare(&set[0], &set[1]) == Ordering::Less {
            sum_of_indices += (index + 1) as i32;
        }
    }
    sum_of_indices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 13i32);
    }
}
