// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/07
    Solution idea:

*/
use aoc_utils::grid::Grid;
use glam::IVec2;
use std::collections::BTreeSet;

#[tracing::instrument]
pub fn aoc_2025_07_a(input: &str) -> Result<usize, String> {
    let mut grid = input.parse::<Grid<char>>().expect("valid grid");
    let width = grid.width as i32;
    let start = grid.find_cursor('S', 'S');

    // list all active x coordinates of beams in a sorted set
    let mut beams = BTreeSet::new();
    beams.insert(start.x);

    let mut split_count = 0;
    for y in start.y + 1..grid.height as i32 {
        // do not mutate the set while iterating over it
        let mut splits = Vec::with_capacity(grid.width);
        for x in beams.iter() {
            if let Some(cell) = grid.get_mut(IVec2::new(*x, y)) {
                if *cell == '^' {
                    // beam split, remove original, add both sides
                    splits.push(*x);
                    split_count += 1;
                    continue;
                } else if *cell == '.' {
                    // beam continues
                    *cell = '|';
                    continue;
                }
            }
        }
        for x in splits.iter() {
            beams.remove(x);
            if x > &0 {
                beams.insert(x - 1);
            }
            if x + 1 < width {
                beams.insert(x + 1);
            }
        }
        println!("{}", grid);
    }

    Ok(split_count)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Beam {
    x: i32,
    count: usize,
}

impl Beam {
    fn new(x: i32, count: usize) -> Self {
        Self { x, count }
    }
}

#[tracing::instrument]
pub fn aoc_2025_07_b(input: &str) -> Result<usize, String> {
    let mut grid = input.parse::<Grid<char>>().expect("valid grid");
    let width = grid.width as i32;
    let start = grid.find_cursor('S', 'S');

    // list all active x coordinates of beams in a sorted set
    let mut beams = BTreeSet::new();
    beams.insert(Beam::new(start.x, 1));

    let mut split_count = 0;

    for y in start.y + 1..grid.height as i32 {
        // do not mutate the set while iterating over it
        let mut splits = Vec::with_capacity(grid.width);
        for beam in beams.iter() {
            let pos = IVec2::new(beam.x, y);
            if let Some(cell) = grid.get_mut(pos) {
                if *cell == '^' {
                    // beam split, remove original, add both sides
                    splits.push(*beam);
                    // println!("split at {} => {:?}", pos, splits);
                    continue;
                } else if *cell == '.' {
                    // beam continues
                    *cell = '|';
                    continue;
                }
            }
        }

        // an additional timeline is created if two splitter splits to the same position
        // we must keep track of how many possibilities are for each beam position

        // only splitter with gap of 1 can create duplicates, so duplicates must be adjacent in splits
        // there are no directly adjacent splitters, so no more than two can overlap at once
        // dupes are not enough as the multiplicity propagates, so we need to count per beam
        // let dupes = splits
        //     .windows(2)
        //     .filter(|w| w[0].x + 2 == w[1].x)
        //     .cloned()
        //     .collect::<Vec<_>>();
        // println!("y={} splits={:?} dupes={}", y, splits, dupes);

        for beam in splits.iter() {
            let incoming_possibilities = beam.count;
            let x = beam.x;

            // remove x-1, x, x+1 range and re-insert updated beams
            let chunk = beams
                .extract_if(
                    Beam::new(x - 1, usize::MIN)..Beam::new(x + 1, usize::MAX),
                    |_| true,
                )
                .collect::<Vec<_>>();

            if x > 0 {
                let existing = if chunk[0].x == x - 1 {
                    chunk[0].count
                } else {
                    0
                };
                beams.insert(Beam::new(x - 1, existing + incoming_possibilities));
            }

            let last = chunk.len()-1;
            if x + 1 < width {
                let existing = if chunk[last].x == x + 1 {
                    chunk[last].count
                } else {
                    0
                };
                beams.insert(Beam::new(x + 1, existing + incoming_possibilities));
            }
        }
        split_count = beams.iter().map(|b| b.count).sum::<usize>();
        // println!(
        //     "row: {} split_count: {} #beams {} beams: [{:?}]",
        //     y,
        //     split_count,
        //     beams.len(),
        //     beams,
        // );
    }

    Ok(split_count)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 21)]
    fn aoc_2025_07_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_07_a(input), Ok(expected));
    }

    #[test]
    fn aoc_2025_07_a() {
        assert_eq!(super::aoc_2025_07_a(super::INPUT), Ok(1628));
    }

    #[rstest]
    #[case(TEST_INPUT, 40)]
    fn aoc_2025_07_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_07_b(input), Ok(expected));
    }

    #[test]
    fn aoc_2025_07_b() {
        assert_eq!(super::aoc_2025_07_b(super::INPUT), Ok(0));
    }

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[allow(dead_code)]
    const TEST_INPUT_DIAGRAM: &str = "
.......S....... 01:
.......|....... 02:
......|^|...... 03: 1 split
......|.|...... 04:
.....|^|^|..... 05: 2 splits, 1+2+1=4 possibilities, 3 beams
.....|.2.|..... 06:
....|^|^|^|.... 07: 3 splits, 1+3+3+1=8 possibilities, 4 beams
....|.3.3.|.... 08:
...|^|^|||^|... 09: 3 splits, 1+4+3+1+1=10 possibilities, 5 beams
...|.4.33|.|... 10:
..|^|^|||^|^|.. 11: 4 splits, 1+5+4+3+4+1=18 possibilities, 6 beams
..|.5.434.5.|.. 12:
.|^|||^||.||^|. 13: 3 splits, 1+1+5+4+7+4+5+1+1=29 possibilities, 8 beams
.|.|54.74.5|.|. 14: 
|^|^|^|^|^|||^| 15: 5 splits, 1+2+8+11+4+5+1+1+1=40 possibilities, 9 beams (split 
|.2.9.B.4.5||.| 16: ";
}
