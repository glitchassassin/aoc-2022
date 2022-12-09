/**
 * Given a grid of numbers, a grid square is "visible" if all other squares to the top, left, right, and down are less than it.
 *
 * Given a coordinate, check if it is not visible.
 */
pub fn check_visible(grid: &[Vec<usize>], x: usize, y: usize) -> bool {
    let num = grid[y][x];
    if grid[0..y].iter().all(|row| row[x] < num) {
        return true;
    }
    if grid[y][0..x].iter().all(|square| square < &num) {
        return true;
    }
    if grid[y + 1..].iter().all(|row| row[x] < num) {
        return true;
    }
    if grid[y][x + 1..].iter().all(|square| square < &num) {
        return true;
    }
    false
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

    let mut visible = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {
            if check_visible(&grid, x, y) {
                visible += 1;
            }
        }
    }
    visible
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 21usize);
    }
}
