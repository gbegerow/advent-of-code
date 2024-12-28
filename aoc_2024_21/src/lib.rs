// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/21
    Solution idea:

    Build initial distance matrix for both types of keypads
        with Floyd-Warshall or Dijkstra with weight 1.
        Can use petgraph, but think I roll my own.
    Construct path from  key x  to key y
    Calculate cost of path. Cost is now weight in dstance matrix.
    Continue till distance matrix of Numpad is complete.
    Cost of code is just cost of path  on Numpad graph.
    Debug utiil: display expanding path
                 annotate ASCII graph with weights to display weight
                 matrix because we can :-)
                 (replace ab in raw string with weight of edge from a to b)
    No tiime to do this today

    ASCII graphs with: graph-easy --input=graph.dot --as_ascii
    (boxart looks better but ascii is more AoC style)

    ## Numpad
    +---+---+---+
    | 7 | 8 | 9 |
    +---+---+---+
    | 4 | 5 | 6 |
    +---+---+---+
    | 1 | 2 | 3 |
    +---+---+---+
        | 0 | A |
        +---+---+

   graph "Numpad" {
        rankdir=LR;

        a -- 0;
        a -- 3;
        0 -- 2;
        3 -- 2;
        3 -- 6;
        2 -- 5;
        2 -- 1;
        6 -- 5;
        6 -- 9;
        5 -- 4;
        5 -- 8;
        4 -- 7;
        9 -- 8;
        8 -- 7;
    }

  +--------------+         +------------------------+
  |              |         |                        |
  |  +---+     +---+     +---+     +---+     +---+  |
  |  | 0 | --- | 2 | --- | 5 | --- | 4 | --- | 7 |  |
  |  +---+     +---+     +---+     +---+     +---+  |
  |              |         |                   |    |
  |  +---+     +---+     +---+     +---+     +---+  |
  |  | a | --- | 3 | --- | 6 | --- | 9 | --- | 8 | -+
  |  +---+     +---+     +---+     +---+     +---+
  |    |
  |  +---+
  +- | 1 |
     +---+


## D-Pad
        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+

    graph "D-pad" {
        rankdir=LR;

        a -- "^"
        a -- ">"
        "^" -- "v"
        "v" -- ">"
        "v" -- "<"
    }

+---+     +---+     +---+     +---+
| a | --- | ^ | --- | v | --- | < |
+---+     +---+     +---+     +---+
  |                   |
+---+                 |
| > |-----------------+
+---+

┌───┐     ┌───┐     ┌───┐     ┌───┐
│ A │ ─── │ ^ │ ─── │ v │ ─── │ < │
└───┘     └───┘     └───┘     └───┘
  │                   │
  │                   │
  │                   │
┌───┐                 │
│ > │─────────────────┘
└───┘


    Distance matrix D-Pad
    | |a|^|>|v|<|
    |a|0|1|1|2|3|
    |^| |0|2|1|2|
    |>| | |0|1|2|
    |v| | | |0|1|
    |<| | | | |0|

*/
// use aoc_utils::grid::Grid;

use std::{
    collections::{BTreeMap, VecDeque},
    fmt::Write,
    usize,
};

type Graph = BTreeMap<char, Vec<char>>;

struct DistanceMatrix {
    matrix: Vec<u32>,
    keys: Vec<char>,
    prev: Vec<usize>,
    len: usize,
}

fn print_matrix(title: &str, matrix: &Vec<u32>, keys: &Vec<char>) {
    let len = keys.len();
    let index = |from: usize, to: usize| from * len + to;

    println!("{}", title);
    println!(
        "{}",
        keys.iter().fold("| |".to_string(), |mut s, k| {
            write!(s, "{:^5}|", k).unwrap();
            s
        })
    );
    for (ni, i) in keys.iter().enumerate() {
        print!("|{i}|");
        for (nj, _j) in keys.iter().enumerate() {
            print!(
                "{}|",
                match matrix[index(ni, nj)] {
                    u32::MAX => "  ∞  ".to_string(),
                    _ => format!("{:5}", matrix[index(ni, nj)]),
                }
            );
        }
        println!();
    }
}

fn print_prev(title: &str, prev: &Vec<usize>, keys: &Vec<char>) {
    let len = keys.len();
    let index = |from: usize, to: usize| from * len + to;

    println!("{}", title);
    println!(
        "{}",
        keys.iter().fold("| |".to_string(), |mut s, k| {
            write!(s, "{:^3}|", k).unwrap();
            s
        })
    );
    for (ni, i) in keys.iter().enumerate() {
        print!("|{i}|");
        for (nj, _j) in keys.iter().enumerate() {
            print!(
                "{}|",
                match prev[index(ni, nj)] {
                    usize::MAX => " ∞ ".to_string(),
                    _ => format!("{:^3}", keys[prev[index(ni, nj)]]),
                }
            );
        }
        println!();
    }
}

fn floyd_warshall(g: &Graph) -> DistanceMatrix {
    let mut keys: Vec<_> = g.keys().copied().collect();
    keys.sort();
    let len = keys.len();

    // distance matrix
    let mut matrix = vec![u32::MAX; len * len];

    // previous node on path (contains index of node in keys)
    let mut prev = vec![usize::MAX; matrix.len()];

    let index = |from: usize, to: usize| from * len + to;

    println!("Diagonal");
    // fill distance to self
    for i in 0..len {
        let idx = index(i, i);
        matrix[idx] = 0;
        prev[idx] = i;
    }

    // for each edge, initial weight is 1, predecessor is node we came from
    for k in 0..len {
        for i in &g[&keys[k]] {
            if let Some(n) = keys.iter().position(|v| v == i) {
                let idx = index(k, n);
                // println!("'{}' -> '{}' ({}->{}) [{}]", &keys[k], i, k, n, idx);

                matrix[idx] = 1;
                prev[idx] = k;
            }
        }
    }

    // print_matrix("Flood Distances", &matrix, &keys);
    // now calculate the transitive distance/weight
    for k in 0..len {
        for i in 0..len {
            for j in 0..len {
                let idx = index(i, j);
                assert!(
                    idx < matrix.len(),
                    "matrix index out of bounds for {},{}",
                    i,
                    j
                );

                if matrix[index(i, k)] < u32::MAX && matrix[index(k, j)] < u32::MAX {
                    if matrix[idx] > matrix[index(i, k)] + matrix[index(k, j)] {
                        matrix[idx] = matrix[index(i, k)] + matrix[index(k, j)];
                        prev[idx] = prev[index(k, j)];
                    }
                }
            }
        }
    }
    print_matrix("Final Distances", &matrix, &keys);
    print_prev("Final previous chain", &prev, &keys);

    DistanceMatrix {
        matrix,
        keys,
        prev,
        len,
    }
}

impl DistanceMatrix {
    #[inline(always)]
    fn index(&self, u: usize, v: usize) -> usize {
        u * self.len + v
    }

    fn get_path(&self, from: char, to: char) -> VecDeque<char> {
        let Some(u) = self.keys.iter().position(|k| k == &from) else {
            panic!("unknown key {}", from)
        };
        let Some(mut v) = self.keys.iter().position(|k| k == &to) else {
            panic!("unknown key {}", to)
        };

        let idx = self.index(u, v);

        if self.prev[idx] == usize::MAX {
            // with our graphs here, it is an error if there is no path
            unreachable!("All nodes should be reachable from each other");
            // return [];
        }

        let mut path = VecDeque::from([self.keys[v]]);
        while u != v {
            v = self.prev[self.index(u, v)];
            path.push_front(self.keys[v]);
        }
        path
    }
}

fn numpad_graph() -> Graph {
    Graph::from([
        ('a', vec!['0', '3']),
        ('0', vec!['a', '2']),
        ('1', vec!['2', '4']),
        ('2', vec!['0', '1', '3', '5']),
        ('3', vec!['a', '2', '6']),
        ('4', vec!['1', '5', '7']),
        ('5', vec!['2', '4', '6', '8']),
        ('6', vec!['3', '5', '9']),
        ('7', vec!['4', '8']),
        ('8', vec!['5', '7', '9']),
        ('9', vec!['6', '8']),
    ])
}

fn dpad_graph() -> Graph {
    Graph::from([
        ('a', vec!['^', '>']),
        ('^', vec!['a', 'v']),
        ('>', vec!['v', 'a']),
        ('v', vec!['<', '^', '>']),
        ('<', vec!['v']),
    ])
}

#[tracing::instrument]
pub fn aoc_2024_21_a(input: &str) -> usize {
    let codes = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    0
}

#[tracing::instrument]
pub fn aoc_2024_21_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_21_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_21_a(input), expected);
        assert_eq!(super::aoc_2024_21_a(input), expected);
    }

    #[test]
    fn aoc_2024_21_a() {
        assert_eq!(super::aoc_2024_21_a(super::INPUT), 0);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_21_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_21_b(input), expected);
        assert_eq!(super::aoc_2024_21_b(input), expected);
    }

    #[test]
    fn aoc_2024_21_b() {
        assert_eq!(super::aoc_2024_21_b(super::INPUT), 0);
    }

    #[test]
    fn floyd_warshall_should() {
        let g = dpad_graph();

        let dm = floyd_warshall(&g);

        #[rustfmt::skip]
        assert_eq!(
            dm.matrix,
            vec![0, 2, 2, 3, 1, 
                 2, 0, 2, 1, 1, 
                 2, 2, 0, 1, 1, 
                 3, 1, 1, 0, 2, 
                 1, 1, 1, 2, 0]
        );

        #[rustfmt::skip]
        assert_eq!(
            dm.prev,           
            vec![0, 2, 2, 3, 1, 
                 2, 0, 2, 1, 1, 
                 2, 2, 0, 1, 1, 
                 3, 1, 1, 0, 2, 
                 1, 1, 1, 2, 0]
        );
    }

    #[test]
    fn path_should() {
        let g = dpad_graph();

        let dm = floyd_warshall(&g);
        let path = dm.get_path('a', '<');

        assert_eq!(path, VecDeque::from(['a', '>', 'v', '<']));
    }

    const TEST_INPUT: &str = "
    029A
    980A
    179A
    456A
    379A";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
