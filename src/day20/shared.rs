/**
 * Load lines with numbers from the input file
 */
pub fn parse_input(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}
