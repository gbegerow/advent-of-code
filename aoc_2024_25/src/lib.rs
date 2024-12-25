// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/25
    Solution idea:

*/

use std::iter::once;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum ScanMode {
    #[default]
    LookingForPart,
    Lock,
    Key,
    SkipFirstRow,
    Finish,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PartType {
    Lock,
    Key,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    // do we need an id?
    typ: PartType,
    heights: [u8; 5],
    // do we need a bit pattern? can easily be reconstructed from heightes
}

// schematics are 5 cols x 7 rows, first and last row does not count for height
const MAX_HEIGHT: u8 = 6;

fn parse(input: &str) -> (Vec<Part>, Vec<Part>) {
    // keep track of keys and locks so we can enumerate them separately
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    // state driven parser
    let mut mode = ScanMode::LookingForPart;
    let mut row = 0;
    let mut col = 0;
    let mut heights = [0; 5];
    let mut typ = PartType::Key;

    // add newline as guard, so we always finish
    for c in input.chars().chain(once('\n')) {
        if c == '\n' {
            println!("Row: {row} Mode: {mode:?}")
        }

        if col == 2{
            println!("Row: {row} Col: {col} Char: '{c}' Mode: {mode:?}")
        }

        match (mode, c) {
            (ScanMode::LookingForPart, '#') => {
                // found a lock
                mode = ScanMode::SkipFirstRow;
                typ = PartType::Lock;
            }

            (ScanMode::LookingForPart, '.') => {
                // found a key
                mode = ScanMode::SkipFirstRow;
                typ = PartType::Key;
            }

            (ScanMode::SkipFirstRow, '\n') => {
                mode = match typ {
                    PartType::Lock => ScanMode::Lock,
                    PartType::Key => ScanMode::Key,
                };
                row = 0; 
                // reinit heights or 0 will get value from last part
                heights.fill(0);
            }

            (ScanMode::Lock, '#') => {
                heights[col] = row;
            }

            (ScanMode::Key, '#') => {
                if heights[col] == 0 {
                    heights[col] = MAX_HEIGHT - row;
                }
            }

            (ScanMode::Finish, '\n') => {
                // Pattern ended
                let part = Part {
                    typ,
                    heights: heights.clone(),
                };

                match typ {
                    PartType::Lock => locks.push(part),
                    PartType::Key => keys.push(part),
                }

                mode = ScanMode::LookingForPart;
            }

            _ => { /* do nothing for '.' or if skipping or looking for a part */ }
        }

        // advance counters
        if c == '\n' {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }

        // Skip scanning last row
        if row == MAX_HEIGHT {
            mode = ScanMode::Finish;
        }
    }

    (locks, keys)
}

#[tracing::instrument]
pub fn aoc_2024_25_a(input: &str) -> usize {
    let (locks, keys) = parse(input);
    println!("Locks: {locks:?}\nKeys: {keys:?}");

    // general form of cross product:  let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
    locks.iter().flat_map(|lock| keys.iter()
    .map(move |key| (lock, key)))
    .filter(|(lock, key)| 
        lock.heights.iter().zip(key.heights.iter())
            .all(|(h1, h2)| h1 + h2 < MAX_HEIGHT))
    .count()
}

#[tracing::instrument]
pub fn aoc_2024_25_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    // one lock
    #[case("#####
.####
.####
.####
.#.#.
.#...
.....", (vec![Part{typ: PartType::Lock,heights:[0,5,3,4,3]}], vec![]))]

    // one other lock
    #[case("
    
#####
##.##
.#.##
...##
...#.
...#.
.....", (vec![Part{typ: PartType::Lock,heights:[1,2,0,5,3]}], vec![]))]

// one key
#[case(".....
#....
#....
#...#
#.#.#
#.###
#####", ( vec![], vec![Part{typ: PartType::Key,heights:[5,0,2,1,3]}]))]

// same lock twice
#[case("#####
.####
.####
.####
.#.#.
.#...
.....

#####
.####
.####
.####
.#.#.
.#...
.....", (vec![
    Part{typ: PartType::Lock,heights:[0,5,3,4,3]},
    Part{typ: PartType::Lock,heights:[0,5,3,4,3]},
    ], vec![]))]

    // two different locks. Why does this fail? because height in second comes from first if 0
    #[case("#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....", (vec![
        Part{typ: PartType::Lock,heights:[0,5,3,4,3]},
        Part{typ: PartType::Lock,heights:[1,2,0,5,3]},
        ], vec![]))]
    
#[case(TEST_INPUT, 
    (
        vec![
            Part{typ: PartType::Lock,heights:[0,5,3,4,3]},
            Part{typ: PartType::Lock,heights:[1,2,0,5,3]},
            ],
        vec![
            Part{typ: PartType::Key,heights:[5,0,2,1,3]},
            Part{typ: PartType::Key,heights:[4,3,4,0,2]},
            Part{typ: PartType::Key,heights:[3,0,2,0,1]},
            ]
    )
)]
    fn parse_should(#[case] input: &str, #[case] expected: (Vec<Part>, Vec<Part>)) {
        assert_eq!(parse(input), expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 3)]
    fn aoc_2024_25_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_25_a(input), expected);
    }

    #[test]
    fn aoc_2024_25_a() {
        assert_eq!(super::aoc_2024_25_a(super::INPUT), 3301);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_25_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_25_b(input), expected);
    }

    #[test]
    fn aoc_2024_25_b() {
        assert_eq!(super::aoc_2024_25_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
