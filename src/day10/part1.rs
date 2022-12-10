use super::shared::*;

/**
 * Given a series of commands, generate a Vec<usize> of the state of register `x` at each cycle.
 * The command `addx 1` will take two cycles to complete, and will add 1 to register `x` on the second cycle.
 * The command `noop` will take one cycle to complete, and will have no other effect.
 */
pub fn run(input: &str) -> i32 {
    let cycles = parse_cycles(input);

    // return the 20th item and every 40th item after that
    cycles
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|(i, cycle)| -> i32 { ((i as i32) + 1) * cycle })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 13140i32);
    }
}
