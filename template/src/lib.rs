// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/9999/day/99
    Solution idea:

*/
// use aoc_utils::grid::Grid;

#[tracing::instrument]
pub fn part_a(input: &str) -> usize {
    // let grid = input.parse::<Grid<char>>().expect("valid grid");
    // let x =input.trim().lines().map(|l| .... )
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

#[tracing::instrument]
pub fn part_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn part_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2016_01_a(input), exepected);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(super::INPUT), 0);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn part_b_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2016_01_b(input), exepected);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
