// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/21
    Solution idea:

    Build initial distance matrix for both types of keypads
        with Floyd-Warshall or Dijkstra with weight 1.
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
        a -- 1
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

    Distance matrix D-Pad
    | |a|^|>|v|<|
    |a|0|1|1|2|3|
    |^| |0|2|1|2|
    |>| | |0|1|2|
    |v| | | |0|1|
    |<| | | | |0|

*/
// use aoc_utils::grid::Grid;

#[tracing::instrument]
pub fn aoc_2024_21_a(input: &str) -> usize {
    // let grid = input.parse::<Grid<char>>().expect("valid grid");
    // let x =input.trim().lines().map(|l| .... )
    // for line in input.trim().lines() {
    //     //
    // }
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

    const TEST_INPUT: &str = "";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
