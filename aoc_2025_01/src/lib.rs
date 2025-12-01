// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/01
    Solution idea:

*/
// use aoc_utils::grid::Grid;

#[tracing::instrument]
pub fn aoc_2025_01_a(input: &str) -> usize {
    // let grid = input.parse::<Grid<char>>().expect("valid grid");
    // let x =input.trim().lines().map(|l| .... )
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

#[tracing::instrument]
pub fn aoc_2025_01_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2025_01_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_01_a(input), expected);
    }

    #[test]
    fn aoc_2025_01_a() {
        assert_eq!(super::aoc_2025_01_a(super::INPUT), 0);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2025_01_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_01_b(input), expected);
    }

    #[test]
    fn aoc_2025_01_b() {
        assert_eq!(super::aoc_2025_01_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
