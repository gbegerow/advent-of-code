/*
    After gotten burned pretty hard with a traditional mutatable node based tree
    and seeing lots of the discussions on reddit, I try to simplify it like raul100 did.
    I don't want to directly take an AreanaTree, but I take some ideas from it and ECS.
    For my own sanity I want to keep the names of directories. Technically we only need
    the accumulative size and the nesting.
*/
use regex::Regex;

#[allow(dead_code)]
#[derive(Debug)]
struct Directory {
    name: String,
    size: usize,
    parent: usize,
}

fn parse_screen(input: &str) -> Vec<Directory> {
    // just go with a simple vector of directories.
    // parent is just the index in the vector of parent dir
    // size is accumulative size of all files and directories beneath.
    let mut directories = vec![Directory {
        name: "/".to_string(),
        size: 0,
        parent: 0,
    }];
    // root is always at index 0 so we can make it its own parent, no need for an Option
    let root = 0;
    // no need for stack, we just store the current index
    let mut cwd = root;

    // no need for lazy static, these are created only once
    // we only care about changing the directory
    let dir_rx = Regex::new(r"\s*\$ cd (?P<target>.+)").unwrap();
    // and files for their size
    let file_rx = Regex::new(r"\s*(?P<size>\d+)\s+(?P<file>.+)").unwrap();

    for line in input.lines() {
        if let Some(cap) = dir_rx.captures(line) {
            // println!("CD {}", cap.get(0).unwrap().as_str());
            match &cap["target"] {
                "/" => cwd = 0,
                ".." => cwd = directories[cwd].parent,
                name => {
                    // put it in filesystem. We never visit a dir twice so it's safe.
                    directories.push(Directory {
                        name: String::from(name), // capture should be a slice of input but it has different lifetime
                        size: 0,
                        parent: cwd,
                    });
                    cwd = directories.len() - 1; // cwd is now the last entry
                }
            }
        } else if let Some(cap) = file_rx.captures(line) {
            // println!("FILE {}", cap.get(0).unwrap().as_str());
            // we could store the files but there is no need for it
            let size = &cap["size"].parse().unwrap();

            // accumalate size on the whole path
            let mut p = cwd;
            loop {
                directories[p].size += size;
                if p == root {
                    break;
                }
                p = directories[p].parent;
            }
        }
    }

    directories
}

pub fn aoc_2022_07_a(input: &str) -> usize {
    let filesystem = parse_screen(input);
    // println!("{:?}", filesystem);
    filesystem
        .iter()
        .map(|d| d.size)
        .filter(|size| *size <= 100000)
        .sum()
}

pub fn aoc_2022_07_b(input: &str) -> usize {
    let filesystem = parse_screen(input);
    let free_space = 70000000 - filesystem[0].size;
    let need_to_free = 30000000 - free_space;

    let _size = filesystem[0].size; // root is max, everything else is smaller
    filesystem
        .iter()
        .map(|d| d.size)
        .filter(|size| *size >= need_to_free)
        .min()
        .expect("a least one directory must be deleted")    
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_07_a_example() {
        assert_eq!(super::aoc_2022_07_a(TEST_INPUT), 95437);
    }

    #[test]
    fn aoc_2022_07_a() {
        assert_eq!(super::aoc_2022_07_a(include_str!("input.txt")), 1513699);
    }

    #[test]
    fn aoc_2022_07_b_example() {
        assert_eq!(super::aoc_2022_07_b(TEST_INPUT), 24933642);
    }

    #[test]
    fn aoc_2022_07_b() {
        assert_eq!(super::aoc_2022_07_b(include_str!("input.txt")), 7991939);
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

// remember to not do this again!
// use std::collections::HashMap;

// use regex::Regex;

// #[derive(Debug)]
// struct Dir<'a> {
//     name: &'a str,
//     childs: HashMap<&'a str, Node<'a>>,
// }

// #[derive(Debug)]
// struct File<'a> {
//     name: &'a str,
//     size: usize,
// }

// #[derive(Debug)]
// enum Node<'a> {
//     Dir(Dir<'a>),
//     File(File<'a>),
// }

// impl<'a> Node<'a> {
//     fn walk_size(&self) -> usize {
//         match self {
//            Node::Dir(dir)=> dir.childs.values().map(|n| n.walk_size()).sum(),
//            Node::File(file) => file.size,
//         }

//     }
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
//     fn get_mut(&mut self, name: &str) -> Option<&mut Node> {
//         self.childs.get_mut(name)
//     }

// }

// fn parse_screen(input: &str) -> Node {
//     let dir_cmd_rx = Regex::new(r"\s*\$ cd (?P<target>.+)").unwrap();
//     // let ls_cmd_rx = Regex::new(r"\s*\$ ls").unwrap();
//     let dir_rx = Regex::new(r"\s*dir (?P<dir>.+))").unwrap();
//     let file_rx = Regex::new(r"\s*(?P<size>\d+)\s+(?P<file>.+)").unwrap();

//     let mut root = Dir {
//         name: "/",
//         childs: HashMap::new(),
//     };
//     let mut cwd = &mut root;
//     let mut stack:Vec<&mut Dir> = Vec::new();

//     for line in input.trim().lines() {
//         if let Some(cap) = dir_cmd_rx.captures(line) {
//             println!("CD {}", cap.get(0).unwrap().as_str());
//             let name = &cap["target"];
//             cwd.mkdir(name);
//             match cwd.get_mut(name) {
//                         Some(Node::Dir(dir)) => { stack.push(cwd); cwd = dir},
//                 _ => panic!("Dir does not exists")
//             };
//         } else if let Some(cap) = dir_rx.captures(line) {
//             println!("DIR {}", cap.get(0).unwrap().as_str());
//         } else if let Some(cap) = file_rx.captures(line) {
//             println!("FILE {}", cap.get(0).unwrap().as_str());
//         }
//     }
//     Node::Dir(root)
// }

// pub fn aoc_2022_07_a(input: &str) -> usize {
// let tree = parse_screen(input);
// tree.walk_size()
// }
