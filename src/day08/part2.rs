use std::vec;

use itertools::Itertools;

/**
 * Given a grid of numbers, sum the number of squares to the top, left, right, and down that are less than it. Multiply these numbers together to return the scenic score.
 */
pub fn scenic_score(grid: &[Vec<usize>], x: usize, y: usize) -> usize {
    let num = grid[y][x];
    let (top, _) = grid[0..y]
        .iter()
        .rev()
        .find_position(|row| row[x] >= num)
        .unwrap_or((y, &vec![]));
    let (bottom, _) = grid[y + 1..]
        .iter()
        .find_position(|row| row[x] >= num)
        .unwrap_or((grid.len() - y, &vec![]));
    let (left, _) = grid[y][0..x]
        .iter()
        .rev()
        .find_position(|square| square >= &&num)
        .unwrap_or((x, &0));
    let (right, _) = grid[y][x + 1..]
        .iter()
        .find_position(|square| square >= &&num)
        .unwrap_or((grid[y].len() - x, &0));
    // min of top or y
    (top + 1).min(y)
        * (bottom + 1).min(grid.len() - y - 1)
        * (left + 1).min(x)
        * (right + 1).min(grid[y].len() - x - 1)
}

pub fn run(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|num| num.to_digit(10).unwrap().try_into().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut max_scenic_score = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            max_scenic_score = max_scenic_score.max(scenic_score(&grid, x, y));
        }
    }
    max_scenic_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 8usize);
    }
}
