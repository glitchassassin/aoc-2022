use itertools::Itertools;
use num_bigint::BigUint;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operations {
    Add(usize),
    Subtract(usize),
    Multiply(usize),
    Divide(usize),
    Square(),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: Vec<BigUint>,
    pub operation: Operations,
    pub test: usize,     // Divisible by `test`?
    pub if_true: usize,  // Index of a monkey
    pub if_false: usize, // Index of a monkey
}

pub fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    for line in input.lines() {
        // check if line matches regex /Monkey (\d+)/

        if Regex::new(r"Monkey (\d+)").unwrap().is_match(line) {
            monkeys.push(Monkey {
                items: vec![],
                operation: Operations::Add(0),
                test: 0,
                if_true: 0,
                if_false: 0,
            });
            continue;
        }
        // get current monkey
        let monkey = monkeys.last_mut().unwrap();

        // check starting items
        if let Some(starting_items) = Regex::new(r"  Starting items: (.*)")
            .unwrap()
            .captures(line)
            .map(|c| {
                c.iter()
                    .nth(1)
                    .unwrap()
                    .unwrap()
                    .as_str()
                    .split(", ")
                    .map(|i| i.parse::<BigUint>().unwrap())
                    .collect::<Vec<BigUint>>()
            })
        {
            // println!("Starting items: {:?}", starting_items);
            if !starting_items.is_empty() {
                monkey.items = starting_items;
                continue;
            }
        }

        // check operation
        if let Some(operation_string) = Regex::new(r"  Operation: new = (.*)")
            .unwrap()
            .captures(line)
            .map(|c| c.iter().nth(1).unwrap().unwrap().as_str().to_string())
        {
            let (_, operator, operand) = operation_string.split_whitespace().next_tuple().unwrap();
            // println!("Operation: {} {}", operator, operand);
            let operation = match operator {
                "+" => Operations::Add(operand.parse().unwrap()),
                "-" => Operations::Subtract(operand.parse().unwrap()),
                "*" => match operand {
                    "old" => Operations::Square(),
                    _ => Operations::Multiply(operand.parse().unwrap()),
                },
                "/" => Operations::Divide(operand.parse().unwrap()),
                _ => panic!("Unknown operation: {}", operator),
            };
            monkey.operation = operation;
            continue;
        }

        // check test
        if let Some(divisible) = Regex::new(r"  Test: divisible by (\d+)")
            .unwrap()
            .captures(line)
            .map(|c| c.iter().nth(1).unwrap().unwrap().as_str().to_string())
        {
            monkey.test = divisible.parse().unwrap();
            continue;
        }

        // check if_true
        if let Some(target) = Regex::new(r"    If true: throw to monkey (\d+)")
            .unwrap()
            .captures(line)
            .map(|c| c.iter().nth(1).unwrap().unwrap().as_str().to_string())
        {
            monkey.if_true = target.parse().unwrap();
            continue;
        }

        // check if_false
        if let Some(target) = Regex::new(r"    If false: throw to monkey (\d+)")
            .unwrap()
            .captures(line)
            .map(|c| c.iter().nth(1).unwrap().unwrap().as_str().to_string())
        {
            monkey.if_false = target.parse().unwrap();
            continue;
        }
    }

    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_monkeys() {
        let demo = include_str!("inputs/sample.txt");
        let monkey = parse_monkeys(demo)[0].clone();
        assert_eq!(
            monkey.items,
            vec![BigUint::from(79usize), BigUint::from(98usize)]
        );
        assert_eq!(monkey.operation, Operations::Multiply(19));
        assert_eq!(monkey.test, 23);
        assert_eq!(monkey.if_true, 2);
        assert_eq!(monkey.if_false, 3);
    }
}
