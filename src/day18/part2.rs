use std::collections::HashSet;

use itertools::Itertools;

use super::shared::*;

impl Point3D {
    pub fn chebyshev_neighbors(&self) -> Vec<Point3D> {
        let mut neighbors: Vec<Point3D> = vec![];
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    if x == 0 && y == 0 && z == 0 {
                        continue;
                    }
                    neighbors.push(Point3D(self.0 + x, self.1 + y, self.2 + z));
                }
            }
        }
        neighbors
    }
}

fn outside_points(points: &HashSet<Point3D>) -> HashSet<Point3D> {
    let mut outside_points = HashSet::new();
    // point furthest in the positive X direction will be on the outside
    let extreme_point = points.iter().max_by_key(|p| p.0.abs()).unwrap();
    let steam_start_point = Point3D(extreme_point.0 + 1, extreme_point.1, extreme_point.2);

    // flood fill from the steam start point
    let mut frontier = vec![steam_start_point];
    while let Some(point) = frontier.pop() {
        // adjacent but not contained in the set
        frontier.extend(
            point
                .manhattan_neighbors()
                .iter()
                .filter(|p| {
                    !points.contains(p)
                        && !outside_points.contains(*p)
                        && p.chebyshev_neighbors().iter().any(|p| points.contains(p))
                })
                .cloned()
                .collect_vec(),
        );
        outside_points.insert(point);
    }
    outside_points
}

fn outside_surface_area(points: HashSet<Point3D>) -> usize {
    let mut surface_area = 0;
    let outside = outside_points(&points);
    for point in points.iter() {
        surface_area += point
            .manhattan_neighbors()
            .iter()
            .filter(|p| outside.contains(p))
            .count();
    }
    surface_area
}

pub fn run(input: &str) -> i32 {
    let points = parse_points(input);
    outside_surface_area(points) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 58i32);
    }
}
