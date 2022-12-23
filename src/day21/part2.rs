use std::collections::HashMap;

use super::shared::*;

#[derive(Debug, Clone)]
enum Operation {
    Variable(),
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
    Eq(String, String),
}

fn contains_variable(string: &String, variables: &HashMap<String, Operation>) -> bool {
    match variables.get(string) {
        Some(Operation::Variable()) => true,
        Some(Operation::Num(_)) => false,
        Some(Operation::Add(ref op1, ref op2)) => {
            contains_variable(op1, variables) || contains_variable(op2, variables)
        }
        Some(Operation::Sub(ref op1, ref op2)) => {
            contains_variable(op1, variables) || contains_variable(op2, variables)
        }
        Some(Operation::Mul(ref op1, ref op2)) => {
            contains_variable(op1, variables) || contains_variable(op2, variables)
        }
        Some(Operation::Div(ref op1, ref op2)) => {
            contains_variable(op1, variables) || contains_variable(op2, variables)
        }
        Some(Operation::Eq(ref op1, ref op2)) => {
            contains_variable(op1, variables) || contains_variable(op2, variables)
        }
        None => panic!("invalid variable"),
    }
}

/**
 * Given a key to an Eq operation, find the side with the Variable(). If it only has a Variable, we're done!
 *
 * Otherwise, we need to simplify the operation.
 */
fn simplify(string: &String, variables: &mut HashMap<String, Operation>) -> bool {
    let readonly = variables.clone();
    let eq_op = readonly.get(string);
    if let Some(Operation::Eq(op1, op2)) = eq_op {
        let (op_to_reduce, op_to_replace) = if contains_variable(op1, &readonly) {
            (op1.clone(), op2.clone())
        } else {
            (op2.clone(), op1.clone())
        };
        let var_op = readonly.get(&op_to_reduce);
        match var_op {
            Some(Operation::Variable()) => {
                return true; // equation simplified!
            }
            Some(Operation::Add(inner_op1, inner_op2)) => {
                // create new Operation::Sub with the non-variable side and op2
                // then set string to Operation::Eq(inner, new)
                let (var_op_id, non_var_op_id) = if contains_variable(inner_op1, variables) {
                    (inner_op1.clone(), inner_op2.clone())
                } else {
                    (inner_op2.clone(), inner_op1.clone())
                };
                let new_op = Operation::Sub(op_to_replace.clone(), non_var_op_id.clone());
                let new_op_name = format!("{} - {}", op_to_replace, non_var_op_id);
                variables.insert(new_op_name.clone(), new_op);
                variables.insert(string.clone(), Operation::Eq(var_op_id, new_op_name));
            }
            Some(Operation::Sub(inner_op1, inner_op2)) => {
                // Sub is special because it's not commutative
                if contains_variable(inner_op1, variables) {
                    let new_op = Operation::Add(op_to_replace.clone(), inner_op2.clone());
                    let new_op_name = format!("{} + {}", op_to_replace, inner_op2);
                    variables.insert(new_op_name.clone(), new_op);
                    variables.insert(
                        string.clone(),
                        Operation::Eq(inner_op1.clone(), new_op_name),
                    );
                } else {
                    let new_op = Operation::Sub(inner_op1.clone(), op_to_replace.clone());
                    let new_op_name = format!("{} - {}", op_to_replace, inner_op1);
                    variables.insert(new_op_name.clone(), new_op);
                    variables.insert(
                        string.clone(),
                        Operation::Eq(inner_op2.clone(), new_op_name),
                    );
                };
            }
            Some(Operation::Mul(inner_op1, inner_op2)) => {
                // create new Operation::Sub with the non-variable side and op2
                // then set string to Operation::Eq(inner, new)
                let (var_op_id, non_var_op_id) = if contains_variable(inner_op1, variables) {
                    (inner_op1.clone(), inner_op2.clone())
                } else {
                    (inner_op2.clone(), inner_op1.clone())
                };
                let new_op = Operation::Div(op_to_replace.clone(), non_var_op_id.clone());
                let new_op_name = format!("{} / {}", op_to_replace, non_var_op_id);
                variables.insert(new_op_name.clone(), new_op);
                variables.insert(string.clone(), Operation::Eq(var_op_id, new_op_name));
            }
            Some(Operation::Div(inner_op1, inner_op2)) => {
                // Div is special because it's not commutative
                if contains_variable(inner_op1, variables) {
                    let new_op = Operation::Mul(op_to_replace.clone(), inner_op2.clone());
                    let new_op_name = format!("{} * {}", op_to_replace, inner_op2);
                    variables.insert(new_op_name.clone(), new_op);
                    variables.insert(
                        string.clone(),
                        Operation::Eq(inner_op1.clone(), new_op_name),
                    );
                } else {
                    let new_op = Operation::Div(inner_op1.clone(), op_to_replace.clone());
                    let new_op_name = format!("{} / {}", op_to_replace, inner_op1);
                    variables.insert(new_op_name.clone(), new_op);
                    variables.insert(
                        string.clone(),
                        Operation::Eq(inner_op2.clone(), new_op_name),
                    );
                };
            }
            _ => panic!("invalid variable"),
        }
    } else {
        panic!("Not an Eq, can't reduce")
    }
    false
}

#[allow(dead_code)]
fn render(string: &String, variables: &HashMap<String, Operation>) -> String {
    match variables.get(string) {
        Some(Operation::Num(num)) => format!("{}", num),
        Some(Operation::Variable()) => string.clone(),
        Some(Operation::Add(op1, op2)) => {
            format!("({} + {})", render(op1, variables), render(op2, variables))
        }
        Some(Operation::Sub(op1, op2)) => {
            format!("({} - {})", render(op1, variables), render(op2, variables))
        }
        Some(Operation::Mul(op1, op2)) => {
            format!("({} * {})", render(op1, variables), render(op2, variables))
        }
        Some(Operation::Div(op1, op2)) => {
            format!("({} / {})", render(op1, variables), render(op2, variables))
        }
        Some(Operation::Eq(op1, op2)) => {
            format!("{} = {}", render(op1, variables), render(op2, variables))
        }
        None => panic!("invalid variable"),
    }
}

fn reduce(string: &String, variables: &HashMap<String, Operation>) -> i64 {
    let op = variables.get(string);
    match op {
        Some(Operation::Num(num)) => *num,
        Some(Operation::Add(op1, op2)) => reduce(op1, variables) + reduce(op2, variables),
        Some(Operation::Sub(op1, op2)) => reduce(op1, variables) - reduce(op2, variables),
        Some(Operation::Mul(op1, op2)) => reduce(op1, variables) * reduce(op2, variables),
        Some(Operation::Div(op1, op2)) => reduce(op1, variables) / reduce(op2, variables),
        _ => panic!("invalid variable"),
    }
}

pub fn run(input: &str) -> i64 {
    let monkeys = parse_input(input);
    let mut operations = HashMap::new();

    for monkey in monkeys {
        let operation = match monkey.name.as_str() {
            "humn" => Operation::Variable(),
            "root" => match monkey.op {
                Ops::Add(ref op1, ref op2) => Operation::Eq(op1.clone(), op2.clone()),
                Ops::Sub(ref op1, ref op2) => Operation::Eq(op1.clone(), op2.clone()),
                Ops::Mul(ref op1, ref op2) => Operation::Eq(op1.clone(), op2.clone()),
                Ops::Div(ref op1, ref op2) => Operation::Eq(op1.clone(), op2.clone()),
                _ => panic!("invalid root op"),
            },
            _ => match monkey.op {
                Ops::Yell(n) => Operation::Num(n),
                Ops::Add(ref op1, ref op2) => Operation::Add(op1.clone(), op2.clone()),
                Ops::Sub(ref op1, ref op2) => Operation::Sub(op1.clone(), op2.clone()),
                Ops::Mul(ref op1, ref op2) => Operation::Mul(op1.clone(), op2.clone()),
                Ops::Div(ref op1, ref op2) => Operation::Div(op1.clone(), op2.clone()),
            },
        };
        operations.insert(monkey.name.clone(), operation);
    }
    // solve for humn
    // println!("{}", render(&"root".to_string(), &operations));
    while !simplify(&"root".to_string(), &mut operations) {
        // println!("{}", render(&"root".to_string(), &operations));
    }
    // println!("{}", render(&"root".to_string(), &operations));

    if let Some(Operation::Eq(op1, op2)) = operations.get("root") {
        let op_to_reduce = if op1 == "humn" { op2 } else { op1 };
        reduce(op_to_reduce, &operations)
    } else {
        panic!("invalid root op");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 301i64);
    }
}
