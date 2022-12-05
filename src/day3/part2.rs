use std::collections::HashSet;
use itertools::Itertools;

pub fn run(input: &str) -> usize {
    let priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score: usize = 0;

    let iter = input.split('\n').map(|i| i.chars().collect::<HashSet<char>>());
    
    for (elf1, elf2, elf3) in iter.tuples() {
        let reduced = [elf2, elf3]
            .iter()
            .fold(elf1, |init, set| init.intersection(set).copied().collect());
        let badge = reduced.iter().last();
        if badge.is_none() {
            break;
        }
        let priority = priorities.find(|c| &c == badge.unwrap());
                    
        score += match priority {
            Some(val) => val + 1,
            None => 0
        };
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 70usize);
    }
}