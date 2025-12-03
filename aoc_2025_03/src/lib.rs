// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/03
    Solution idea:

*/
// use aoc_utils::grid::Grid;

fn find_max_pair(bank: &str) -> usize {
    let digits = 2;
    let mut max = 0usize;
    let mut max2 = 0usize;
    for (pos, digit) in bank.trim().chars().enumerate() {
        let val = digit.to_digit(10).unwrap() as usize;
        if val > max && pos < bank.len() + 1 - digits {
            max = val;
            // reset max2 if we found a new max
            max2 = 0;
        } else if max > 0 && val > max2 {
            max2 = val;
        }
    }
    max * 10 + max2
}

fn find_max_run(bank: &str, digits: usize) -> usize {
    let mut max_digits = vec![0usize; digits];
    for (pos, digit) in bank.trim().chars().enumerate() {
        let val = digit.to_digit(10).unwrap() as usize;

        for d in 0..digits {
            // only test subsequent digits if prior digit is set
            if d > 0 && max_digits[d - 1] == 0 {
                break;
            }

            // found a new max for this digit?
            // also ensure enough digits remain in the bank
            if val > max_digits[d] && pos < bank.len() + 1 - digits + d {
                max_digits[d] = val;
                // reset subsequent digits on new max
                // for dd in d + 1..digits {
                //     max_digits[dd] = 0;
                // }

                // clippy says to use iter_mut with take/skip instead of range
                for item in max_digits.iter_mut().skip(d + 1) {
                    *item = 0;
                }
                break;
            }
        }
    }

    // convert max_digits to a number
    max_digits.iter().fold(0, |acc, &d| acc * 10 + d)
}

#[tracing::instrument]
pub fn aoc_2025_03_a(input: &str) -> usize {
    input.trim().lines().map(find_max_pair).sum()
}

#[tracing::instrument]
pub fn aoc_2025_03_b(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|line| find_max_run(line, 12))
        .sum()
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 357)]
    fn aoc_2025_03_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_03_a(input), expected);
    }

    #[test]
    fn aoc_2025_03_a() {
        assert_eq!(super::aoc_2025_03_a(super::INPUT), 17452);
    }

    #[rstest]
    #[case(TEST_INPUT, 3121910778619)]
    fn aoc_2025_03_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_03_b(input), expected);
    }

    #[test]
    fn aoc_2025_03_b() {
        assert_eq!(super::aoc_2025_03_b(super::INPUT), 173300819005913);
    }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("330000000000000", 33)]
    fn test_find_max_pair(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::find_max_pair(input), expected);
    }

    #[rstest]
    #[case("987654321111111", 98)]
    #[case("811111111111119", 89)]
    #[case("234234234234278", 78)]
    #[case("818181911112111", 92)]
    #[case("330000000000000", 33)]
    fn test_find_max_run_2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::find_max_run(input, 2), expected);
    }

    #[rstest]
    #[case("987654321111111", 987654321111)]
    #[case("811111111111119", 811111111119)]
    #[case("234234234234278", 434234234278)]
    #[case("818181911112111", 888911112111)]
    #[case("330000000000000", 330000000000)]
    fn test_find_max_run(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::find_max_run(input, 12), expected);
    }

    const TEST_INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
