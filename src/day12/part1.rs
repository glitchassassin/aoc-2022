pub fn parse_map(input: &str) -> (Vec<Vec<i32>>, (i32, i32), (i32, i32)) {
    let mut map: Vec<Vec<i32>> = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        map.push(
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let height = match c {
                        'S' => {
                            start = (x as i32, y as i32);
                            'a'
                        }
                        'E' => {
                            end = (x as i32, y as i32);
                            'z'
                        }
                        c => c,
                    };
                    (height as i32) - ('a' as i32)
                })
                .collect(),
        );
    }

    (map, start, end)
}

/**
 * Get the neighbors of a node on a map.
 */
fn get_neighbors(map: &Vec<Vec<i32>>, node: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors = vec![
        (node.0, node.1 - 1),
        (node.0, node.1 + 1),
        (node.0 - 1, node.1),
        (node.0 + 1, node.1),
    ];
    neighbors.retain(|n| {
        n.0 >= 0
            && n.1 >= 0
            && n.0 < map[0].len() as i32
            && n.1 < map.len() as i32
            && map[n.1 as usize][n.0 as usize] <= map[node.1 as usize][node.0 as usize] + 1
    });
    neighbors
}

/**
 * Find the shortest path between two points on a map. Each step can only ascend by 1, but can descend by any amount.
 *
 * Return the length of the shortest path.
 */
fn breadth_first_search(map: Vec<Vec<i32>>, start: (i32, i32), end: (i32, i32)) -> i32 {
    let mut queue = vec![(start, 0)];
    let mut visited = vec![vec![false; map[0].len()]; map.len()];

    while !queue.is_empty() {
        let (node, distance) = queue.remove(0);
        if node == end {
            return distance;
        }
        if visited[node.1 as usize][node.0 as usize] {
            continue;
        }
        visited[node.1 as usize][node.0 as usize] = true;
        for neighbor in get_neighbors(&map, node) {
            queue.push((neighbor, distance + 1));
        }
    }
    0
}

pub fn run(input: &str) -> i32 {
    let (map, start, end) = parse_map(input);

    breadth_first_search(map, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 31i32);
    }
}
