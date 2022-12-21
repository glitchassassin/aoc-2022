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

    let total_iterations: i64 = 1000000000000;
    let stable_interval = jets.len() as i64;
    // First stable_interval rocks gets the tower to a stable state
    for _ in 0..stable_interval {
        step_game_state_until_rock_lands(&mut state, &mut jets);
    }
    let base_height = state.tower_size();
    // now evaluate the repeating pattern
    let mut repeating_interval = 0;

    let mut last_height = base_height;
    let mut height_diffs = vec![];
    let mut differentiated = false;
    for i in 1..10000 {
        // one iteration to calculate target value
        for _ in 0..stable_interval {
            step_game_state_until_rock_lands(&mut state, &mut jets);
        }
        let net_height = state.tower_size() - last_height;
        last_height = state.tower_size();
        height_diffs.push(net_height);

        if height_diffs[0] != height_diffs[height_diffs.len() - 1] {
            differentiated = true;
        }

        if height_diffs[0..height_diffs.len() / 2] == height_diffs[height_diffs.len() / 2..] {
            if differentiated {
                // we have found the repeating interval
                repeating_interval = i / 2;
                break;
            } else {
                // we have found the differentiation interval
                differentiated = true;
            }
        }
    }
    if repeating_interval == 0 {
        panic!("no repeating pattern found");
    }
    let repeating_height = (state.tower_size() - base_height) / 2;
    // compute the number of segments we can skip
    let skip_segments =
        (total_iterations - stable_interval) / (repeating_interval * stable_interval);
    let remaining_rocks =
        (total_iterations - stable_interval) % (repeating_interval * stable_interval);

    // calculate the height of the remaining rocks
    for _ in 0..remaining_rocks {
        step_game_state_until_rock_lands(&mut state, &mut jets);
    }
    let remaining_height = state.tower_size() - base_height - (repeating_height * 2);

    base_height + repeating_height * skip_segments + remaining_height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 1514285714288i64);
    }
}
