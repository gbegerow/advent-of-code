// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/12
    Solution idea:
    linear equations with 2 unknown
*/
use aoc_utils::grid::Grid;

#[tracing::instrument]
pub fn aoc_2024_12_a(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().expect("valid grid");

    println!("{}", grid);
    0
}

#[tracing::instrument]
pub fn aoc_2024_12_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 0)]
    #[case(TEST_INPUT_2, 0)]
    #[case(TEST_INPUT_3, 0)]
    fn aoc_2024_12_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_12_a(input), expected);
    }

    #[test]
    fn aoc_2024_12_a() {
        assert_eq!(super::aoc_2024_12_a(super::INPUT), 0);
    }

    #[rstest]
    #[case("X, X", 0)]
    fn aoc_2024_12_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_12_b(input), expected);
    }

    #[test]
    fn aoc_2024_12_b() {
        assert_eq!(super::aoc_2024_12_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "
        AAAA
        BBCD
        BBCC
        EEEC";

    const TEST_INPUT_2: &str = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_INPUT_3: &str = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
}
