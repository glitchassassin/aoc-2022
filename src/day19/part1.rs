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
    for next_state in build_robot_options(state) {
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

    let mut quality_levels = 0;

    for blueprint in blueprints.iter() {
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
            time_remaining: 24,
            blueprint: *blueprint,
        };

        let best_state = walk_tree(state);
        if let Some(states) = best_state {
            quality_levels += states[0].inventory.geode * blueprint.id;
        }
    }

    quality_levels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 33i32);
    }
}
