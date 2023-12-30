use std::collections::BTreeSet;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/11
    Solution idea:
    Sparse storage not grid. For every empty line during parsing increase y coordinate by 2.
    2nd pass for every column after empty column, increase x by spacing.
    Distance is just straight line manhattan distance (or ordered manhattan distance?). No need for any graph algo.
    itertools catesian product
*/
use itertools::Itertools;

pub fn calculate_expansion(input: &str, expansion_factor: usize) -> usize {
    let spectated_galaxy_map = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.trim()
                .chars()
                .enumerate()
                .flat_map(move |(col, c)| match c {
                    '#' => Some((row, col)),
                    _ => None,
                })
        })
        .collect::<Vec<_>>();

    // empty lines/columns = all.except(existing)
    let existing = spectated_galaxy_map.iter().fold(
        (BTreeSet::new(), BTreeSet::new(), 0, 0),
        |(mut row_set, mut col_set, max_row, max_col), coord| {
            row_set.insert(coord.0);
            col_set.insert(coord.1);
            (row_set, col_set, max_row.max(coord.0), max_col.max(coord.1))
        },
    );
    // coming from BTreeSet, empty_rows/cols are sorted
    let empty_rows = BTreeSet::from_iter(0..existing.2)
        .difference(&existing.0)
        .cloned()
        .collect::<Vec<_>>();
    let empty_cols = BTreeSet::from_iter(0..existing.3)
        .difference(&existing.1)
        .cloned()
        .collect::<Vec<_>>();
    // println!("empty rows: {:?} empty cols: {:?} galaxies: {}", empty_rows, empty_cols, spectated_galaxy_map.len());

    // expand universe: map every empty row/column to one spacing more than its predecessor.
    // From highest to lowest add spacing to galaxy if coord is higher than empty
    let factor = expansion_factor - 1; // -1 because we already have one spacing
    let expanded_galaxy_map = spectated_galaxy_map
        .iter()
        .map(|(row, col)| {
            let mut new_row = *row;
            let mut new_col = *col;
            // no need to reverse, we collect in a new vec
            // possible optimization: precalculate the offset for every empty row/col
            for empty_row in empty_rows.iter() {
                if *empty_row < *row {
                    new_row += factor;
                }
            }
            for empty_col in empty_cols.iter() {
                if *empty_col < *col {
                    new_col += factor;
                }
            }
            (new_row, new_col)
        })
        .collect::<Vec<_>>();

    // manhattan distance of all galaxies
    expanded_galaxy_map
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
                (a.0 as isize - b.0 as isize).abs() as usize
            +   (a.1 as isize - b.1 as isize).abs() as usize
        })
        .sum()
}

pub fn aoc_2023_11_a(input: &str) -> usize {
    calculate_expansion(input, 2)
}
pub fn aoc_2023_11_b(input: &str) -> usize {
    calculate_expansion(input, 1_000_000)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn aoc_2023_11_a_example() {
        assert_eq!(super::aoc_2023_11_a(TEST_INPUT), 374);
    }

    #[test]
    fn aoc_2023_11_a() {
        assert_eq!(super::aoc_2023_11_a(INPUT), 9799681);
    }

    #[rstest]
    #[case(2, 374)]
    #[case(10, 1030)]
    #[case(100, 8410)]
    fn aoc_2023_11_b_example(#[case] expansion_factor: usize, #[case] expected: usize) {
        assert_eq!(
            super::calculate_expansion(TEST_INPUT, expansion_factor),
            expected
        );
    }

    #[test]
    fn aoc_2023_11_b() {
        assert_eq!(super::aoc_2023_11_b(INPUT), 513171773355);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";
}
