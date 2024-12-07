use std::str::FromStr;

use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space1, u64},
    error::Error,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, IResult,
};
// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/07
    Solution idea:

*/

#[derive(Debug)]
enum Op {
    Add,
    Multiplication,
}
#[derive(Clone, Copy, PartialEq, Eq)]
struct OpCombination {
    combination: u64,
    // len in bits max 64
    len: usize,
}

impl std::fmt::Debug for OpCombination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut bit_count = 0;
        while bit_count < self.len {
            let bit = (self.combination >> bit_count) & 1;
            bit_count += 1;
            match bit {
                0 => write!(f, "+")?,
                1 => write!(f, "*")?,
                _ => unreachable!("bit not 0 or 1"),
            };
        }
        Ok(())

        // write!(f, " ({:0width$b})", self.combination, width = self.len)
    }
}

impl OpCombination {
    fn new(op_len: usize) -> Self {
        Self {
            combination: 0,
            len: op_len,
        }
    }

    fn iter(&self) -> OpIterator {
        OpIterator {
            current: 0,
            combination: self.combination,
            len: self.len,
        }
    }
}

struct OpIterator {
    current: usize,
    combination: u64,
    // len in bits max 64
    len: usize,
}

impl Iterator for OpIterator {
    type Item = Op;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.len {
            let bit = (self.combination >> self.current) & 1;
            self.current += 1;

            match bit {
                0 => Some(Op::Add),
                1 => Some(Op::Multiplication),
                _ => unreachable!("Bit can't be anything other than 0 or 1"),
            }
        } else {
            None
        }
    }
}

struct OpCombinationIterator(OpCombination);
/// iterate over all combinations
impl Iterator for OpCombinationIterator {
    type Item = OpCombination;

    fn next(&mut self) -> Option<Self::Item> {
        let nx = self.0.combination;

        if nx < (1 << self.0.len) {
            self.0.combination += 1;
            Some(OpCombination {
                combination: nx,
                len: self.0.len,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Eqation {
    result: u64,
    numbers: Vec<u64>,
}

impl Eqation {
    fn combinations(&self) -> OpCombinationIterator {
        OpCombinationIterator(OpCombination::new(self.numbers.len() - 1))
    }

    fn eval(&self, oc: OpCombination) -> u64 {
        let evaluated = self.numbers.iter().skip(1).zip(oc.iter()).fold(
            self.numbers[0],
            |val, (v, op)| match op {
                Op::Add => val + v,
                Op::Multiplication => val * v,
            },
        );

        // println!("{} == {}", self.format(oc), evaluated);

        evaluated
    }

    fn probe_ops(&self) -> Vec<OpCombination> {
        self.combinations()
            .map(|oc| (oc, self.eval(oc)))
            .filter(|(_oc, val)| self.result == *val)
            .inspect(|(oc, val)| println!("{} == {}", self.format(*oc), val))
            .map(|(oc, _)| oc)
            .collect()
    }

    #[allow(dead_code)]
    fn format(&self, ops: OpCombination) -> String {
        let op_str = format!("{:?}", ops);
        let mixed = self
            .numbers
            .iter()
            .skip(1)
            .zip(op_str.chars())
            .map(|(n, op)| format!(" {} {}", op, n))
            .collect::<String>();
        format!("{} = {}{}", self.result, self.numbers[0], mixed)
    }
}

// sample for FromStr with nom: https://docs.rs/nom/latest/nom/recipes/index.html#implementing-fromstr
impl FromStr for Eqation {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_equation(s).finish() {
            Ok((_, equation)) => Ok(equation),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

fn parse_equation(input: &str) -> IResult<&str, Eqation> {
    let (rest, (result, numbers)) =
        separated_pair(u64, tag(": "), separated_list1(space1, u64))(input)?;

    Ok((rest, Eqation { result, numbers }))
}

fn parse(input: &str) -> IResult<&str, Vec<Eqation>> {
    separated_list1(line_ending, parse_equation)(input)
}

#[tracing::instrument(skip(input))]
pub fn aoc_2024_07_a(input: &str) -> u64 {
    let (_, eqations) = parse(input).expect("should be valid input");

    eqations
        .iter()
        .map(|e| (e, e.probe_ops()))
        .filter(|(_e, combinations)| !combinations.is_empty())
        .map(|(e, _)| e.result)
        .sum()
}

#[tracing::instrument]
pub fn aoc_2024_07_b(input: &str) -> u64 {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{Eqation, OpCombination, OpCombinationIterator};

    #[test]
    fn show_combinatons() {
        let iter = OpCombinationIterator(OpCombination {
            combination: 0,
            len: 3,
        });

        let vec = iter
            .map(|oc| format!("{:?}:{:08b}", oc, oc.combination))
            .collect::<Vec<_>>();
        println!("{:?}", vec);

        println!(
            "4 ops: {:?}",
            OpCombination {
                combination: 0b1111,
                len: 4,
            }
        );

        // assert!(false);
    }

    #[rstest]
    #[case("5: 1 1 1 1 1", 0b00010)]
    #[case("27: 3 3 3", 0b111)]
    fn combinations_is_full_range(#[case] input: &str, #[case] exepected: u64) {
        let sut = input.parse::<Eqation>().expect("invalid input");

        let probes = sut.probe_ops();

        println!(
            "{:08b} {} {}",
            probes[0].combination,
            probes[0].len,
            sut.numbers.len() - 1
        );

        assert_eq!(
            probes,
            vec![OpCombination {
                combination: exepected,
                len: sut.numbers.len() - 1
            }]
        );
    }

    #[rstest]
    #[case(TEST_INPUT, 3749)]
    fn aoc_2024_07_a_example(#[case] input: &str, #[case] exepected: u64) {
        assert_eq!(super::aoc_2024_07_a(input), exepected);
    }

    #[test]
    fn aoc_2024_07_a() {
        assert_eq!(super::aoc_2024_07_a(super::INPUT), 975671981569);
    }

    #[rstest]
    #[case(TEST_INPUT, 11387)]
    fn aoc_2024_07_b_example(#[case] input: &str, #[case] exepected: u64) {
        assert_eq!(super::aoc_2024_07_b(input), exepected);
    }

    #[test]
    fn aoc_2024_07_b() {
        assert_eq!(super::aoc_2024_07_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
}
