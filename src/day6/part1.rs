use std::collections::HashSet;

pub fn run(input: &str) -> usize {
    let size = 4;
    for (index, window) in input.chars().collect::<Vec<char>>().windows(size).enumerate() {
        if window.iter().collect::<HashSet<&char>>().len() == size {
            return index + size;
        }
    }
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 7usize);
    }
}