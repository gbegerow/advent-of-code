use std::{ops::RangeInclusive, str::FromStr};
// use winnow::prelude::*;
// use winnow::Result;
// use winnow::ascii::dec_uint;
// use winnow::ascii::multispace1;

// use winnow::combinator::separated;
// use winnow::combinator::separated_pair;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/05
    Solution idea:

*/

#[derive(Debug)]
struct Ingredients {
    ranges: Vec<RangeInclusive<i64>>,
    available: Vec<i64>,
}

impl FromStr for Ingredients {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse_ingredients.parse(s).map_err(|e| e.to_string())

        let parts: Vec<&str> = s.trim().split("\n\n").collect();
        debug_assert!(parts.len() == 2);

        let ranges = parts[0]
            .lines()
            .map(|l| {
                let parts: Vec<&str> = l.trim().split('-').collect();
                if parts.len() != 2 {
                    return Err(format!("invalid range: {}", l));
                }
                let start = parts[0].parse::<i64>().map_err(|e| e.to_string())?;
                let end = parts[1].parse::<i64>().map_err(|e| e.to_string())?;
                Ok(start..=end)
            })
            .collect::<Result<Vec<RangeInclusive<i64>>, String>>()?;

        let available = parts[1]
            .lines()
            .map(|l| l.trim().parse::<i64>().map_err(|e| e.to_string()))
            .collect::<Result<Vec<i64>, String>>()?;

        Ok(Ingredients { ranges, available })
    }
}

// fn parse_range(input: &str) -> Result<RangeInclusive<i64>> {
//     separated_pair(
//         dec_uint::<i64>,
//         winnow::ascii::char('-'),
//         dec_uint::<i64>,
//     ).map(|(start, end)| RangeInclusive::new(start, end))
// }

// fn parse_ranges(input: &str) -> Result<Vec<RangeInclusive<i64>>> {
//     separated(1.., parse_range, multispace1
//     ).map(|(start, end)| RangeInclusive::new(start, end))
// }

// fn parse_ids(input: &str) -> Result<Vec<i64>> {
//     separated(1.., dec_uint::<i64>, multispace1)
//     .map(map)
// }

// fn parse_ingredients(input: &str) -> Result<Ingredients> {
//     separated_pair(parse_ranges, "\n\n", parse_ids)
//     .map(|ranges, available| Ingredients { ranges, available })
// }

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
    let mut sorted_ranges: Vec<RangeInclusive<i64>> = ingredients.ranges.to_vec();
    sorted_ranges.sort_by_key(|r| *r.start());

    // merge overlapping ranges to reduce number of checks and count without double counting
    let merged_ranges: Vec<RangeInclusive<i64>> =
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
    // let set: HashSet<i64> = merged_ranges
    //     .iter()
    //     .flat_map(|r| r.clone().into_iter())
    //     .inspect(|i| println!("{i}"))
    //     .collect();

    // Ok(set.len())

    // do not iterate over all ranges for each available id, just add the lengths of the merged ranges (to avoid double counting)
    // remember these are inclusive ranges
    let total_count: usize = merged_ranges
        .iter()
        .map(|r| (r.end() - r.start() + 1) as usize)
        .sum();
    Ok(total_count)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

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
