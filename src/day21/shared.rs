use regex::Regex;

#[derive(Debug, Clone)]
pub enum Ops {
    Add(String, String),
    Mul(String, String),
    Sub(String, String),
    Div(String, String),
    Yell(i64),
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub name: String,
    pub op: Ops,
}

/**
 * Load lines with monkey jobs from file
 *
 * Lines are formatted like `name: n` for Yell ops or `name: nam2 * nam3` for other ops
 */
pub fn parse_input(input: &str) -> Vec<Monkey> {
    let mut monkeys = vec![];
    let re_math = Regex::new(r"(.{4}): (.{4}) ([+\-/\*]) (.{4})").unwrap();
    let re_yell = Regex::new(r"(.{4}): (-?\d+)").unwrap();
    for line in input.lines() {
        if let Some(caps) = re_math.captures(line) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let op_type = caps.get(3).unwrap().as_str();
            let operand1 = caps.get(2).unwrap().as_str().to_string();
            let operand2 = caps.get(4).unwrap().as_str().to_string();
            let op = match op_type {
                "+" => Ops::Add(operand1, operand2),
                "-" => Ops::Sub(operand1, operand2),
                "*" => Ops::Mul(operand1, operand2),
                "/" => Ops::Div(operand1, operand2),
                _ => panic!("Unknown op type {}", op_type),
            };
            monkeys.push(Monkey { name, op });
        } else if let Some(caps) = re_yell.captures(line) {
            let name = caps.get(1).unwrap().as_str().to_string();
            let op = Ops::Yell(caps.get(2).unwrap().as_str().parse().unwrap());
            monkeys.push(Monkey { name, op });
        } else {
            panic!("Unknown line format: {}", line);
        }
    }
    monkeys
}
