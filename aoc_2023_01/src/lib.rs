// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/01
    Solution idea:

*/
use regex::Regex;

pub fn aoc_2023_01_a(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|l| {
            let digits = l
                .trim()
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<Vec<_>>();
            [digits[0], digits[digits.len() - 1]]
                .iter()
                .collect::<String>()
        })
        // .inspect(|s| println!("{}", s))
        .filter_map(|s| s.parse::<i32>().ok())
        .sum()
}

fn word_to_digit(s: &str) -> i32 {
    // eightwo should give 8 not 2, so no chain of replaces <-- This lead me to the false assumptions...
    // but "eighthree" is 83 and for "sevenine" is 79.  so plain regex is out of the window too

    // much to complicated. just parse all _overlapping_ matches with
    //  s[i..].starts_with(digit_word) (not implemented after it worked)
    // TODO: static Regex
    let re_first: Regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine|\d)").unwrap();

    let first: Vec<_> = re_first
        .find_iter(s)
        .filter_map(|m| match m.as_str() {
            "one" => Some(1),
            "two" => Some(2),
            "three" => Some(3),
            "four" => Some(4),
            "five" => Some(5),
            "six" => Some(6),
            "seven" => Some(7),
            "eight" => Some(8),
            "nine" => Some(9),
            _ => m.as_str().parse().ok(),
        })
        .collect();

    // no rfind, just reverse all
    let re_last: Regex = Regex::new(r"(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|\d)").unwrap();
    let last: Vec<_> = re_last
        .find_iter(s.chars().rev().collect::<String>().as_str())
        .filter_map(|m| match m.as_str() {
            "eno" => Some(1),
            "owt" => Some(2),
            "eerht" => Some(3),
            "ruof" => Some(4),
            "evif" => Some(5),
            "xis" => Some(6),
            "neves" => Some(7),
            "thgie" => Some(8),
            "enin" => Some(9),
            _ => m.as_str().parse().ok(),
        })
        .collect();
    // println!("{}{}: {}", first[0], last[0], s);

    first[0] * 10 + last[0]
}

pub fn aoc_2023_01_b(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|l| word_to_digit(l))
        // .inspect(|d| println!("{:?}", d))
        // .map(|digits| digits[0] * 10 + digits[digits.len() - 1])
        // .inspect(|i| assert!(*i < 100))
        // .inspect(|s| println!("{}", s))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_01_a_example() {
        assert_eq!(super::aoc_2023_01_a(TEST_INPUT), 142);
    }

    #[test]
    fn aoc_2023_01_a() {
        assert_eq!(super::aoc_2023_01_a(INPUT), 54697);
    }

    #[test]
    fn aoc_2023_01_b_example() {
        assert_eq!(super::aoc_2023_01_b(TEST_INPUT2), 281);
    }

    #[test]
    fn aoc_2023_01_b() {
        assert_eq!(super::aoc_2023_01_b(INPUT), 54885);
    }

    #[test]
    fn all_digit_words() {
        assert_eq!(
            super::aoc_2023_01_b(TEST_INPUT3),
            // wrong
            //11 + 22 + 33 + 44 + 55 + 66 + 77 + 88 + 99
            // "eighthree" is 83 and for "sevenine" is 79.
            18 + 21 + 38 + 44 + 58 + 66 + 77 + 82 + 98
        );
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";

    const TEST_INPUT2: &str = "
    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";

    const TEST_INPUT3: &str = "
    oneight
    twone
    threeight
    four
    fiveight
    six
    seven
    eightwo
    nineight
    ";
}
