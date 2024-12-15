use std::string::ParseError;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/15
    Solution idea:

*/
use aoc_utils::grid::{Grid, EAST, NORTH, SOUTH, WEST};
use glam::IVec2;

fn parse_moves(mv: &str) -> Vec<IVec2> {
    let moves: Vec<_> = mv
        .chars()
        .flat_map(|c| match c {
            '^' => Some(NORTH),
            '>' => Some(EAST),
            'v' => Some(SOUTH),
            '<' => Some(WEST),
            _ => None,
        })
        .collect();

    moves
}

#[allow(dead_code)]
fn print_moves(moves: &Vec<IVec2>) {
    for m in moves {
        print!(
            "{}",
            match m {
                &NORTH => '^',
                &EAST => '<',
                &SOUTH => 'v',
                &WEST => '<',
                _ => '?',
            }
        )
    }
    println!("");
}

fn parse_wide_grid(s: &str) -> Result<Grid<char>, ParseError> {
    let s = s.trim();
    let width = 2 * s.lines().next().unwrap().trim().chars().count() as usize;
    let height = s.lines().count() as usize;

    // use lines, we want to trim any line individually
    let values: Vec<char> = s
        .lines()
        .flat_map(|x| {
            x.trim().chars().flat_map(|c| match c {
                '@' => ['@', '.'],
                'O' => ['[', ']'],
                _ => [c, c],
            })
        })
        .collect();

    Ok(Grid::new(values, width, height))
}

fn move_to(grid: &mut Grid<char>, direction: IVec2) {
    // look for '#' in move direction
    let mut next = grid.cursor + direction;
    match grid[next] {
        // if there is a run of crates 'O' and then a '.',
        //  swap first 'O' with '.' (crates are interchangable)
        'O' => {
            while grid[next] != '.' && grid[next] == 'O' {
                next += direction;
            }
            // is there empty space to move to?
            if grid[next] == '.' {
                grid.cursor += direction;
                let cursor = grid.cursor;
                grid[cursor] = '.';
                grid[next] = 'O';
            }
        }
        // empty space, just move cursor
        '.' => {
            grid.cursor += direction;
        }
        // do nothing on walls
        _ => {}
    }
}

#[tracing::instrument]
pub fn aoc_2024_15_a(input: &str) -> i32 {
    let (gd, mv) = input.split_once("\n\n").expect("valid grid");

    let mut grid = gd.parse::<Grid<char>>().expect("valid grid");
    grid.find_cursor('@', '.');

    let moves = parse_moves(mv);

    for direction in moves {
        move_to(&mut grid, direction);
    }

    println!("{grid}");

    grid.iter_with_positions()
        .filter(|(_, c)| **c == 'O')
        .fold(0, |accu, (p, _)| accu + p.x + p.y * 100)
}

#[tracing::instrument]
pub fn aoc_2024_15_b(input: &str) -> i32 {
    let (gd, mv) = input.split_once("\n\n").expect("valid grid");

    let mut grid = parse_wide_grid(gd).expect("valid grid");
    grid.find_cursor('@', '.');

    let moves = parse_moves(mv);

    println!("{grid:#}");
    print_moves(&moves);

    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 10092)]
    fn aoc_2024_15_a_example(#[case] input: &str, #[case] exepected: i32) {
        assert_eq!(super::aoc_2024_15_a(input), exepected);
    }

    #[test]
    fn aoc_2024_15_a() {
        assert_eq!(super::aoc_2024_15_a(super::INPUT), 1514353);
    }

    #[rstest]
    #[case(TEST_INPUT, 9021)]
    fn aoc_2024_15_b_example(#[case] input: &str, #[case] exepected: i32) {
        assert_eq!(super::aoc_2024_15_b(input), exepected);
    }

    #[test]
    fn aoc_2024_15_b() {
        assert_eq!(super::aoc_2024_15_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "
    ##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
}
