use itertools::Itertools;

use super::shared::*;

/**
 * For each number in `encrypted`, shift the number based on its value. For example,
 * if the number is 3, shift the number 3 places to the right. If the number is -2,
 * shift it two places to the left. Wrap the index around if it goes out of bounds.
 *
 * Surrounding elements should be shifted to make room.
 */
pub fn mix(encrypted: Vec<i32>) -> Vec<i32> {
    let mut mixed = encrypted.iter().enumerate().collect_vec();
    let rotate_len = (encrypted.len() - 1) as i32;
    for (original_index, num) in encrypted.iter().enumerate() {
        if num == &0 {
            continue;
        }
        let current_index = mixed
            .iter()
            .find_position(|p| p.0 == original_index)
            .unwrap()
            .0;
        let mut new_index = current_index as i32 + num;

        new_index = (new_index + rotate_len * 2) % rotate_len;

        let pulled = mixed.remove(current_index);
        mixed.insert(new_index as usize, pulled);
    }
    mixed.iter().map(|(_, p)| p).cloned().cloned().collect_vec()
}

pub fn coordinates(encrypted: Vec<i32>) -> i32 {
    let start = encrypted.iter().find_position(|p| p == &&0).unwrap().0;
    let numbers = [start + 1000, start + 2000, start + 3000]
        .iter()
        .map(|p| encrypted[p % encrypted.len()])
        .collect::<Vec<_>>();
    println!("{:?}", numbers);
    numbers.iter().sum()
}

pub fn run(input: &str) -> i32 {
    let encrypted = parse_input(input);
    let mixed = mix(encrypted);
    coordinates(mixed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 3i32);
    }
}
