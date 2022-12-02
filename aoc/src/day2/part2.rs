pub fn run(input: &str) -> i32 {
    let mut score: i32 = 0;

    for line in input.split('\n') {
        score += match line {
            "A X" => 3 + 0, // Lose  Rock - Scissors
            "B X" => 1 + 0, //       Paper - Rock
            "C X" => 2 + 0, //       Scissors - Paper
            "A Y" => 1 + 3, // Tie   Rock - Rock
            "B Y" => 2 + 3, //       Paper - Paper
            "C Y" => 3 + 3, //       Scissors - Scissors
            "A Z" => 2 + 6, // Win   Rock - Paper
            "B Z" => 3 + 6, //       Paper - Scissors
            "C Z" => 1 + 6, //       Scissors - Rock
            &_ => panic!("Unknown option encountered")
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
        assert_eq!(run(demo), 12i32);
    }
}