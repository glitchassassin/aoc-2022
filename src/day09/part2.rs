use itertools::Itertools;
use std::collections::HashSet;

use super::shared::*;

/**
 * Given a series of commands such as "R 4", map the direction (R = Direction.Right) and the distance (4) and iteratively apply the simulate_step function.
 */
pub fn run(input: &str) -> usize {
    let mut rope = vec![];
    rope.resize(10, (0, 0));
    let mut visited = HashSet::new();
    for line in input.lines() {
        let (dir, distance) = line.split_whitespace().collect_tuple().unwrap();
        for _ in 0..distance.parse::<usize>().unwrap() {
            rope = simulate_rope(
                rope,
                match dir {
                    "R" => Direction::Right,
                    "L" => Direction::Left,
                    "U" => Direction::Up,
                    "D" => Direction::Down,
                    _ => panic!("Invalid direction"),
                },
            );
            visited.insert(rope[9]);
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
        assert_eq!(run(demo), 1usize);
        let demo2 = include_str!("inputs/sample2.txt");
        assert_eq!(run(demo2), 36usize);
    }
}
