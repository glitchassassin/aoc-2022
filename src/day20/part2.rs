use itertools::Itertools;

use super::shared::*;

const DECRYPTION_KEY: i64 = 811589153;

/**
 * For each number in `encrypted`, shift the number based on its value. For example,
 * if the number is 3, shift the number 3 places to the right. If the number is -2,
 * shift it two places to the left. Wrap the index around if it goes out of bounds.
 *
 * Surrounding elements should be shifted to make room.
 */
pub fn mix(encrypted: Vec<(usize, i64)>) -> Vec<(usize, i64)> {
    let mut mixed = encrypted.clone();
    let rotate_len = (encrypted.len() - 1) as i64;
    for original_index in 0..encrypted.len() {
        let (current_index, (_, num)) = mixed
            .iter()
            .find_position(|p| p.0 == original_index)
            .unwrap();
        let mut new_index = current_index as i64 + num;

        new_index = (new_index + (rotate_len * DECRYPTION_KEY * 2)) % rotate_len;

        // println!("{} -> {}", current_index, new_index);

        let pulled = mixed.remove(current_index);
        mixed.insert(new_index as usize, pulled);
    }
    mixed
}

pub fn coordinates(encrypted: Vec<i64>) -> i64 {
    let start = encrypted.iter().find_position(|p| p == &&0).unwrap().0;
    let numbers = [start + 1000, start + 2000, start + 3000]
        .iter()
        .map(|p| encrypted[p % encrypted.len()])
        .collect::<Vec<_>>();
    println!("{:?}", numbers);
    numbers.iter().sum()
}

pub fn run(input: &str) -> i64 {
    let mut encrypted = parse_input(input)
        .iter()
        .map(|p| (*p as i64) * DECRYPTION_KEY)
        .enumerate()
        .collect_vec();
    for _ in 0..10 {
        encrypted = mix(encrypted);
        println!("{:?}", encrypted);
    }
    coordinates(encrypted.iter().map(|p| p.1).collect_vec())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 1623178306i64);
    }
}
