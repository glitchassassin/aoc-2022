use num_bigint::BigUint;

use super::shared::*;

fn run_round(mut monkeys: Vec<Monkey>) -> (Vec<Monkey>, Vec<usize>) {
    let len = monkeys.len();

    // product of all divisibility tests. Because math, we can modulus the worry level within
    // this range and tests will still be correct
    let max = monkeys
        .iter()
        .fold(BigUint::from(1usize), |acc, monkey| acc * monkey.test);

    let mut inspections = vec![];
    for i in 0..len {
        let monkey = &monkeys[i];

        // list items to throw to other monkeys
        let moves = monkey
            .items
            .iter()
            .map(|item| {
                // inspect item, increase worry level
                let mut new_item: BigUint = match monkey.operation {
                    Operations::Add(value) => item + value,
                    Operations::Subtract(value) => item - value,
                    Operations::Multiply(value) => item * value,
                    Operations::Divide(value) => item / value,
                    Operations::Square() => item * item,
                };

                // this keeps the worry level manageable without corrupting the divisibility test
                new_item %= &max;

                // test worry level and throw to another monkey
                let target = if (&new_item / monkey.test) * monkey.test == new_item {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                (target, new_item)
            })
            .collect::<Vec<(usize, BigUint)>>();

        // count inspections
        inspections.push(moves.len());

        // throw items to other monkeys
        moves.iter().for_each(|(target, item)| {
            monkeys[*target].items.push(item.clone());
        });

        // clear items from this monkey
        let monkey = &mut monkeys[i];
        monkey.items.clear();
    }

    (monkeys, inspections)
}

pub fn run(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    let mut inspections: Vec<usize> = (0..monkeys.len()).map(|_| 0).collect();

    for _ in 0..10000 {
        let (new_monkeys, new_inspections) = run_round(monkeys);
        monkeys = new_monkeys;
        inspections
            .iter_mut()
            .zip(new_inspections)
            .for_each(|(i, j)| *i += j);
    }
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 2713310158usize);
    }
}
