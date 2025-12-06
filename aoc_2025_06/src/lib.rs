// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/06
    Solution idea:

*/

use aoc_utils::grid::Grid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Number(u64),
    Plus,
    Asterisk,
}

fn parse1(input: &str) -> Vec<Vec<Entry>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(' ')
                .flat_map(|s| match s {
                    "+" => Some(Entry::Plus),
                    "*" => Some(Entry::Asterisk),
                    "" => None,
                    n => Some(Entry::Number(n.parse::<u64>().expect("invalid number"))),
                })
                .collect()
        })
        .collect::<Vec<Vec<Entry>>>()
}

#[tracing::instrument]
pub fn aoc_2025_06_a(input: &str) -> u64 {
    let x = parse1(input);

    debug_assert!(
        x.iter().all(|r| r.len() == x[0].len()),
        "all rows must have the same number of columns"
    );

    let ops_row = x.len() - 1;
    // define operations now to ensure they are not created in the loop and both have the same type
    let add: fn(u64, u64) -> u64 = |a, b| a + b;
    let multiply: fn(u64, u64) -> u64 = |a, b| a * b;

    let results = (0..x[0].len())
        .map(|col| {
            let (f, neutral) = match x[ops_row][col] {
                Entry::Plus => (add, 0u64),
                Entry::Asterisk => (multiply, 1u64),
                _ => panic!("Invalid operator in ops_row"),
            };

            (0..x.len() - 1).fold(neutral, |acc, row| match x[row][col] {
                Entry::Number(n) => f(acc, n),
                _ => panic!("Invalid entry in number rows"),
            })
        })
        .collect::<Vec<u64>>();

    results.iter().sum::<u64>()
}

#[tracing::instrument]
pub fn aoc_2025_06_b(input: &str) -> u64 {
    let _grid = input.parse::<Grid<char>>().expect("valid grid");
    todo!("Math not implemented yet");
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 4277556)]
    fn aoc_2025_06_a_example(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(super::aoc_2025_06_a(input), expected);
    }

    #[test]
    fn aoc_2025_06_a() {
        assert_eq!(super::aoc_2025_06_a(super::INPUT), 6100348226985);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2025_06_b_example(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(super::aoc_2025_06_b(input), expected);
    }

    #[test]
    fn aoc_2025_06_b() {
        assert_eq!(super::aoc_2025_06_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
