use super::shared::*;

/**
 * Given a node:
 * 1. If the node has size 0, sum the size of all of its children. If less than 100000, add the size to the sum.
 * 2. For each child, recursively call this function.
 * 3. Return the sum.
 */
fn find_directories(node: &Node) -> usize {
    let mut sum = 0;
    if node.size == 0 {
        let size = get_size(node);
        if size < 100000 {
            sum += size;
        }
    }
    for child in &node.children {
        sum += find_directories(child);
    }
    sum
}

pub fn run(input: &str) -> usize {
    let directories = parse_commands(input);

    find_directories(&directories)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let demo = include_str!("inputs/sample.txt");
        assert_eq!(run(demo), 95437usize);
    }
}
