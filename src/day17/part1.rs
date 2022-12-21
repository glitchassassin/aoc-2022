use itertools::Itertools;

use super::shared::*;

pub fn run(input: &str) -> i64 {
    let mut state = GameState::new();
    let mut jets = input
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("invalid input"),
        })
        .collect_vec();
    for _ in 0..2022 {
        step_game_state_until_rock_lands(&mut state, &mut jets);
    }

    state.tower_size()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 3068i64);
    }
}
