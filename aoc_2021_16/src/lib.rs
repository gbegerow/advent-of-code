// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2021/day/16
    Solution idea:
    use Nom to build a bitwise parser on bytes s. https://blog.adamchalmers.com/nom-bits/
*/
//use num_bigint::BigUint;
// use bitvec::prelude::*;
use nom::{bits::complete::take, IResult};
type BitInput<'a> = (&'a [u8], usize);

const Literal_ID: u8 = 4;

enum Package {
    Literal {
        version: u8,
        type_id: u8, // literal is always type 4
        value: u64,
    },
    Operator {
        version: u8,
        type_id: u8,
        packages: Vec<Package>,
    },
}



pub fn aoc_2021_16_a(input: &str) -> usize {
    let package_def = input
        .trim()
        .chars()
        .filter_map(|h| u8::from_str_radix(&h.to_string(), 16).ok())
        .collect::<Vec<_>>();


    0
}

pub fn aoc_2021_16_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_16_a_example() {
        assert_eq!(super::aoc_2021_16_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2021_16_a() {
        assert_eq!(super::aoc_2021_16_a(INPUT), 0);
    }

    #[test]
    fn aoc_2021_16_b_example() {
        assert_eq!(super::aoc_2021_16_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2021_16_b() {
        assert_eq!(super::aoc_2021_16_b(INPUT), 0);
    }

    // #[test]
    // fn literal_D2FE28() {
    //     assert_eq!(super::literal("D2FE28"), 2021)
    // }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}
