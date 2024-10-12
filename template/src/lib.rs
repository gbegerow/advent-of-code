// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/9999/day/99
    Solution idea:

*/

pub fn part_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn part_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("X, X", 0)]
    fn part_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2016_01_a(input), exepected);
    }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(INPUT), 0);
    }

    #[test]
    fn part_b_example() {
        assert_eq!(super::part_b(TEST_INPUT), 0);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(INPUT), 0);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}
