// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/04
    Solution idea:

*/
use aoc_utils::grid::Grid;
use glam::IVec2;

fn accessible_rolls(grid: &Grid<char>) -> Vec<IVec2> {
   grid.iter_with_positions()
     .filter(|(pos, value)| 
            **value == '@' &&
            grid.iter_adajacent_neighbours(*pos)
            .filter(|n| **n == '@')
            .count() < 4
    ).map(|(pos, _)| pos)
    .collect()
}

#[tracing::instrument]
pub fn aoc_2025_04_a(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().expect("valid grid");
 
    accessible_rolls(&grid).len()
}

#[tracing::instrument]
pub fn aoc_2025_04_b(input: &str) -> usize {
    let mut grid = input.parse::<Grid<char>>().expect("valid grid");

    let mut removables = accessible_rolls(&grid);
    let mut removed = 0;
    while !removables.is_empty() {
        for pos in &removables {
            // remove roll
            if let Some(cell) = grid.get_mut(*pos) {
                *cell = '.';
                removed += 1;
            }
        }

        // invariant: number of accessible rolls decreases each iteration
        removables = accessible_rolls(&grid);        
    }

    removed
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 13)]
    fn aoc_2025_04_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_04_a(input), expected);
    }

    #[test]
    fn aoc_2025_04_a() {
        assert_eq!(super::aoc_2025_04_a(super::INPUT), 1537);
    }

    #[rstest]
    #[case(TEST_INPUT, 43)]
    fn aoc_2025_04_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_04_b(input), expected);
    }

    #[test]
    fn aoc_2025_04_b() {
        assert_eq!(super::aoc_2025_04_b(super::INPUT), 8707);
    }

    const TEST_INPUT: &str = "
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
