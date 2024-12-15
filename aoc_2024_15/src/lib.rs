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
            match *m {
                NORTH => '^',
                EAST => '<',
                SOUTH => 'v',
                WEST => '<',
                _ => '?',
            }
        )
    }
    println!();
}

fn parse_wide_grid(s: &str) -> Result<Grid<char>, ParseError> {
    let s = s.trim();
    let width = 2 * s.lines().next().unwrap().trim().chars().count();
    let height = s.lines().count();

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

        // handle wide crate
        '[' | ']' => {
            // there might be a huge buildup which all must move
            // bfs for the whole blob

            let mut frontier = vec![next];
            // move all or nothing Last In First out, soHashSet is not good choice here
            let mut move_it = Vec::new();
            let mut can_move = true;

            while let Some(pos) = frontier.pop() {
                move_it.push(pos);
                // we have explicit neighbours
                // are we pushing agaist one side of a crate? Add the other to the pile
                match grid[pos] {
                    '[' if !frontier.contains(&(pos + EAST))
                        && !move_it.contains(&(pos + EAST)) =>
                    {
                        frontier.push(pos + EAST);
                    }
                    ']' if !frontier.contains(&(pos + WEST))
                        && !move_it.contains(&(pos + WEST)) =>
                    {
                        frontier.push(pos + WEST);
                    }
                    _ => (),
                }

                let moves_to = pos + direction;
                match grid[moves_to] {
                    // found a new box, move it along
                    '[' | ']' if !frontier.contains(&moves_to) && !move_it.contains(&moves_to) => {
                        frontier.push(moves_to);
                    }
                    // hit a rock (literaly)
                    '#' => {
                        can_move = false;
                        break;
                    }
                    _ => (),
                }
            }

            if can_move {
                // move the whole pile in LIFO order
                while let Some(m) = move_it.pop() {
                    // there should only be '.' or already overwritten stuff there as we work in LIFO order
                    grid[m + direction] = grid[m];
                    grid[m] = '.'; // cleanup behind
                }

                // and move the cursor
                grid.cursor += direction;
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

fn calculate_score(grid: &Grid<char>) -> i32 {
    grid.iter_with_positions()
        .filter(|(_, c)| **c == 'O' || **c == '[')
        .fold(0, |accu, (p, _)| accu + p.x + p.y * 100)
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

    calculate_score(&grid)
}

#[tracing::instrument]
pub fn aoc_2024_15_b(input: &str) -> i32 {
    // setup
    let (gd, mv) = input.split_once("\n\n").expect("valid grid");

    let mut grid = parse_wide_grid(gd).expect("valid grid");
    grid.find_cursor('@', '.');
    // println!("{grid:?}\n{grid}");

    let moves = parse_moves(mv);

    // update
    for direction in moves {
        move_to(&mut grid, direction);
        // println!("{grid}");
    }
    println!("{grid}");

    // result
    calculate_score(&grid)
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
    #[case(TEST_INPUT_2, 1751)]
    #[case(TEST_INPUT_3, 618)]
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

    #[allow(dead_code)]
    const TEST_INPUT_3: &str = "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
}
