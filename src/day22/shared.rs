use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
pub enum Instructions {
    Forward(i32),
    Left(),
    Right(),
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub struct PlayerState {
    pub x: usize,
    pub y: usize,
    pub dir: Direction,
}
impl PlayerState {
    pub fn initialize(map: &Map) -> PlayerState {
        let (x, y) = starting_point(map);
        PlayerState {
            x,
            y,
            dir: Direction::Right,
        }
    }
    pub fn rotate(&mut self, inst: &Instructions) {
        match inst {
            Instructions::Left() => {
                self.dir = match self.dir {
                    Direction::Up => Direction::Left,
                    Direction::Down => Direction::Right,
                    Direction::Left => Direction::Down,
                    Direction::Right => Direction::Up,
                }
            }
            Instructions::Right() => {
                self.dir = match self.dir {
                    Direction::Up => Direction::Right,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                }
            }
            _ => panic!("invalid instruction"),
        }
    }
    pub fn move_forward(&mut self, inst: &Instructions, map: &Map) {
        match inst {
            Instructions::Forward(n) => match self.dir {
                Direction::Up => {
                    for _ in 0..*n {
                        let mut next_y = if self.y > 0 {
                            self.y - 1
                        } else {
                            map.len() - 1
                        };
                        if map[next_y][self.x] == ' ' {
                            // wrap to start
                            let (origin, slice) = slice_x(map, self.x);
                            next_y = origin + slice.len() - 1;
                        }
                        let next_tile = map[next_y][self.x];
                        if next_tile == '#' {
                            return;
                        } else if next_tile == '.' {
                            self.y = next_y;
                            continue;
                        }
                    }
                }
                Direction::Down => {
                    for _ in 0..*n {
                        let mut next_y = self.y + 1;
                        if next_y >= map.len() || map[next_y][self.x] == ' ' {
                            // wrap to start
                            let (origin, _) = slice_x(map, self.x);
                            next_y = origin;
                        }
                        let next_tile = map[next_y][self.x];
                        if next_tile == '#' {
                            return;
                        } else if next_tile == '.' {
                            self.y = next_y;
                            continue;
                        }
                    }
                }
                Direction::Left => {
                    for _ in 0..*n {
                        let mut next_x = if self.x > 0 {
                            self.x - 1
                        } else {
                            map[0].len() - 1
                        };
                        if map[self.y][next_x] == ' ' {
                            // wrap to start
                            let (origin, slice) = slice_y(map, self.y);
                            next_x = origin + slice.len() - 1;
                        }
                        let next_tile = map[self.y][next_x];
                        if next_tile == '#' {
                            return;
                        } else if next_tile == '.' {
                            self.x = next_x;
                            continue;
                        }
                    }
                }
                Direction::Right => {
                    for _ in 0..*n {
                        let mut next_x = self.x + 1;
                        if next_x >= map[0].len() || map[self.y][next_x] == ' ' {
                            // wrap to start
                            let (origin, _) = slice_y(map, self.y);
                            next_x = origin;
                        }
                        let next_tile = map[self.y][next_x];
                        if next_tile == '#' {
                            return;
                        } else if next_tile == '.' {
                            self.x = next_x;
                            continue;
                        }
                    }
                }
            },
            _ => panic!("invalid instruction"),
        }
    }
    pub fn password(&self) -> usize {
        1000 * (self.y + 1) + 4 * (self.x + 1) + (self.dir as usize)
    }
    pub fn follow_instruction(&mut self, inst: &Instructions, map: &Map) {
        match inst {
            Instructions::Forward(_) => self.move_forward(inst, map),
            _ => self.rotate(inst),
        }
        // println!("{}\n", render_map(map, self));
    }
}

pub type Map = Vec<Vec<char>>;

pub fn slice_y(map: &Map, y: usize) -> (usize, Vec<char>) {
    let mut slice = vec![];
    let mut origin = None;
    for (i, c) in map[y].iter().enumerate() {
        if c != &' ' {
            slice.push(*c);
            if origin.is_none() {
                origin = Some(i);
            }
        }
    }
    (origin.unwrap(), slice)
}
pub fn slice_x(map: &Map, x: usize) -> (usize, Vec<char>) {
    let mut slice = vec![];
    let mut origin = None;
    for (i, c) in map.iter().map(|row| row[x]).enumerate() {
        if c != ' ' {
            slice.push(c);
            if origin.is_none() {
                origin = Some(i);
            }
        }
    }
    (origin.unwrap(), slice)
}
pub fn max_dist(slice: &Vec<char>, offset: usize) -> Option<usize> {
    let mut rotated = (*slice).clone();
    rotated.rotate_left(offset);
    rotated
        .iter()
        .find_position(|c| **c == '#')
        .map(|p| p.0 - 1)
}

/**
 * first non-empty tile in the top row
 */
pub fn starting_point(map: &Map) -> (usize, usize) {
    let (origin, _) = slice_y(map, 0);
    (origin, 0)
}

/**
 * Load map and path directions
 */
pub fn parse_input(input: &str) -> (Map, Vec<Instructions>) {
    let lines = input.lines();
    let mut map = Vec::new();
    // load map first
    for line in lines.take_while(|line| !line.is_empty()) {
        map.push(line.chars().collect_vec());
    }
    // pad map, if needed
    let max_len = map.iter().map(|row| row.len()).max().unwrap();
    for row in map.iter_mut() {
        while row.len() < max_len {
            row.push(' ');
        }
    }
    // now load path
    let re = Regex::new(r"(?:(R)|(L)|(\d+))").unwrap();
    let caps = re.captures_iter(input.lines().last().unwrap());
    let mut path = Vec::new();
    for cap in caps {
        if cap.get(1).is_some() {
            path.push(Instructions::Right());
        } else if cap.get(2).is_some() {
            path.push(Instructions::Left());
        } else if let Some(dir) = cap.get(3) {
            path.push(Instructions::Forward(dir.as_str().parse().unwrap()));
        }
    }

    (map, path)
}

pub fn render_map(map: &Map, player: &PlayerState) -> String {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, c)| {
                    if y == player.y && x == player.x {
                        match player.dir {
                            Direction::Up => '^',
                            Direction::Down => 'v',
                            Direction::Left => '<',
                            Direction::Right => '>',
                        }
                    } else {
                        *c
                    }
                })
                .collect::<String>()
        })
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_right() {
        let demo = include_str!("inputs/sample.txt");

        let (map, _) = parse_input(demo);

        let mut player = PlayerState {
            x: 8,
            y: 0,
            dir: Direction::Right,
        };

        player.move_forward(&Instructions::Forward(1), &map);
        assert_eq!(player.x, 9);
        assert_eq!(player.y, 0);
        player.move_forward(&Instructions::Forward(10), &map);
        assert_eq!(player.x, 10);
        assert_eq!(player.y, 0);
    }

    #[test]
    fn test_move_left() {
        let demo = include_str!("inputs/sample.txt");

        let (map, _) = parse_input(demo);

        let mut player = PlayerState {
            x: 10,
            y: 0,
            dir: Direction::Left,
        };

        player.move_forward(&Instructions::Forward(1), &map);
        assert_eq!(player.x, 9);
        assert_eq!(player.y, 0);
        player.move_forward(&Instructions::Forward(10), &map);
        assert_eq!(player.x, 8);
        assert_eq!(player.y, 0);
    }

    #[test]
    fn test_move_down() {
        let demo = include_str!("inputs/sample.txt");

        let (map, _) = parse_input(demo);

        let mut player = PlayerState {
            x: 3,
            y: 5,
            dir: Direction::Down,
        };

        player.move_forward(&Instructions::Forward(1), &map);
        assert_eq!(player.x, 3);
        assert_eq!(player.y, 6);
        player.move_forward(&Instructions::Forward(10), &map);
        assert_eq!(player.x, 3);
        assert_eq!(player.y, 7);
    }

    #[test]
    fn test_move_up() {
        let demo = include_str!("inputs/sample.txt");

        let (map, _) = parse_input(demo);

        let mut player = PlayerState {
            x: 2,
            y: 5,
            dir: Direction::Up,
        };

        player.move_forward(&Instructions::Forward(1), &map);
        assert_eq!(player.x, 2);
        assert_eq!(player.y, 4);
        player.move_forward(&Instructions::Forward(10), &map);
        assert_eq!(player.x, 2);
        assert_eq!(player.y, 7);
    }
}
