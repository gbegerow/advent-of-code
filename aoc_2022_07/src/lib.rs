// use std::collections::HashMap;

// use regex::Regex;

// struct Dir<'a> {
//     name: &'a str,
//     childs: HashMap<&'a str, Node<'a>>,
// }
// struct File<'a> {
//     name: &'a str,
//     size: usize,
// }

// #[derive(Debug)]
// enum Node<'a> {
//     Dir(Dir),
//     File(File),
// }

// impl<'a> Dir<'a> {
//     fn mkdir(&mut self, name: &'a str) {
//         if !self.childs.contains_key(name) {
//             self.childs.insert(
//                 name,
//                 Node::Dir( Dir {
//                     name: name,
//                     childs: HashMap::new(),
//                 })
//             );
//         }
//     }

//     fn add_file(&mut self, name: &'a str, size: usize) {
//         if !self.childs.contains_key(name) {
//             self.childs.insert(
//                 name,
//                 Node::File(File {
//                     name: name,
//                     size: size,
//                 }) 
//             );
//         }
//     }

//     fn get(&self, name: &str) -> Option<&Node> {
//         self.childs.get(name)
//     }

//     fn walk_size(&self) -> usize {
//             self.childs.values().map(|n| 
//                 mat
//                 n.walk_size()).sum(),
//             None => self.size,
//         }
//     }
// }

pub fn aoc_2022_07_a(_input: &str) -> usize {
    // let tree = parse_screen(input);
    // tree.walk_size()
    0
}

pub fn aoc_2022_07_b(_input: &str) -> usize {
    0
}

fn parse_screen(input: &str) -> Node {
    let dir_cmd_rx = Regex::new(r"\s*\$ cd (?P<target>.+)").unwrap();
    // let ls_cmd_rx = Regex::new(r"\s*\$ ls").unwrap();
    let dir_rx = Regex::new(r"\s*dir (?P<dir>.+))").unwrap();
    let file_rx = Regex::new(r"\s*(?P<size>\d+)\s+(?P<file>.+)").unwrap();

    let mut root = Node {
        name: "/",
        size: 0,
        childs: Some(HashMap::new()),
    };
    let mut cwd = &root;
    let mut stack = Vec::new();
    for line in input.trim().lines() {
        if let Some(cap) = dir_cmd_rx.captures(line) {
            println!("CD {}", cap.get(0).unwrap().as_str());
            let name = &cap["target"];
            stack.push(cwd);
            cwd.mkdir(name);
            cwd = cwd.get(name).unwrap();
        } else if let Some(cap) = dir_rx.captures(line) {
            println!("DIR {}", cap.get(0).unwrap().as_str());
        } else if let Some(cap) = file_rx.captures(line) {
            println!("FILE {}", cap.get(0).unwrap().as_str());
        }
    }
    root
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_07_a_example() {
        assert_eq!(super::aoc_2022_07_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_07_a() {
        assert_eq!(super::aoc_2022_07_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn aoc_2022_07_b_example() {
        assert_eq!(super::aoc_2022_07_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_07_b() {
        assert_eq!(super::aoc_2022_07_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "
$ cd /
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
}
