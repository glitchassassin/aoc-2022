use itertools::Itertools;
use std::collections::HashMap;

use super::part1::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValveScore {
    pub valve: String,
    pub time: i32,
    pub time_remaining: i32,
}

/**
 * You have a map of valves and the tunnels that connect them. Each valve has a flow rate,
 * and may be open or closed. Find the shortest path to each closed valve, and calculate the
 * time to move to it and open it. Each tunnel takes one minute to move through, and opening
 * the valve takes one minute.
 *
 * For each remaining valve, return the time to open it and its total flow rate for the time remaining.
 */
pub fn next_valve_scores(
    valves: &HashMap<String, Valve>,
    opened: &[&String],
    start: String,
    time_remaining: &i32,
) -> Vec<ValveScore> {
    valves
        .get(&start)
        .unwrap()
        .tunnels
        .iter()
        .filter(|(t, _)| valves.contains_key(t) && !opened.iter().any(|o| o == &t))
        .map(|(tunnel, d)| ValveScore {
            valve: tunnel.clone(),
            time: *d,
            time_remaining: time_remaining - d,
        })
        .filter(|valve_score| valve_score.time_remaining > 0)
        .sorted_by_key(|a| a.time_remaining)
        .collect()
}

/**
 * Given a list of ValveScores, take the most recent score for each Actor, and from those the score that
 * has the most time remaining. Generate the next_valve_scores from that starting point and, for each
 * possibility, return walk_scores for that possibility.
 */
fn walk_scores(
    valves: &HashMap<String, Valve>,
    start: String,
    opened_valves: &[&String],
    time_remaining: &i32,
) -> Vec<Vec<ValveScore>> {
    let next_actions = next_valve_scores(valves, opened_valves, start, time_remaining);

    let mut return_actions = vec![];
    for action in next_actions {
        let mut opened_valves = opened_valves.to_vec();
        opened_valves.push(&action.valve);
        let mut next_actions = walk_scores(
            valves,
            action.valve.clone(),
            &opened_valves,
            &action.time_remaining,
        )
        .iter_mut()
        .map(|a| {
            a.insert(0, action.clone());
            a.clone()
        })
        .collect_vec();

        if next_actions.is_empty() {
            return_actions.push(vec![action]);
        } else {
            return_actions.append(&mut next_actions);
        }
    }

    return_actions
}

fn compute_score(valves: &HashMap<String, Valve>, path: &Vec<ValveScore>) -> i32 {
    let mut score = 0;
    for valve in path {
        score += valve.time_remaining * valves.get(&valve.valve).unwrap().flow_rate;
    }
    score
}

fn best_score(valves: &HashMap<String, Valve>, paths: &[Vec<ValveScore>]) -> i32 {
    paths
        .iter()
        .map(|path| compute_score(valves, path))
        .max()
        .unwrap_or(0)
}

/**
 * Return all possible pairs of disjoint sets of valves. A pair of disjoint sets together
 * has all valves, but neither set overlaps.
 */
fn disjoint_sets(valves: &[String]) -> Vec<(Vec<&String>, Vec<&String>)> {
    let mut sets = vec![];

    for i in 0..(valves.len() / 2) {
        for subset in valves.iter().combinations(i) {
            let complement: Vec<&String> =
                valves.iter().filter(|v| !subset.contains(v)).collect_vec();
            sets.push((subset, complement));
        }
    }

    sets
}

pub fn run(input: &str) -> i32 {
    let valves = parse_valves(input);
    let simplified_valves = simplify_valve_map(&valves, "AA".to_string());

    let keys = &simplified_valves.keys().cloned().collect_vec();
    let sets = disjoint_sets(keys);

    println!("sets: {:?}", sets.len());

    let results = sets
        .iter()
        .map(|(me, elephant)| {
            println!("me: {:?}, elephant: {:?}", me, elephant);
            let my_score = best_score(
                &valves,
                &walk_scores(&simplified_valves, "AA".to_string(), elephant, &26),
            );
            let elephant_score = best_score(
                &valves,
                &walk_scores(&simplified_valves, "AA".to_string(), me, &26),
            );
            my_score + elephant_score
        })
        .collect_vec();

    *results.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 1707i32);
    }
}
