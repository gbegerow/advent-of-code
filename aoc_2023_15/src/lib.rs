use std::{str::FromStr, string::ParseError};

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/15
    Solution idea:
    It is a hashtable with collision of BTreeMaps (Order must not be changing)
*/
use regex::Regex;

enum Op<'a> {
    Assign(&'a str, u32),
    Remove(&'a str),
}

const OP_RX: Regex = Regex::new( r"(\w+)((\=\d+)|\-)");
impl FromStr for Op<'a> {
    type Err=ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
    }
}

fn hash(s: &str) -> u32 {
    s.trim()
        .as_bytes()
        .into_iter()
        .fold(0u32, |acc, c| ((acc + *c as u32) * 17) % 256)
}

pub fn aoc_2023_15_a(input: &str) -> u32 {
    input.trim().split(',').map(|s| hash(s)).sum()
}

pub fn aoc_2023_15_b(input: &str) -> u32 {


    input.trim().split(',').map(|s| )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn aoc_2023_15_a_example() {
        assert_eq!(super::aoc_2023_15_a(TEST_INPUT), 1320);
    }

    #[test]
    fn aoc_2023_15_a() {
        assert_eq!(super::aoc_2023_15_a(INPUT), 517965);
    }

    #[test]
    fn aoc_2023_15_b_example() {
        assert_eq!(super::aoc_2023_15_b(TEST_INPUT), 145);
    }

    #[test]
    fn aoc_2023_15_b() {
        assert_eq!(super::aoc_2023_15_b(INPUT), 0);
    }

    #[rstest]
    #[case("HASH", 52)]
    #[case("rn=1", 30)]
    #[case("cm-", 253)]
    #[case("qp=3", 97)]
    #[case("cm=2", 47)]
    #[case("qp-", 14)]
    #[case("pc=4", 180)]
    #[case("ot=9", 9)]
    #[case("ab=5", 197)]
    #[case("pc-", 48)]
    #[case("pc=6", 214)]
    #[case("ot=7", 231)]
    fn hash_should_give(#[case] s: &str, #[case] expected: u32) {
        assert_eq!(super::hash(s), expected);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
}
