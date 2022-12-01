mod part1;
mod part2;

fn main() {
    let input = include_str!("../../../inputs/day1.txt");
    
    println!("Part 1");
    println!("Largest inventory: {}", part1::run(input));
    
    println!("Part 2");
    println!("Top 3 inventories: {}", part2::run(input));
}