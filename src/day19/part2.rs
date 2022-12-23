use super::shared::*;

/**
 * For each possible option, recursively build the tree of possible states,
 * ending when the time_remaining is zero. Return the state with the highest
 * geode inventory level.
 */
pub fn walk_tree(state: State) -> Option<Vec<State>> {
    if state.time_remaining == 0 {
        return None;
    }
    let mut best_state: Option<Vec<State>> = None;
    let options = build_robot_options(state);
    for next_state in options {
        let next_best_state = walk_tree(next_state).unwrap_or_else(|| vec![next_state]);
        if let Some(best) = &best_state {
            if next_best_state[0].inventory.geode > best[0].inventory.geode {
                best_state = Some(next_best_state);
            }
        } else {
            best_state = Some(next_best_state);
        }
    }
    if let Some(mut best) = best_state {
        best.push(state);
        best_state = Some(best);
    }
    best_state
}

pub fn run(input: &str) -> i32 {
    let blueprints = parse_blueprints(input);

    let mut geode_counts = 1;

    for blueprint in blueprints.iter().take(3) {
        let state = State {
            inventory: ResourceVec {
                ore: 0,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            inputs: ResourceVec {
                ore: 1,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            time_remaining: 32,
            blueprint: *blueprint,
        };

        let best_state = walk_tree(state);
        if let Some(states) = best_state {
            println!("{}: {}", blueprint.id, states[0].inventory.geode);
            geode_counts *= states[0].inventory.geode;
        }
    }

    geode_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 56i32 * 62i32);
    }
}
