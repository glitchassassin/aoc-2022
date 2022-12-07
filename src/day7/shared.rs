use std::borrow::BorrowMut;

// Struct representing a filesystem node
#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub children: Vec<Node>,
    pub size: usize,
}

/**
 * Given a string that is either a file or a folder, generate the appropriate child for the root node
 *
 * Folders will have the format `dir [a]`, where `a` is the name of the folder.
 * Files will have the format `[100] [b]`, where `100` is the size of the file and `b` is the name of the file.
 */
pub fn parse_node(command: &str, root: &mut Node) {
    let mut parts = command.split_whitespace();
    // if the first segment is a number, then it's a file; directories have size 0
    let size = parts.next().unwrap().parse::<usize>().unwrap_or(0);
    let name = parts.next().unwrap().to_string();
    let node = Node {
        name,
        children: vec![],
        size,
    };
    root.children.push(node);
}

/**
 * Given a node and a list of commands, create a stack of Nodes that represent the current directory. For each command in the list:
 *
 * If the line matches '$ cd folder', set the current directory to the folder.
 *
 * If the line matches '$ cd ..', set the current directory to the parent directory.
 *
 * If the line doesn't start with a `$`, then it's a file or folder. Parse it and add it to the current directory.
 *
 * Return the root node.
 */
pub fn parse_commands(commands: &str) -> Node {
    let mut root = Node {
        name: "".to_string(),
        children: vec![Node {
            name: "/".to_string(),
            children: vec![],
            size: 0,
        }],
        size: 0,
    };
    let mut stack = vec![];
    for line in commands.lines() {
        if line.starts_with("$ cd ") {
            let folder = line.split_whitespace().nth(2).unwrap();
            if folder == ".." {
                stack.pop();
            } else {
                stack.push(folder.to_string());
            }
        } else if !line.starts_with('$') {
            let mut current = &mut root;
            for folder in stack.iter() {
                current = current
                    .children
                    .iter_mut()
                    .find(|node| node.name == *folder)
                    .unwrap();
            }
            parse_node(line, current.borrow_mut());
        }
    }
    root
}

/**
 * Given a node, return the size of the node and all of its children.
 */
pub fn get_size(node: &Node) -> usize {
    node.size + node.children.iter().map(get_size).sum::<usize>()
}
