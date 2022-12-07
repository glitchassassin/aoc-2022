use super::shared::*;

/**
 * Walk the tree and find the smallest directory that is greater than space_available
 */
fn find_directories(node: &Node, space_needed: usize) -> usize {
    let mut min = usize::max_value();
    if node.size == 0 {
        let size = get_size(node);
        if size >= space_needed && size < min {
            min = size;
        }
    }
    for child in &node.children {
        let size = find_directories(child, space_needed);
        if size >= space_needed && size < min {
            min = size;
        }
    }
    min
}

pub fn run(input: &str) -> usize {
    let directories = parse_commands(input);

    let space_needed = 30000000 - (70000000 - get_size(&directories));

    find_directories(&directories, space_needed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 24933642usize);
    }
}
