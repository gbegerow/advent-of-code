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

#[tracing::instrument]
pub fn aoc_2024_06_a(input: &str) -> usize {
    let mut grid: Grid<char> = input.parse().expect("valid grid");
    // guard is always looking north (in sample and my input)
    grid.find_cursor('^', '.');
    let mut direction = NORTH;

    // record turns as nodes on the graph
    let mut nodes = vec![(grid.cursor, direction)];
    let mut visited = HashSet::with_capacity(grid.width * grid.height);
    // while inside
    while let Some(c) = grid.move_cursor(direction) {
        match c {
            // obstruction
            '#' => {
                // reached an obstruction, turn right / clockwise
                grid.cursor -= direction;
                direction = IVec2::new(-direction.y, direction.x);
                nodes.push((grid.cursor, direction));
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
                }
            }
        }
    }
    println!("{grid:#}");
    // 1 less for full input
    // TODO: calc visited with both methods and look for difference

    // let grid_points = input
    //     .trim()
    //     .lines()
    //     .enumerate()
    //     .flat_map(|(row, l)| {
    //         l.chars().enumerate().flat_map(move |(col, c)| match c {
    //             '#' => Some((IVec2::new(row as i32, col as i32), b'#')),
    //             '^' => Some((IVec2::new(row as i32, col as i32), b'^')),
    //             '>' => Some((IVec2::new(row as i32, col as i32), b'>')),
    //             'v' => Some((IVec2::new(row as i32, col as i32), b'v')),
    //             '<' => Some((IVec2::new(row as i32, col as i32), b'<')),
    //             _ => None,
    //         })
    //     })
    //     .collect::<Vec<_>>();

    // // extract guards position from grid and remove it.
    // let (start, direction) = match grid_points.iter().find(|&(_p, t)| *t != b'#') {
    //     Some((p, b'^')) => (p, N),
    //     Some((p, b'>')) => (p, E),
    //     Some((p, b'v')) => (p, S),
    //     Some((p, b'<')) => (p, W),
    //     _ => panic!("No guard position found"),
    // };

    // let obstructions = grid_points
    //     .iter()
    //     .filter(|&(_p, t)| *t == b'#')
    //     .map(|&(p, _t)| p)
    //     .collect::<HashSet<_>>();
    // // is there an easy way to not iterate multiple times over input
    // let width = input
    //     .lines()
    //     .map(|l| l.len() as i32)
    //     .next()
    //     .expect("At least one line");
    // let height = input.lines().count() as i32;

    // let mut cursor = start.clone();
    // let mut direction = direction;

    // let mut visited = HashSet::with_capacity((width * height) as usize);
    // let outer_bounds = IVec2::new(width - 1, height - 1);
    // println!("bounds {} - {}", IVec2::ZERO, outer_bounds);
    // print_grid(cursor, direction, outer_bounds, &obstructions, &visited);

    // while cursor.min(IVec2::ZERO) == IVec2::ZERO && cursor.max(outer_bounds) == outer_bounds {
    //     // just one step at a time. if it was cheap to get the next obstacle in direction, this could be much more efficien as distance
    //     if obstructions.contains(&cursor) {
    //         // stop before obstacle, not in it
    //         cursor -= direction;
    //         // reached an obstruction, turn right / clockwise
    //         direction = IVec2::new(direction.y, -direction.x);

    //         print_grid(cursor, direction, outer_bounds, &obstructions, &visited);
    //     } else {
    //         visited.insert(cursor);
    //     }
    //     cursor += direction;
    // }
    // print_grid(cursor, direction, outer_bounds, &obstructions, &visited);

    // assert!(
    //     obstructions.is_disjoint(&visited),
    //     "Obstacles should never be visited"
    // );

    visited.len()
}

// fn print_grid(
//     cursor: IVec2,
//     direction: IVec2,
//     outer_bounds: IVec2,
//     obstructions: &HashSet<IVec2>,
//     visited: &HashSet<IVec2>,
// ) {
//     println!(
//         "cursor: {} direction: {} visited: {}",
//         cursor,
//         direction,
//         visited.len()
//     );
//     if visited.len() < 50 {
//         println!("{:?}", visited);
//     }

//     for row in 0..=outer_bounds.y {
//         for col in 0..=outer_bounds.x {
//             let cur = IVec2::new(row, col);
//             let c = if cur == cursor {
//                 match direction {
//                     N => '^',
//                     E => '>',
//                     S => 'v',
//                     W => '<',
//                     _ => '?',
//                 }
//             // } else if cur == IVec2::ZERO {
//             //     'o'
//             } else if obstructions.contains(&cur) {
//                 '#'
//             } else if visited.contains(&cur) {
//                 '*'
//             } else {
//                 '.'
//             };
//             print!("{c}");
//         }
//         println!("");
//     }
//     println!("");
// }

#[tracing::instrument]
pub fn aoc_2024_06_b(input: &str) -> usize {
    0
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
        assert_eq!(super::aoc_2024_06_b(super::INPUT), 0);
    }

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
