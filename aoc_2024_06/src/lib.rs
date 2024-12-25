// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/06
Solution idea:
    Lines in 2D, either sparsemap aka HashSet or K-D-Tree (2D-Tree, alternating x,y partitioning)
        or bitmap
    main op: find all cells between current pos and next obstruction.
    count every visited cell exactly once
    end if cursor leaves bounds
    Similar to 2023 16

b How to construct a loop?
candidate must be in visited
canditate must be "before" an obstacle
    and cause a right turn to another obstacle

record for every visited pos and dir
record every turn incoming_direction
loop_dir = incoming_direction turn counter clockwise
scan every turn backwards for visited point in loopdir, add to candidates

run patrol for every candidate, if a turn is encounted twice in same direction we found loop
*/
use aoc_utils::grid::{Grid, EAST, NORTH, SOUTH, WEST};
use glam::IVec2;
use std::collections::HashSet;

fn get_visited(mut grid: Grid<char>) -> (HashSet<IVec2>, bool) {
    let mut direction = NORTH;

    // record turns as nodes on the graph
    let mut nodes = vec![(grid.cursor, direction)];
    let mut visited = HashSet::with_capacity(grid.width * grid.height);
    visited.insert(grid.cursor);

    // while inside
    while let Some(c) = grid.move_cursor(direction) {
        match c {
            // obstruction
            '#' => {
                // reached an obstruction, turn right / clockwise
                grid.cursor -= direction;
                direction = IVec2::new(-direction.y, direction.x);

                let node = (grid.cursor, direction);
                if nodes.contains(&node) {
                    // found a loop
                    return (visited, true);
                } else {
                    nodes.push(node);
                }
            }
            // moved
            _ => {
                let cursor = grid.cursor;
                visited.insert(cursor);
                grid[cursor] = match direction {
                    NORTH => '^',
                    EAST => '>',
                    SOUTH => 'v',
                    WEST => '<',
                    _ => '*',
                };
            }
        }
    }
    // println!("{grid:#}");
    (visited, false)
}

#[tracing::instrument]
pub fn aoc_2024_06_a(input: &str) -> usize {
    let mut grid: Grid<char> = input.parse().expect("valid grid");
    // guard is always looking north (in sample and my input)
    grid.find_cursor('^', '.');

    let (visited, _) = get_visited(grid);
    visited.len()
}

#[tracing::instrument]
pub fn aoc_2024_06_b(input: &str) -> usize {
    let mut grid: Grid<char> = input.parse().expect("valid grid");
    // guard is always looking north (in sample and my input)
    grid.find_cursor('^', '.');

    let g2 = grid.clone();
    let (visited, _) = get_visited(g2);

    // we could have constraiint further, but it is fast enough to just test every visited point
    let mut loops = 0;
    for v in visited {
        let mut g2 = grid.clone();
        // set a new obstacle
        g2[v] = '#';

        let (_, in_loop) = get_visited(g2);
        if in_loop {
            loops += 1;
        }
    }

    loops
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {

    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 41)]
    fn aoc_2024_06_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_06_a(input), expected);
    }

    #[test]
    fn aoc_2024_06_a() {
        assert_eq!(super::aoc_2024_06_a(super::INPUT), 5101);
    }

    #[rstest]
    #[case(TEST_INPUT, 6)]
    fn aoc_2024_06_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_06_b(input), expected);
    }

    #[test]
    fn aoc_2024_06_b() {
        assert_eq!(super::aoc_2024_06_b(super::INPUT), 1951);
    }

    // #[test]
    // fn visited_should_be_same_as_old() {
    //     use std::collections::HashSet;

    // use glam::IVec2;
    // // aoc grid yields 1 less for full input, bc start was not in visited

    //     let new = super::get_visited(super::INPUT);
    //     let old = super::get_visited_old(super::INPUT)
    //         .iter()
    //         // old uses row, col instead of x,y
    //         .map(|v| IVec2::new(v.y, v.x))
    //         .collect::<HashSet<_>>();

    //     let diff = new.symmetric_difference(&old).collect::<HashSet<_>>();

    //     assert_eq!(diff, HashSet::new());
    //     // assert_eq!(diff.len(), 0);
    //     // new.iter()
    //     //     .zip(old.iter())
    //     //     .enumerate()
    //     //     .for_each(|(i, (n, o))| assert_eq!(n, o, "{}: {}  vs  {}", i, n, o));
    // }

    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
}
