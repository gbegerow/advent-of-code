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
    Edges are labeld by key on D-pad to go West or North

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

   graph  {
        label="Numpad";
        rankdir=LR;

        a -- 0 [label="<"];
        a -- 3 [label="^"];
        0 -- 2 [label="^"];
        3 -- 2 [label="<"];
        3 -- 6 [label="^"];
        2 -- 5 [label="^"];
        2 -- 1 [label="<"];
        6 -- 5 [label="<"];
        6 -- 9 [label="^"];
        5 -- 4 [label="<"];
        5 -- 8 [label="^"];
        4 -- 7 [label="^"];
        9 -- 8 [label="<"];
        8 -- 7 [label="<"];
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

        a -- "^" [label="<"]
        a -- ">" [label="v"]
        "^" -- "v" [label="v"]
        "v" -- ">" [label=">"]
        "v" -- "<" [label="<"]
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

use std::{collections::BTreeMap, fmt::Write, iter::once};

struct KeypadSeries<'a> {
    keypads: Vec<Graph<'a>>,
    distances: Vec<DistanceMatrix>,
}

impl<'a> std::fmt::Debug for KeypadSeries<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (pad, dist) in self.keypads.iter().zip(self.distances.iter()) {
            writeln!(f, "{:?}{:?}\n", pad, dist)?;
        }
        Ok(())
    }
}

impl<'a> Default for KeypadSeries<'a> {
    fn default() -> Self {
        let keypads = vec![
            dpad_graph("manual"),
            dpad_graph("ice"),
            dpad_graph("vacuum"),
            numpad_graph(),
        ];

        let distances = keypads.iter().map(|g| floyd_warshall(g)).collect();

        Self { keypads, distances }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Graph<'a> {
    name: &'a str,
    graph: BTreeMap<char, Vec<char>>,
    keypad: BTreeMap<(char, char), char>,
    display: &'a str,
}

impl<'a> std::fmt::Debug for Graph<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.name)?;
        writeln!(f, "{}", self.display)
    }
}

impl<'a> Graph<'a> {
    fn new(
        name: &'a str,
        graph: BTreeMap<char, Vec<char>>,
        keypad: BTreeMap<(char, char), char>,
        display: &'a str,
    ) -> Self {
        let keypad = keypad
            .into_iter()
            .map(|((a, b), k)| {
                let mirrored_edge = match k {
                    '>' => '<',
                    '<' => '>',
                    '^' => 'v',
                    'v' => '^',
                    _ => unreachable!("Unknown D-Pad direction {}", k),
                };
                [((a, b), k), ((b, a), mirrored_edge)]
            })
            .flatten()
            .collect();

        Self {
            name,
            graph,
            keypad,
            display,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct DistanceMatrix {
    matrix: Vec<u32>,
    keys: Vec<char>,
    prev: Vec<usize>,
    len: usize,
}

impl std::fmt::Debug for DistanceMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Distances")?;
        self.format_matrix(f)?;

        writeln!(f, "Prev Chain")?;
        self.format_prev(f)
    }
}

impl DistanceMatrix {
    fn format_matrix(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let len = self.keys.len();
        let index = |from: usize, to: usize| from * len + to;

        // writeln!(f, "{}", title)?;
        writeln!(
            f,
            "{}",
            self.keys.iter().fold("| |".to_string(), |mut s, k| {
                write!(s, "{:^5}|", k).unwrap();
                s
            })
        )?;
        for (ni, i) in self.keys.iter().enumerate() {
            write!(f, "|{i}|")?;
            for (nj, _j) in self.keys.iter().enumerate() {
                write!(
                    f,
                    "{}|",
                    match self.matrix[index(ni, nj)] {
                        u32::MAX => "  ∞  ".to_string(),
                        _ => format!("{:5}", self.matrix[index(ni, nj)]),
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }

    fn format_prev(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            self.keys.iter().fold("| |".to_string(), |mut s, k| {
                write!(s, "{:^3}|", k).unwrap();
                s
            })
        )?;
        for (ni, i) in self.keys.iter().enumerate() {
            write!(f, "|{i}|")?;
            for (nj, _j) in self.keys.iter().enumerate() {
                write!(
                    f,
                    "{}|",
                    match self.prev[self.index(ni, nj)] {
                        usize::MAX => " ∞ ".to_string(),
                        _ => format!("{:^3}", self.keys[self.prev[self.index(ni, nj)]]),
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn floyd_warshall(graph: &Graph) -> DistanceMatrix {
    let g = &graph.graph;

    let mut keys: Vec<_> = g.keys().copied().collect();
    keys.sort();
    let len = keys.len();

    // distance matrix
    let mut matrix = vec![u32::MAX; len * len];

    // previous node on path (contains index of node in keys)
    let mut prev = vec![usize::MAX; matrix.len()];

    let index = |from: usize, to: usize| from * len + to;

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

                // do not add to infiniity
                if matrix[index(i, k)] < u32::MAX && matrix[index(k, j)] < u32::MAX {
                    if matrix[idx] > matrix[index(i, k)] + matrix[index(k, j)] {
                        matrix[idx] = matrix[index(i, k)] + matrix[index(k, j)];
                        prev[idx] = prev[index(k, j)];
                    }
                }
            }
        }
    }

    let dm = DistanceMatrix {
        matrix,
        keys,
        prev,
        len,
    };

    // println!("{}: {:?}", graph.name, dm);
    dm
}

impl DistanceMatrix {
    #[inline(always)]
    fn index(&self, u: usize, v: usize) -> usize {
        u * self.len + v
    }

    fn get_path(&self, from: char, to: char) -> Vec<char> {
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

        let mut path = Vec::from([self.keys[v]]);
        while u != v {
            v = self.prev[self.index(u, v)];
            path.push(self.keys[v]);
        }
        path.reverse();
        path
    }

    // convert a shortest path between two vertices to a list of keypad directions
    // keypad directions starts and end with moving and pressing from / to 'a' on d-pad
    fn path_to_dpad(
        &self,
        from: char,
        to: char,
        keypad: &BTreeMap<(char, char), char>,
    ) -> Vec<char> {
        (dbg!(self.get_path(from, to)))[..]
            .windows(2)
            .map(|pair| match pair {
                &[a, b] if keypad.contains_key(&(a, b)) => keypad[&(a, b)],
                _ => unreachable!("Unknown pair {:?}", pair),
            })
            // path must end on 'A' on D-pad to press key
            .chain(once('a'))
            .collect()
    }
}

fn numpad_graph<'a>() -> Graph<'a> {
    Graph::new(
        "NumPad",
        //Edges
        BTreeMap::from([
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
        ]),
        // Keypad keys / directions from a -> b  (Only one direction given)
        BTreeMap::from([
            (('a', '0'), '<'),
            (('a', '3'), '^'),
            (('0', '2'), '^'),
            (('3', '2'), '<'),
            (('3', '6'), '^'),
            (('2', '5'), '^'),
            (('2', '1'), '<'),
            (('6', '5'), '<'),
            (('6', '9'), '^'),
            (('5', '4'), '<'),
            (('5', '8'), '^'),
            (('4', '7'), '^'),
            (('9', '8'), '<'),
            (('8', '7'), '<'),
        ]),
        // Display / Layout of Graph
        r"+--------------+         +------------------------+
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
",
    )
}

fn dpad_graph<'a>(name: &'a str) -> Graph<'a> {
    Graph::new(
        name,
        BTreeMap::from([
            ('a', vec!['^', '>']),
            ('^', vec!['a', 'v']),
            ('>', vec!['v', 'a']),
            ('v', vec!['<', '^', '>']),
            ('<', vec!['v']),
        ]),
        BTreeMap::from([
            (('a', '>'), 'v'),
            (('a', '^'), '>'),
            (('^', 'v'), 'v'),
            (('v', '>'), '>'),
            (('<', 'v'), '>'),
        ]),
        r"+---+     +---+     +---+     +---+
| a | --- | ^ | --- | v | --- | < |
+---+     +---+     +---+     +---+
  |                   |
+---+                 |
| > |-----------------+
+---+",
    )
}

#[tracing::instrument]
pub fn aoc_2024_21_a(input: &str) -> usize {
    let keypads = KeypadSeries::default();
    println!("{:?}", keypads);

    let _codes = input
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
    #[case(TEST_INPUT, 126384)]
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
        let g = dpad_graph("Test");

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
        let g = dpad_graph("Test");

        let dm = floyd_warshall(&g);
        let path = dm.get_path('a', '<');

        assert_eq!(path, vec!['a', '>', 'v', '<']);
    }

    #[test]
    fn path_to_keypad_should() {
        let g = dpad_graph("Test");

        let dm = floyd_warshall(&g);
        let path_on_keypad = dm.path_to_dpad('a', '<', &g.keypad);

        assert_eq!(path_on_keypad, vec!['v', '<', '<', 'a']);
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
