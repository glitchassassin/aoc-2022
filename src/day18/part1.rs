use std::collections::HashSet;

use super::shared::*;

fn surface_area(points: HashSet<Point3D>) -> usize {
    let mut surface_area = 0;
    for point in points.iter() {
        surface_area += point
            .manhattan_neighbors()
            .iter()
            .filter(|p| !points.contains(p))
            .count();
    }
    surface_area
}

pub fn run(input: &str) -> i32 {
    let points = parse_points(input);
    surface_area(points) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 64i32);
    }
}
