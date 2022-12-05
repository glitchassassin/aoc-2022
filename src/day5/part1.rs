use std::collections::HashMap;

use itertools::Itertools;

pub fn run(input: &str) -> String {
    let (stacks, commands) = input.split("\n\n").next_tuple().unwrap();

    let stack_names: Vec<char> = stacks.split('\n').last().unwrap().trim().chars().skip(0).step_by(4).collect();

    let mut stacks_map: HashMap<char, Vec<char>> = HashMap::new();

    // Initialize stacks
    for stack in stack_names.clone() {
        stacks_map.insert(stack, Vec::new());
    }

    // Populate stacks
    for row in stacks.split('\n') {
        if !row.contains('[') {
            continue;
        }
        stack_names
            .clone()
            .iter()
            .zip(
                row
                    .chars()
                    .skip(1)
                    .step_by(4)
            )
            .filter(|(_, c)| !c.is_whitespace())
            .for_each(|(stack, item)| 
                stacks_map
                    .get_mut(stack)
                    .unwrap()
                    .insert(0, item)
                );
    }


    // parse commands
    for command in commands.split('\n') {
        let (_, count, _, from, _, to) = command.split(' ').next_tuple().unwrap();
        
        let from_char = from.chars().next().unwrap();
        let to_char = to.chars().next().unwrap();

        for _ in 0..count.parse().unwrap() {
            let from_list = stacks_map.get_mut(&from_char).unwrap();
            let element = from_list.pop().unwrap();
            let to_list = stacks_map.get_mut(&to_char).unwrap();
            to_list.push(element);
        }
    }

    stack_names.iter().map(|stack| stacks_map.get(stack).unwrap().iter().last().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), "CMZ");
    }
}