// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/02
    Solution idea:

*/

pub fn aoc_2016_02_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn aoc_2016_02_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("X, X", 0)]
    fn aoc_2016_02_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2016_02_a(input), exepected);
    }

    #[test]
    fn aoc_2016_02_a() {
        assert_eq!(super::aoc_2016_02_a(INPUT), 0);
    }

    #[test]
    fn aoc_2016_02_b_example() {
        assert_eq!(super::aoc_2016_02_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2016_02_b() {
        assert_eq!(super::aoc_2016_02_b(INPUT), 0);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}
