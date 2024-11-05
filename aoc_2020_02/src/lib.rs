// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2020/day/02
    Solution idea:

*/
use regex::Regex;

pub fn aoc_2020_02_a(input: &str) -> usize {
    let re: Regex =
        Regex::new(r"^\s*(?P<min>\d)\s*-\s*(?P<max>\d)\s+(?P<letter>\w): (?P<pwd>\w+)\s*$")
            .unwrap();

    // 1-3 a: abcde
    input
        .trim()
        .lines()
        .flat_map(|s| re.captures(s))
        .flat_map(|&caps| {
            let (min, max, letter, pwd) = (
                caps["min"].parse::<usize>().unwrap(),
                caps["max"].parse::<usize>().unwrap(),
                caps["letter"].chars().next(),
                caps["pwd"],
            );
            let letter_count = pwd.chars().filter_map(|c| c == letter).count();

            if letter_count < min || letter_count > max {
                0
            } else {
                1
            }
        })
        .sum()
}

pub fn aoc_2020_02_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2020_02_a_example() {
        assert_eq!(super::aoc_2020_02_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2020_02_a() {
        assert_eq!(super::aoc_2020_02_a(INPUT), 0);
    }

    #[test]
    fn aoc_2020_02_b_example() {
        assert_eq!(super::aoc_2020_02_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2020_02_b() {
        assert_eq!(super::aoc_2020_02_b(INPUT), 0);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}
