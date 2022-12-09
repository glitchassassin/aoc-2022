use std::cmp::Ordering;

/**
 * Return the Chebyshev distance between two points.
 */
fn chebyshev_distance((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> i32 {
    (x1 - x2).abs().max((y1 - y2).abs())
}

// Direction enum
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/**
 * Given the position of head H and tail T, and a direction D, move the head in that direction and move the tail if its chebyshev distance from the head is greater than 1.
 */
pub fn simulate_step(
    (h_x, h_y): (i32, i32),
    (t_x, t_y): (i32, i32),
    d: Direction,
) -> ((i32, i32), (i32, i32)) {
    let (h_x, h_y) = match d {
        Direction::Up => (h_x, h_y - 1),
        Direction::Down => (h_x, h_y + 1),
        Direction::Left => (h_x - 1, h_y),
        Direction::Right => (h_x + 1, h_y),
    };
    follow_the_leader((h_x, h_y), (t_x, t_y))
}

/**
 * Given the position of head H and tail T, move the tail if its chebyshev distance from the head is greater than 1.
 */
pub fn follow_the_leader(
    (h_x, h_y): (i32, i32),
    (t_x, t_y): (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    if chebyshev_distance((h_x, h_y), (t_x, t_y)) <= 1 {
        return ((h_x, h_y), (t_x, t_y));
    }
    let t_x = match h_x.cmp(&t_x) {
        Ordering::Greater => t_x + 1,
        Ordering::Less => t_x - 1,
        Ordering::Equal => t_x,
    };
    let t_y = match h_y.cmp(&t_y) {
        Ordering::Greater => t_y + 1,
        Ordering::Less => t_y - 1,
        Ordering::Equal => t_y,
    };
    ((h_x, h_y), (t_x, t_y))
}

/**
 * Given a rope of points and a direction D, move the first point in the rope
 * in that direction and move each successive point if its chebyshev distance
 * from the preceeding is greater than 1.
 */
pub fn simulate_rope(rope: Vec<(i32, i32)>, d: Direction) -> Vec<(i32, i32)> {
    let mut new_rope = vec![];
    let (h_x, h_y) = rope[0];
    let (h_x, h_y) = match d {
        Direction::Up => (h_x, h_y - 1),
        Direction::Down => (h_x, h_y + 1),
        Direction::Left => (h_x - 1, h_y),
        Direction::Right => (h_x + 1, h_y),
    };
    new_rope.push((h_x, h_y));

    let mut head = (h_x, h_y);
    rope.iter().skip(1).for_each(|tail| {
        let (_, new_tail) = follow_the_leader(head, *tail);
        new_rope.push(new_tail);
        head = new_tail;
    });
    new_rope
}
