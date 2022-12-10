use super::shared::*;
use itertools::Itertools;

/**
 * Given a series of commands, generate a Vec<i32> of the state of register `x` at each cycle.
 * The command `addx 1` will take two cycles to complete, and will add 1 to register `x` on the second cycle.
 * The command `noop` will take one cycle to complete, and will have no other effect.
 */
pub fn run(input: &str) -> String {
    let mut cycles = vec![1];
    cycles.extend(parse_cycles(input).iter());

    // return the 20th item and every 40th item after that
    let chunks = cycles
        .iter()
        .enumerate()
        .skip(1)
        .map(|(i, cycle)| -> char {
            if ((cycle - 1)..=(cycle + 1)).contains(&(((i - 1) % 40) as i32)) {
                '#'
            } else {
                '.'
            }
        })
        .chunks(40);
    chunks
        .into_iter()
        .map(|chunk| chunk.collect::<String>())
        .filter(|line| line.len() == 40)
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(
            run(demo),
            r"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
            "
            .trim()
            .to_string()
        );
    }
}
