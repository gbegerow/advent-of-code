// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/06
    Solution idea:

*/
#[tracing::instrument]
pub fn aoc_2024_06_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    // input.trim().lines.map(|l| .... )
    0
}

#[tracing::instrument]
pub fn aoc_2024_06_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("X, X", 0)]
    fn aoc_2024_06_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_06_a(input), exepected);
    }

    #[test]
    fn aoc_2024_06_a() {
        assert_eq!(super::aoc_2024_06_a(super::INPUT), 0);
    }

    #[rstest]
    #[case("X, X", 0)]
    fn aoc_2024_06_b_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_06_b(input), exepected);
    }

    #[test]
    fn aoc_2024_06_b() {
        assert_eq!(super::aoc_2024_06_b(super::INPUT), 0);
    }
}
