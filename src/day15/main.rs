mod part1;
mod part2;

fn main() {
    let input = include_str!("inputs/input.txt");

    println!("Part 1: {}", part1::run(input, 2000000));
    println!("Part 2: {}", part2::run(input, 4000000, 4000000));
}
