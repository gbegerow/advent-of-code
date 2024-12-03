// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/03
    Solution idea:

*/

use regex::Regex;
// use tracing;

#[tracing::instrument(skip(input))]
pub fn aoc_2024_03_a(input: &str) -> i64 {
    let rx = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut res: i64 = 0;
    for (_, [astr, bstr]) in rx.captures_iter(input).map(|c| c.extract()) {
        // println!("as: {astr} bs: {bstr}");
        let a: i64 = astr.parse().expect("numbers only");
        let b: i64 = bstr.parse().expect("numbers only");
        // println!("a: {a} b: {b}");

        res += a * b;
    }

    res
}

// #[tracing::instrument]
pub fn aoc_2024_03_b(input: &str) -> i64 {
    // part b needs a context sensitive language, no more regex
    if input.len() < 4 {
        return 0;
    };

    let rx = Regex::new(r"mul\((\d+),(\d+)\)$").unwrap();

    let mut res = 0i64;
    let mut enabled = true; // mul is enabled at start

    // scan for do(), don't(), mul(a,b)
    // all valid expression end with ). Look for ) and test slice bevor
    let mut closing = 3; // earliest valid position for ')'
    while let Some(next) = input[closing..].find(')') {
        closing = closing + next + 1; // next is offset relativ to closing

        // println!(
        //     "Closing @{} enabled {} '{}'",
        //     closing,
        //     enabled,
        //     &input[closing.saturating_sub(12)..closing]
        // );
        if input[..closing].ends_with("do()") {
            enabled = true;
        } else if input[..closing].ends_with("don't()") {
            enabled = false;
        }

        if enabled {
            // println!(
            //     "test for mul {}",
            //     &input[closing.saturating_sub(10)..closing]
            // );
            // too lazy for optimized parser
            for (_, [astr, bstr]) in rx.captures_iter(&input[..closing]).map(|c| c.extract()) {
                // println!("as: {astr} bs: {bstr}");
                let a: i64 = astr.parse().expect("numbers only");
                let b: i64 = bstr.parse().expect("numbers only");

                res += a * b;
            }
        }
    }
    res
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn aoc_2024_03_a_example() {
        assert_eq!(super::aoc_2024_03_a(TEST_INPUT), 161);
    }

    #[test]
    fn aoc_2024_03_a() {
        assert_eq!(super::aoc_2024_03_a(super::INPUT), 153469856);
    }

    #[rstest]
    #[case(TEST_INPUT, 161)]
    #[case(TEST_INPUT2, 48)]
    fn aoc_2024_03_b_example(#[case] input: &str, #[case] exepected: i64) {
        assert_eq!(super::aoc_2024_03_b(input), exepected);
    }

    #[test]
    fn aoc_2024_03_b() {
        assert_eq!(super::aoc_2024_03_b(super::INPUT), 77055967);
    }

    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
}
