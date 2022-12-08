use std::rc::Rc;
use std::cell::RefCell;
use std::iter;

#[derive(Debug)]
pub struct Node {
    name: String,
    node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Dir(Vec<Rc<RefCell<Node>>>),
    File(usize),
}

pub fn directories(node: &Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
    match &node.borrow().node_type {
         NodeType::File(_) => Vec::new(),
         NodeType::Dir(children) => {
             let me = iter::once(node.clone());

             let children = children.iter().map(directories).flatten();

             me.chain(children).collect()
         },
    }
}

pub fn size(node: &Rc<RefCell<Node>>) -> usize {
    match &node.borrow().node_type {
         NodeType::File(size) => *size,
         NodeType::Dir(children) => children.iter().map(size).sum::<usize>(),
    }
}

impl Node {
    // pub fn child_by_name(&self, name: &str) -> Option<Rc<RefCell<Node>>> {
    //     match &self.node_type {
    //         NodeType::File(_) => None,
    //         NodeType::Dir(dir) => {
    //             dir.iter().find(|node| node.borrow().name == name).cloned()
    //         }
    //     }
    // }

    pub fn child_by_name_or_create(&mut self, name: &str) -> Option<Rc<RefCell<Node>>> {
        match &mut self.node_type {
            NodeType::File(_) => None,
            NodeType::Dir(ref mut dir) => {
                match dir.iter().find(|node| node.borrow().name == name) {
                    Some(node) => Some(node.clone()),
                    None => {
                        let node = Node {
                            name: name.to_string(),
                            ..Node::default()
                        };

                        let node = Rc::new(RefCell::new(node));
                        dir.push(node.clone());

                        Some(node)
                    }
                }
            }
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: "/".to_string(),
            node_type: NodeType::Dir(Vec::new()),
        }
    }
}

pub fn read(input: &str) -> Rc<RefCell<Node>> {
    let tree = Rc::new(RefCell::new(Node::default()));
    let mut current_node: Rc<RefCell<Node>> = tree.clone();
    let mut commands = input.split("$ ");
    let mut dir_stack: Vec<Rc<RefCell<Node>>> = vec![current_node.clone()];

    commands.next().unwrap(); // ignore cd /

    for command in commands {
        let mut tokens = command.split(|c| c == ' ' || c == '\n');
        let command = tokens.next().unwrap();

        match command {
            "cd" => {
                match tokens.next().unwrap() {
                    ".." => current_node = dir_stack.pop().unwrap(),
                    path => {
                        let new_dir = current_node.borrow_mut().child_by_name_or_create(path).unwrap();
                        dir_stack.push(current_node);
                        current_node = new_dir;
                    }
                }
            },
            "ls" => {
                let mut size: Option<&str> = None;
                for token in tokens {
                    if let Some(arg1) = size {
                        let name = token.to_string();

                        let node = match arg1 {
                            "" => break,
                            "dir" => Node {
                                name,
                                node_type: NodeType::Dir(Vec::new()),
                            },
                            size => Node {
                                name,
                                node_type: NodeType::File(size.parse().unwrap()),
                            }
                        };

                        size = None;

                        let node = Rc::new(RefCell::new(node));

                        let node_type = &mut current_node.borrow_mut().node_type;
                        if let NodeType::Dir(dir) = node_type {
                            dir.push(node);
                        } else {
                            panic!("Current node is not a directory");
                        }

                    } else {
                        size = Some(token);
                    }
                }
            },
            s => panic!("unexpected '{}'", s),
        }
    }

    tree
}

pub fn part1(input: &str) -> String {
    let tree = read(input);
    let dirs = directories(&tree);
    let result = dirs.iter().map(size).filter(|&s| s <= 100000_usize).sum::<usize>().clone();
    format!("{}", result)
}

pub fn part2(input: &str) -> String {
    let tree = read(input);
    let dirs = directories(&tree);

    let total_size = size(&tree);
    let free_space = 70000000 - total_size;
    let needed_space = 30000000 - free_space;
    let mut candidates = dirs.iter().map(size).filter(|&s| s >= needed_space).collect::<Vec<_>>();
    candidates.sort();
    let result = candidates.get(0).unwrap();

    format!("{}", result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "95437");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "24933642");
    }
}
