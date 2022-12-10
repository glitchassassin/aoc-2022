pub fn parse_cycles(input: &str) -> Vec<i32> {
    let mut cycles: Vec<i32> = vec![1];

    for line in input.lines() {
        let mut words = line.split_whitespace();
        let command = words.next().unwrap();

        match command {
            "addx" => {
                let value = words.next().unwrap().parse::<i32>().unwrap();
                cycles.push(cycles[cycles.len() - 1]);
                cycles.push(cycles[cycles.len() - 1] + value);
            }
            "noop" => {
                cycles.push(cycles[cycles.len() - 1]);
            }
            _ => panic!("Unknown command: {}", command),
        }
    }
    cycles
}
