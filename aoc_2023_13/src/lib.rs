// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/13
    Solution idea:
    For every cell, test neighbours down, right if they are same and mark as candidate with directions.
    For every candidate test for symmetry in direction till you reach bounds or find differnence.
    Test next row/column while there are candidates.
    Is this too much premature optimization? benchmark when part 2 is known (expand by 5 along symmetry? stack layers in 3D?)
*/

use std::{str::FromStr, string::ParseError};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Vertical,
    Horizontal,
}

struct Pattern(Vec<Vec<u8>>);

#[derive(Debug, Clone)]
struct Candidate {
    // position in major direction
    major: usize,
    // position in minor direction
    minor: usize,
    defects: u16,
    direction: Direction,
}

impl Pattern {
    fn get(&self, row: usize, col: usize) -> Option<&u8> {
        self.0.get(row).and_then(|r| r.get(col))
    }

    fn find_symmetry(&self, threshold: u16) -> Vec<Candidate> {
        let (width, height) = (self.0[0].len(), self.0.len());
        let mut symmetries = Vec::with_capacity(width + height);

        // all rows and columns except the outermost are candidates for symmetry
        // we just overengeneer it a little bit :-)
        let mut candidates = (1..height - 1)
            .map(|row| Candidate {
                major: row,
                minor: 1,
                defects: 0,
                direction: Direction::Vertical,
            })
            .chain((1..width - 1).map(|col| Candidate {
                major: col,
                minor: 1,
                defects: 0,
                direction: Direction::Horizontal,
            }))
            .collect::<Vec<_>>();

        // test candidates for symmetry
        while let Some(Candidate {
            major,
            minor,
            mut defects,
            direction,
        }) = candidates.pop()
        {
            // do we already know it is not a symmetrie?
            if defects > threshold {
                continue;
            }

            // are we done with this candidate?
            if match direction {
                Direction::Vertical => minor == width,
                Direction::Horizontal => minor == height,
            } {
                symmetries.push(Candidate {
                    major,
                    minor,
                    defects,
                    direction,
                });
                continue;
            }

            // test from our current point to both sides if they pointwise equal 
            // offset from start to test now
            let mut offset = 0;

            while defects <= threshold {
                let (before, after) = match direction {
                    Direction::Vertical if (major >= offset) => (
                        self.get(major - offset, minor),
                        self.get(major + 1 + offset, minor),
                    ),
                    Direction::Horizontal if (major >= offset) => (
                        self.get(minor, major - offset),
                        self.get(minor, major + 1 + offset),
                    ),
                    _ => (None, None),
                };

                match (before, after) {
                    // both are different, symmetry broken
                    (Some(b), Some(a)) if b != a => {
                        defects += 1;
                    }

                    // both are same, continue testing
                    (Some(_), Some(_)) => (),

                    // reached bounds (None,_) , (_,None) no sense in going further
                    _ => {
                        println!("{:?} {:?}/{:?}: {:?} Defects Offset {:?}", direction, major, minor, defects, offset);
                            // at least 1 test must be successfull
                        if defects <= threshold {
                            // schedule next point in minor direction for test
                            candidates.push(Candidate {
                                major,
                                minor: minor + 1,
                                defects,
                                direction,
                            });
                        }
                        break;
                    }
                }
                offset += 1;
            }
        }

        symmetries
    }
}

impl FromStr for Pattern {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Pattern(
            input
                .lines()
                .map(|l| l.trim().as_bytes().into())
                .collect::<Vec<_>>(),
        ))
    }
}

fn score(symmetries: Vec<Candidate>) -> usize {
    symmetries
        .iter()
        .map(|s| match s.direction {
            Direction::Vertical => s.major,
            Direction::Horizontal => s.major * 100,
        })
        .sum()
}

pub fn aoc_2023_13_a(input: &str) -> usize {
    let pattern: Vec<Pattern> = input
        .trim()
        .split("\n\n")
        .map(|b| b.parse().expect("valid pattern"))
        .collect();

    let symmetries = dbg!(pattern
        .iter()
        .map(|p| p.find_symmetry(0))
        .collect::<Vec<_>>());
    symmetries.into_iter().map(|s| score(s)).sum()
}

pub fn aoc_2023_13_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_13_a_example() {
        assert_eq!(super::aoc_2023_13_a(TEST_INPUT), 405);
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
