// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/01
    Solution idea:

*/

use std::collections::hash_map::*;

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let (mut left, mut right) =
        input
            .trim()
            .lines()
            .fold((Vec::new(), Vec::new()), |mut accu, l| {
                if let [Ok(a), Ok(b)] = l
                    .trim()
                    .split_ascii_whitespace()
                    .map(|s| s.parse::<i32>())
                    .collect::<Vec<_>>()
                    .as_slice()
                {
                    accu.0.push(*a);
                    accu.1.push(*b);
                }
                accu
            });

    left.sort();
    right.sort();

    (left, right)
}

fn histogram(v: Vec<i32>) -> HashMap<i32, i32> {
    let mut map = HashMap::with_capacity(v.len());
    for i in v {
        *map.entry(i).or_insert(0) += 1;
    }
    map
}

#[tracing::instrument]
pub fn aoc_2024_01_a(input: &str) -> i32 {
    let (left, right) = parse(input);

    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

#[tracing::instrument]
pub fn aoc_2024_01_b(input: &str) -> i32 {
    let (left, right) = parse(input);

    let left_hist = histogram(left);
    let right_hist = histogram(right);

    left_hist.iter().fold(0, |accu, (i, n)| {
        accu + right_hist.get(i).unwrap_or(&0) * n * i
    })
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 11)]
    fn aoc_2024_01_a_example(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(super::aoc_2024_01_a(input), expected);
    }

    #[test]
    fn aoc_2024_01_a() {
        assert_eq!(super::aoc_2024_01_a(super::INPUT), 1506483);
    }

    #[rstest]
    #[case(TEST_INPUT, 31)]
    fn aoc_2024_01_b_example(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(super::aoc_2024_01_b(input), expected);
    }

    #[test]
    fn aoc_2024_01_b() {
        assert_eq!(super::aoc_2024_01_b(super::INPUT), 23126924);
    }

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";
}
