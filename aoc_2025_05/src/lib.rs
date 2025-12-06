use std::{ops::RangeInclusive, str::FromStr};

use winnow::Result;
use winnow::ascii::dec_uint;
use winnow::ascii::line_ending;
use winnow::ascii::multispace0;
use winnow::ascii::multispace1;
use winnow::combinator::opt;
use winnow::error::ContextError;
use winnow::prelude::*;

use winnow::combinator::separated;
use winnow::combinator::separated_pair;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/05
    Solution idea:

*/

#[derive(Debug)]
struct Ingredients {
    ranges: Vec<RangeInclusive<u64>>,
    available: Vec<u64>,
}

impl FromStr for Ingredients {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_ingredients.parse(s).map_err(|e| e.to_string())
    }

    // // iterator parsing version
    // fn from_str(s: &str) -> Result<Self, Self::Err> {
    //     let parts: Vec<&str> = s.trim().split("\n\n").collect();
    //     debug_assert!(parts.len() == 2);

    //     let ranges = parts[0]
    //         .lines()
    //         .map(|l| {
    //             let parts: Vec<&str> = l.trim().split('-').collect();
    //             if parts.len() != 2 {
    //                 return Err(format!("invalid range: {}", l));
    //             }
    //             let start = parts[0].parse::<u64>().map_err(|e| e.to_string())?;
    //             let end = parts[1].parse::<u64>().map_err(|e| e.to_string())?;
    //             Ok(start..=end)
    //         })
    //         .collect::<Result<Vec<RangeInclusive<u64>>, String>>()?;

    //     let available = parts[1]
    //         .lines()
    //         .map(|l| l.trim().parse::<u64>().map_err(|e| e.to_string()))
    //         .collect::<Result<Vec<u64>, String>>()?;

    //     Ok(Ingredients { ranges, available })
    // }
}

// winnow parser implementation
type Stream<'i> = &'i str;

fn parse_range<'s>(i: &mut Stream<'s>) -> Result<RangeInclusive<u64>> {
    separated_pair(dec_uint, "-", dec_uint)
        .parse_next(i)
        .map(|(start, end)| RangeInclusive::new(start, end))
}

fn parse_ranges<'s>(i: &mut Stream<'s>) -> Result<Vec<RangeInclusive<u64>>> {
    separated(1.., parse_range, line_ending).parse_next(i)
}

fn parse_ids<'s>(i: &mut Stream<'s>) -> Result<Vec<u64>> {
    separated(1.., dec_uint::<Stream<'s>, u64, ContextError>, line_ending)
    .parse_next(i)
}

fn parse_ingredients<'s>(i: &mut Stream<'s>) -> Result<Ingredients> {
    (   // allow leading/trailing whitespace
        opt(multispace0),
        separated_pair(parse_ranges, multispace1, parse_ids),
        opt(multispace0),
    )
        .parse_next(i)
        .map(|(_, (ranges, available), _)| Ingredients { ranges, available })
}

#[tracing::instrument]
pub fn aoc_2025_05_a(input: &str) -> Result<usize, String> {
    let ingredients: Ingredients = input.parse()?;

    let mut count = 0;
    for id in &ingredients.available {
        let mut valid = false;
        for range in &ingredients.ranges {
            if range.contains(id) {
                valid = true;
                count += 1;
                break;
            }
        }
        if !valid {
            println!("Invalid ingredient id: {}", id);
        }
    }

    Ok(count)
}

#[tracing::instrument]
pub fn aoc_2025_05_b(input: &str) -> Result<usize, String> {
    let ingredients: Ingredients = input.parse()?;

    // sort ranges by start so we don'T have to check the cardesian product of all ranges
    let mut sorted_ranges: Vec<RangeInclusive<u64>> = ingredients.ranges.to_vec();
    sorted_ranges.sort_by_key(|r| *r.start());

    // merge overlapping ranges to reduce number of checks and count without double counting
    let merged_ranges: Vec<RangeInclusive<u64>> =
        sorted_ranges.into_iter().fold(Vec::new(), |mut acc, r| {
            // try to merge with last range which is enough as ranges are sorted now
            if let Some(last) = acc.last_mut()
                && *r.start() <= *last.end() + 1
            {
                // overlapping or contiguous ranges, merge them
                let new_end = (*last.end()).max(*r.end());
                *last = *last.start()..=new_end;
                return acc;
            }
            acc.push(r);
            acc
        });
    // println!("Merged ranges: {} {:?}", merged_ranges.len(), merged_ranges);

    // Naive approach, iterate over all available ids and check if they are in any range
    // Much too many iterations for input (as to be expected in AoC)
    // let set: HashSet<u64> = merged_ranges
    //     .iter()
    //     .flat_map(|r| r.clone().into_iter())
    //     .inspect(|i| println!("{i}"))
    //     .collect();

    // Ok(set.len())

    // do not iterate over all ranges for each available id, just add the lengths of the merged (to avoid double counting) ranges
    // remember these are inclusive ranges
    // never ever expand ranges in AoC!
    let total_count: usize = merged_ranges
        .iter()
        .map(|r| (r.end() - r.start() + 1) as usize)
        .sum();
    Ok(total_count)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use std::ops::RangeInclusive;

    use rstest::rstest;
    use winnow::error::ContextError;

    #[rstest]
    #[case(TEST_INPUT, 3)]
    fn aoc_2025_05_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_05_a(input), Ok(expected));
    }

    #[test]
    fn aoc_2025_05_a() {
        assert_eq!(super::aoc_2025_05_a(super::INPUT), Ok(744));
    }

    #[rstest]
    #[case(TEST_INPUT, 14)]
    fn aoc_2025_05_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_05_b(input), Ok(expected));
    }

    #[test]
    fn aoc_2025_05_b() {
        assert_eq!(super::aoc_2025_05_b(super::INPUT), Ok(347468726696961));
    }

    #[rstest]
    #[case("10-14", Ok(RangeInclusive::new(10, 14)))]
    #[case("10-14\n", Ok(RangeInclusive::new(10, 14)))]
    fn parse_range_example(
        #[case] input: &str,
        #[case] expected: Result<RangeInclusive<u64>, ContextError>,
    ) {
        let mut input = input;
        let result = super::parse_range(&mut input);
        assert_eq!(result, expected);
    }

    #[test]
    fn parse_ranges_example() {
        let mut input = "1-3\n5-7\n10-14\n";
        let expected = Ok(vec![
            RangeInclusive::new(1, 3),
            RangeInclusive::new(5, 7),
            RangeInclusive::new(10, 14),
        ]);
        let result = super::parse_ranges(&mut input);
        assert_eq!(result, expected);
    }

    const TEST_INPUT: &str = "
3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
