// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/13
    Solution idea:
    For every cell, test neighbours down, right if they are same and mark as candidate with directions.
    For every candidate test for symmetry in direction till you reach bounds.
    If one symmetry test fails, what other tests can be safely skiped? 
    Is this too much premature optimization? benchmark when part 2 is known (expand by 5 along symmetry? stack layers in 3D?)
*/

use std::{str::FromStr, string::ParseError};

struct Pattern(Vec<Vec<char>>);

impl Pattern {
    fn get(&self, row: usize, col: usize) -> Option<&char> {
        self.0.get(row).and_then(|r| r.get(col))
    }

}

impl FromStr for Pattern {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Pattern(
            input
                .lines()
                .map(|l| l.trim().chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        ))
    }
}

pub fn aoc_2023_13_a(input: &str) -> usize {
    let pattern: Vec<Pattern> = input
        .trim()
        .split("\n\n")
        .map(|b| b.parse().expect("valid pattern"))
        .collect();
    0
}

pub fn aoc_2023_13_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_13_a_example() {
        assert_eq!(super::aoc_2023_13_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_13_a() {
        assert_eq!(super::aoc_2023_13_a(INPUT), 0);
    }

    #[test]
    fn aoc_2023_13_b_example() {
        assert_eq!(super::aoc_2023_13_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_13_b() {
        assert_eq!(super::aoc_2023_13_b(INPUT), 0);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
}
