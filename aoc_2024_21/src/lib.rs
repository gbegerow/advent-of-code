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
    No tiime to do this today

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

               +---+
               | 0 |
               +---+
                 |
               +---+
       +-------| 2 | -+
       |       +---+  |
       |         |    |
       |       +---+  |
       |    +- | 5 | -+----+
       |    |  +---+  |    |
       |    |    |    |    |
       |    |  +---+  |    |
       |    |  | 4 |  |    |
       |    |  +---+  |    |
       |    |    |    |    |
       |    |  +---+  |    |
       |    |  | 7 | -+----+----+
       |    |  +---+  |    |    |
  +----+----+         |    |    |
  |    |              |    |    |
  |  +---+     +---+  |    |    |
  |  | 1 | --- | a |  |    |    |
  |  +---+     +---+  |    |    |
  |              |    |    |    |
  |            +---+  |    |    |
  |            | 3 | -+    |    |
  |            +---+       |    |
  |              |         |    |
  |            +---+       |    |
  |            | 6 | ------+    |
  |            +---+            |
  |              |              |
  |            +---+            |
  |            | 9 |            |
  |            +---+            |
  |              |              |
  |            +---+            |
  +----------- | 8 | -----------+
               +---+





        +---+---+
        | ^ | A |
    +---+---+---+
    | < | v | > |
    +---+---+---+

    graph "D-pad" {
        a -- "^"
        a -- ">"
        "^" -- "v"
        "v" -- ">"
        "v" -- "<"
    }

+---+     +---+
| > | --- | a |
+---+     +---+
  |         |
  |       +---+
  |       | ^ |
  |       +---+
  |         |
  |       +---+
  +-------| v |
          +---+
            |
          +---+
          | < |
          +---+




    Distance matrix D-Pad
    | |a|^|>|v|<|
    |a|0|1|1|2|3|
    |^| |0|2|1|2|
    |>| | |0|1|2|
    |v| | | |0|1|
    |<| | | | |0|

*/
// use aoc_utils::grid::Grid;

use std::collections::BTreeMap;

type Graph = BTreeMap<char, Vec<char>>;
struct DistanceMatrix {
     matrix: Vec<u32>,
     keys: Vec<char>,
     len :usize,
    };

    fn floyd_warshall(g: &Graph) -> DistanceMatrix {
        let keys: Vec<_> = g.keys().collect();
        keys.sort();
        let len = keys.len();

        let mut matrix = vec![u32::MAX; keys.len() * keys.len()];

        let mut index= |a:usize, b:usize | a*len+b;


        // fill distance to self
        for i in 0..len   {
            matrix[index(i,i)] = 0; 
        }

        // for each edge, initial weight is 1
        for k in 0..len{
            for i in g[keys[k]]{
                if let Some(n) = keys.iter().position(|v| v== i){
                    matrix[index(n, k)] =  1;
                }
            }
        }

        for i in 0..len{
            for j in 0..len{

            }
        }

        DistanceMatrix {matrix, keys, len}
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
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_21_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_21_a(input), exepected);
        assert_eq!(super::aoc_2024_21_a(input), exepected);
    }

    #[test]
    fn aoc_2024_21_a() {
        assert_eq!(super::aoc_2024_21_a(super::INPUT), 0);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_21_b_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_21_b(input), exepected);
        assert_eq!(super::aoc_2024_21_b(input), exepected);
    }

    #[test]
    fn aoc_2024_21_b() {
        assert_eq!(super::aoc_2024_21_b(super::INPUT), 0);
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
