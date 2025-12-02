// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/02
    Solution idea:

*/
use std::ops::RangeInclusive;

fn parse_ranges(input: &str) -> Vec<RangeInclusive<i64>> {
    input
        .trim()
        .split(",")
        .map(|l| {
            let parts: Vec<&str> = l.trim().split('-').collect();
            if parts.len() != 2 {
                panic!("invalid range: {}", l);
            }
            let start = parts[0]
                .parse::<i64>()
                .expect(&format!("invalid start of range {}", parts[0]));
            let end = parts[1]
                .parse::<i64>()
                .expect(&format!("invalid end of range {}", parts[1]));
            start..=end
        })
        .collect()
}

#[tracing::instrument]
pub fn aoc_2025_02_a(input: &str) -> i64 {
    let ranges = parse_ranges(input);
    let x = ranges
        .iter()
        .flat_map(|r| r.clone().into_iter())
        .filter_map(|i| {
            let s = format!("{i}");
            if s.len() % 2 == 0 {
                let half = s.len() / 2;
                let first_half = &s[..half];
                let second_half = &s[half..];
                let halfes_matches = if first_half == second_half {
                    Some(i)
                } else {
                    None
                };
                // println!(
                //     "i: {}, first_half: {}, second_half: {}, halfes_matches: {:?}",
                //     i, first_half, second_half, halfes_matches
                // );

                halfes_matches
            } else {
                None
            }
        })
        .inspect(|i| println!("Matching number: {}", i))
        .sum();

    x
}

#[tracing::instrument]
pub fn aoc_2025_02_b(_input: &str) -> i64 {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 1227775554)]
    fn aoc_2025_02_a_example(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(super::aoc_2025_02_a(input), expected);
    }

    #[test]
    fn aoc_2025_02_a() {
        assert_eq!(super::aoc_2025_02_a(super::INPUT), 0);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2025_02_b_example(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(super::aoc_2025_02_b(input), expected);
    }

    #[test]
    fn aoc_2025_02_b() {
        assert_eq!(super::aoc_2025_02_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
