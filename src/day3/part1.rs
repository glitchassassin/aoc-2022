use std::collections::HashSet;

pub fn run(input: &str) -> usize {
    let priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut score: usize = 0;

    for line in input.split('\n') {
        let section_length = line.len() / 2;
        let first_half: HashSet<char> = line[0 .. section_length].chars().into_iter().collect();
        let second_half: HashSet<char> = line[section_length .. line.len()].chars().into_iter().collect();
        score += first_half.intersection(&second_half)
            .map(|item| match priorities.find(|c| &c == item) {
                Some(val) => val + 1,
                None => 0
            })
            .sum::<usize>();
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