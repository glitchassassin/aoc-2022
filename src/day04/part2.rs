use itertools::Itertools;

pub fn run(input: &str) -> usize {
    let mut score: usize = 0;

    for line in input.split('\n') {
        let (pair1, pair2) = line
            .split(',')
            .map(|pair| 
                pair
                    .split('-')
                    .map(|n| 
                        n.parse::<usize>().unwrap()
                    )
                    .collect::<Vec<usize>>()
            )
            .next_tuple()
            .unwrap();
        if  pair2.iter().any(|endpoint| (pair1[0]..=pair1[1]).contains(endpoint)) || 
            pair1.iter().any(|endpoint| (pair2[0]..=pair2[1]).contains(endpoint)) {
            score += 1;
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
        assert_eq!(run(demo), 4usize);
    }
}