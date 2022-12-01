pub fn run(input: &str) {
    let mut elves: Vec<u32> = Vec::new();
    elves.push(0); // initialize first elf's inventory

    // Sum the elves' inventories
    for line in input.split('\n') {
        if line.is_empty() {
            elves.push(0);
            continue;
        }
        let last_index = elves.len() - 1;
        let amount: u32 = line.parse().unwrap_or_else(|_| panic!("Could not parse number from line: {}", line));
        elves[last_index] += amount;
    }

    // Get the largest inventory
    let result = elves.iter().max().unwrap();
    println!("Largest inventory: {}", result);
}