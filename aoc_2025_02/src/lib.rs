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
                .unwrap_or_else(|_| panic!("invalid start of range {}", parts[0]));
            let end = parts[1]
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("invalid end of range {}", parts[1]));
            start..=end
        })
        .collect()
}

#[tracing::instrument]
pub fn aoc_2025_02_a(input: &str) -> i64 {
    let ranges = parse_ranges(input);
    ranges
        .iter()
        .flat_map(|r| r.clone())
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
        .sum()
}

#[tracing::instrument]
pub fn aoc_2025_02_b(input: &str) -> i64 {
    let ranges = parse_ranges(input);
    ranges
        .iter()
        .flat_map(|r| r.clone())
        .filter_map(|i| {
            if test_pattern(i) {
                    Some(i)
                } else {
                    None
                }
        })
        .inspect(|i| println!("Matching number: {}", i))
        .sum()
}

fn test_pattern(i: i64) -> bool {
    let s = format!("{i}");
    let len = s.len();
    let l2 =  len / 2;


    // i64.MAX is 19 digits long, so we can preallocate a vector of half that size
    let mut candidates=Vec::with_capacity(10);

    // A pattern can't be longer than half the length of the number or it could not repeat
    // phase 1: a pattern must start with the first digit, find all possible starting points
    let first_char = &s[0..1];
    for pattern_len in 1..=l2 {
        // pattern must fit an integer number of times
        if len % pattern_len != 0 {
            continue;
        }
        if &s[pattern_len..pattern_len+1] == first_char {
            
            candidates.push(pattern_len);
        }
    }
    if candidates.is_empty(){
        return false;
    }   
    // println!("candidates: {:?}", candidates);

    // phase 2: test each candidate pattern
    for candidate in candidates{
        let pattern = &s[0..candidate];

        // candidate pattern found, test repetitions from the end
        let mut repetitions = len / candidate;
        while repetitions > 1 {
            let start = candidate * (repetitions -1);
            let end = start + candidate;
            let segment = &s[start..end];
            if segment != pattern {
                break;
            }
            repetitions -=1;            
        }
        if repetitions == 1 {
            return true;
        }
    }
    // no pattern matched
    false
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
        assert_eq!(super::aoc_2025_02_a(super::INPUT), 28146997880);
    }

    #[rstest]
    #[case(TEST_INPUT, 4174379265)]
    fn aoc_2025_02_b_example(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(super::aoc_2025_02_b(input), expected);
    }

    #[test]
    fn aoc_2025_02_b() {
        assert_eq!(super::aoc_2025_02_b(super::INPUT), 40028128307);
    }

    #[rstest]
    #[case(13, false)]
    #[case(123456123456, true)]
    #[case(123456123457, false)]
    #[case(1698522, false)]
    #[case(1698528, false)]
    #[case(11, true)]
    #[case(22, true)]
    #[case(101, false)]
    #[case(111, true)]
    #[case(1212, true)]
    #[case(123123, true)]
    #[case(38593859, true)]
    #[case(565656, true)]
    #[case(824824824, true)]
    #[case(2121212121, true)]
    fn test_test_pattern(#[case] input: i64, #[case] expected: bool) {
        assert_eq!(super::test_pattern(input), expected);
    }

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
