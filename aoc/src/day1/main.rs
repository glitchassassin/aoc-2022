use aoc::shared::load_input;
mod part1;
mod part2;

fn main() {
    let input = load_input();
    
    println!("Part 1");
    part1::run(&input);
    
    println!("Part 2");
    part2::run(&input);
}