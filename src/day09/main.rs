mod part1;
mod part2;
mod shared;

fn main() {
    let input = include_str!("inputs/input.txt");

    println!("Part 1: {}", part1::run(input));
    println!("Part 2: {}", part2::run(input));
}
