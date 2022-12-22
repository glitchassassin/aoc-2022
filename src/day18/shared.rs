use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Point3D(pub i32, pub i32, pub i32);

impl Point3D {
    pub fn manhattan_neighbors(&self) -> Vec<Point3D> {
        vec![
            Point3D(self.0 + 1, self.1, self.2),
            Point3D(self.0 - 1, self.1, self.2),
            Point3D(self.0, self.1 + 1, self.2),
            Point3D(self.0, self.1 - 1, self.2),
            Point3D(self.0, self.1, self.2 + 1),
            Point3D(self.0, self.1, self.2 - 1),
        ]
    }
}

pub fn parse_points(input: &str) -> HashSet<Point3D> {
    let mut points = HashSet::new();
    for line in input.lines() {
        let mut parts = line.split(',').map(|s| s.parse::<i32>().unwrap());
        let (x, y, z) = parts.next_tuple().unwrap();
        points.insert(Point3D(x, y, z));
    }
    points
}
