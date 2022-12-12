use itertools::Itertools;
use std::collections::HashSet;

use super::shared::*;

/**
 * Given a series of commands such as "R 4", map the direction (R = Direction.Right) and the distance (4) and iteratively apply the simulate_step function.
 */
pub fn run(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);
    let mut visited = HashSet::new();
    for line in input.lines() {
        let (dir, distance) = line.split_whitespace().collect_tuple().unwrap();
        for _ in 0..distance.parse::<usize>().unwrap() {
            (head, tail) = simulate_step(
                head,
                tail,
                match dir {
                    "R" => Direction::Right,
                    "L" => Direction::Left,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    _ => panic!("Invalid direction"),
                },
            );
            visited.insert(tail);
        }
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 13usize);
    }
}
