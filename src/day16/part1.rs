use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Valve {
    pub name: String,
    pub flow_rate: i32,
    pub tunnels: Vec<(String, i32)>,
    pub open: bool,
}

/**
 * Parse a list of strings like:
 *
 * Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
 *
 * Return a vec of Valve structs
 */
pub fn parse_valves(input: &str) -> HashMap<String, Valve> {
    let mut valves = HashMap::new();
    for line in input.lines() {
        let re = Regex::new(r"Valve (?P<name>\w+) has flow rate=(?P<flow_rate>\d+); tunnels? leads? to valves? (?P<tunnels>.+)").unwrap();
        let caps = re.captures(line).unwrap();
        let flow_rate = caps["flow_rate"].parse().unwrap();
        let valve = Valve {
            name: caps["name"].to_string(),
            flow_rate,
            tunnels: caps["tunnels"]
                .split(", ")
                .map(|s| (s.to_string(), 1))
                .collect(),
            open: flow_rate == 0, // treat valves with a flow rate of 0 as already open
        };
        valves.insert(valve.name.clone(), valve);
    }
    valves
}

fn potential_flow_rate(valve: &Valve) -> i32 {
    if valve.open {
        0
    } else {
        valve.flow_rate
    }
}

pub fn simplify_valve_map(
    valves: &HashMap<String, Valve>,
    start: String,
) -> HashMap<String, Valve> {
    let mut simplified_map = HashMap::new();

    for (name, valve) in valves {
        if valve.flow_rate > 0 || name == &start {
            // get distances to all other valves with flow rate > 0
            let mut simplified_valve = valve.clone();
            simplified_valve.tunnels = valve_distances(valves, name.clone());
            simplified_map.insert(name.clone(), simplified_valve);
        }
    }

    simplified_map
}

pub fn valve_distances(valves: &HashMap<String, Valve>, start: String) -> Vec<(String, i32)> {
    let mut frontier = vec![start.clone()];
    let mut visited = vec![];
    let mut distance = HashMap::new();
    distance.insert(start.clone(), 0);

    while !frontier.is_empty() {
        let current = frontier.pop().unwrap();
        visited.push(current.clone());
        let current_valve = valves.get(&current).unwrap();
        for (next, d) in &current_valve.tunnels {
            if !visited.contains(next) {
                frontier.insert(0, next.clone());
                let next_distance = distance.get(&current).unwrap() + d;
                if distance.contains_key(next) && distance.get(next).unwrap() < &next_distance {
                    continue;
                }
                distance.insert(next.clone(), next_distance);
            }
        }
    }

    distance
        .iter()
        .map(|(k, v)| (k.clone(), v + 1))
        .filter(|(k, _)| k != &start && valves.get(k).unwrap().flow_rate > 0)
        .collect()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValveScore(pub String, pub i32);

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
    start: String,
    time_remaining: &i32,
) -> Vec<ValveScore> {
    valves
        .get(&start)
        .unwrap()
        .tunnels
        .iter()
        .filter(|(t, _)| valves.contains_key(t))
        .map(|(tunnel, d)| ValveScore(tunnel.clone(), *d))
        .filter(|valve_score| valve_score.1 <= *time_remaining)
        .collect()
}

fn walk_scores(
    valves: &HashMap<String, Valve>,
    start: String,
    time_remaining: &i32,
) -> Vec<Vec<ValveScore>> {
    let scores = next_valve_scores(valves, start.clone(), time_remaining);
    // generate new valve state, given this change
    let mut new_valves = valves.clone();
    new_valves.remove(&start);

    let results = scores.iter().flat_map(|valve_score| {
        let ValveScore(new_start, time) = valve_score.clone();
        // repeat for new valve state
        let new_results = walk_scores(&new_valves, new_start, &(time_remaining - time));

        if new_results.is_empty() {
            return vec![vec![valve_score.clone()]];
        }

        new_results
            .iter()
            .map(|v| {
                let mut new_v = v.clone();
                new_v.insert(0, valve_score.clone());
                new_v
            })
            .collect::<Vec<Vec<_>>>()
    });
    results.collect::<Vec<_>>()
}

fn compute_score(valves: &HashMap<String, Valve>, path: &Vec<ValveScore>) -> i32 {
    let mut score = 0;
    let mut time_remaining = 30;
    for ValveScore(name, time) in path {
        time_remaining -= time;
        score += time_remaining * potential_flow_rate(valves.get(name).unwrap());
    }
    score
}

pub fn run(input: &str) -> i32 {
    let valves = parse_valves(input);
    let simplified_valves = simplify_valve_map(&valves, "AA".to_string());
    let results = walk_scores(&simplified_valves, "AA".to_string(), &30);
    let results = results
        .iter()
        .map(|path| (compute_score(&simplified_valves, path), path));

    let (score, _) = results.max_by_key(|(score, _)| *score).unwrap();

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 1651i32);
    }
}
