use std::collections::HashMap;

use super::shared::*;

fn run_monkey(
    monkey: Monkey,
    waiting_for: &mut HashMap<String, Monkey>,
    resolved: &mut HashMap<String, i64>,
) {
    let mut resolve = |op1: &String, op2: &String, op: &dyn Fn(i64, i64) -> i64| {
        let val1 = resolved.get(op1);
        let val2 = resolved.get(op2);
        if let (Some(val1), Some(val2)) = (val1, val2) {
            resolved.insert(monkey.name.clone(), op(*val1, *val2));
            if let Some(waiting) = waiting_for.get(&monkey.name) {
                run_monkey(waiting.clone(), waiting_for, resolved);
            }
        } else {
            if val1.is_none() {
                waiting_for.insert(op1.clone(), monkey.clone());
            }
            if val2.is_none() {
                waiting_for.insert(op2.clone(), monkey.clone());
            }
        }
    };
    match &monkey.op {
        Ops::Yell(n) => {
            resolved.insert(monkey.name.clone(), *n);
            if let Some(waiting) = waiting_for.get(&monkey.name) {
                run_monkey(waiting.clone(), waiting_for, resolved);
            }
        }
        Ops::Add(op1, op2) => {
            resolve(op1, op2, &|a, b| a + b);
        }
        Ops::Sub(op1, op2) => {
            resolve(op1, op2, &|a, b| a - b);
        }
        Ops::Mul(op1, op2) => {
            resolve(op1, op2, &|a, b| a * b);
        }
        Ops::Div(op1, op2) => {
            resolve(op1, op2, &|a, b| a / b);
        }
    }
}

fn run_monkeys(monkeys: Vec<Monkey>) -> i64 {
    let mut waiting_for = HashMap::new();
    let mut resolved = HashMap::new();

    for monkey in monkeys {
        run_monkey(monkey, &mut waiting_for, &mut resolved);
        if let Some(root) = resolved.get("root") {
            return *root;
        }
    }

    0
}

pub fn run(input: &str) -> i64 {
    let monkeys = parse_input(input);
    run_monkeys(monkeys)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 152i64);
    }
}
