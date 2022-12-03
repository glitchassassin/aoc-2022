use std::collections::HashSet;

pub fn run(input: &str) -> usize {
    let priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score: usize = 0;

    for line in input.split('\n') {
        let section_length = line.len() / 2;
        let items: HashSet<char> = line[0 .. section_length].chars().into_iter().collect();
        for item in items {
            let in_second_section = line[section_length .. line.len()].find(item);
            if in_second_section.is_none() {
                continue;
            }
            let priority = priorities.find(item);
            score += match priority {
                Some(val) => val + 1,
                None => 0
            }

        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 157usize);
    }
}