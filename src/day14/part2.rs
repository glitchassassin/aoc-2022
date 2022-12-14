use super::part1::*;

pub fn run(input: &str) -> i32 {
    let mut map = generate_map(input);
    // add floor
    map.push(vec![0; map[0].len()]);
    map.push(vec![1; map[0].len()]);

    let mut counter = 0;
    loop {
        let sand = simulate(&map);
        map[sand.y][sand.x] = 1;
        counter += 1;
        if sand.y == 0 && sand.x == 500 {
            break;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 93i32);
    }
}
