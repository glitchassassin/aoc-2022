use super::shared::*;

pub fn run(input: &str) -> usize {
    let (map, instructions) = parse_input(input);
    let mut player = PlayerState::initialize(&map);

    for inst in instructions {
        player.follow_instruction(&inst, &map);
    }

    player.password()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 6032usize);
    }
}
