use itertools::Itertools;

#[derive(Debug)]
pub struct SandGrain {
    pub x: usize,
    pub y: usize,
}

#[allow(clippy::needless_range_loop)]
pub fn generate_map(input: &str) -> Vec<Vec<u8>> {
    let walls = input.lines().map(|line| {
        line.split(" -> ")
            .map(|s| {
                s.split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .next_tuple::<(_, _)>()
                    .unwrap()
            })
            .collect::<Vec<(usize, usize)>>()
    });

    // get bounds of map
    let (max_x, max_y) = walls
        .clone()
        .flatten()
        .fold((0, 0), |(max_x, max_y), (x, y)| {
            (
                if x > max_x { x } else { max_x },
                if y > max_y { y } else { max_y },
            )
        });

    let mut map = vec![vec![0; max_x.max(500 * 2) + 1]; max_y + 1];

    // add walls to map
    walls.for_each(|wall| {
        wall.iter()
            .tuple_windows()
            .for_each(|((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    for y in *y1.min(y2)..=*y1.max(y2) {
                        map[y][*x1] = 1;
                    }
                } else {
                    for x in *x1.min(x2)..=*x1.max(x2) {
                        map[*y1][x] = 1;
                    }
                }
            })
    });

    map
}

#[allow(dead_code)]
fn render_map(map: &[Vec<u8>]) {
    map.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            print!(
                "{}",
                match cell {
                    0 => '.',
                    1 => '#',
                    _ => ' ',
                }
            );
        });
        println!();
    });
}

/**
 * Simulate a grain of sand falling from 500, 0. The sand will move straight down, if it can; otherwise, down and left; otherwise, down and right; otherwise, it will stop.
 *
 * If the grain of sand reaches the bottom of the map, it will stop.
 *
 * Return the final position of the grain of sand.
 */
pub fn simulate(map: &[Vec<u8>]) -> SandGrain {
    let mut sand = SandGrain { x: 500, y: 0 };
    loop {
        // if we're at the bottom of the map, stop
        if sand.y == map.len() - 1 {
            break;
        }

        // if we can move straight down, do so
        if map[sand.y + 1][sand.x] == 0 {
            sand.y += 1;
            continue;
        }

        // if we can move down and left, do so
        if map[sand.y + 1][sand.x - 1] == 0 {
            sand.y += 1;
            sand.x -= 1;
            continue;
        }

        // if we can move down and right, do so
        if map[sand.y + 1][sand.x + 1] == 0 {
            sand.y += 1;
            sand.x += 1;
            continue;
        }

        // if we can't move anywhere, stop
        break;
    }
    sand
}

pub fn run(input: &str) -> i32 {
    let mut map = generate_map(input);
    // render_map(&map);
    let mut counter = 0;
    loop {
        let sand = simulate(&map);
        if sand.y == map.len() - 1 {
            break;
        }
        map[sand.y][sand.x] = 1;
        counter += 1;
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 24i32);
    }
}
