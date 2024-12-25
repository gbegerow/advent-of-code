// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/01
    Solution idea:

*/

use std::collections::HashSet;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Dir {
    #[default]
    North,
    East,
    South,
    West,
}

/// start is (0,0) looking north
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Pos {
    dir: Dir,
    row: i32,
    col: i32,
}

impl Pos {
    fn new(dir: Dir, row: i32, col: i32) -> Self {
        Self { dir, row, col }
    }
}

fn move_on_grid(a: Pos, m: &str) -> Pos {
    let m = m.trim_ascii();
    println!("{a:?} -> {m}");

    let b = m.as_bytes();
    // aoc input is always ASCII
    let turn = b[0];
    // ammount might be more than a single digit!
    let ammount: i32 = m[1..].parse().expect("invalid number");
    // println!("{a:?} -> {m} {ammount}");

    let pos = match (turn, a.dir) {
        (b'R', Dir::North) => Pos::new(Dir::East, a.row, a.col + ammount),
        (b'R', Dir::East) => Pos::new(Dir::South, a.row - ammount, a.col),
        (b'R', Dir::South) => Pos::new(Dir::West, a.row, a.col - ammount),
        (b'R', Dir::West) => Pos::new(Dir::North, a.row + ammount, a.col),

        (b'L', Dir::North) => Pos::new(Dir::West, a.row, a.col - ammount),
        (b'L', Dir::West) => Pos::new(Dir::South, a.row - ammount, a.col),
        (b'L', Dir::South) => Pos::new(Dir::East, a.row, a.col + ammount),
        (b'L', Dir::East) => Pos::new(Dir::North, a.row + ammount, a.col),

        _ => unreachable!("Got {turn} {ammount}"),
    };
    pos
}

pub fn aoc_2016_01_a(input: &str) -> usize {
    let target = input.split(',').fold(Pos::default(), move_on_grid);

    println!("Reached {:?}", target);
    // Manhattan distance from (0,0)
    (target.row.abs() + target.col.abs()) as usize
}

pub fn aoc_2016_01_b(input: &str) -> usize {
    let mut current_pos = Pos::default();
    let mut visited = HashSet::new();

    // find the first position (not endposition) visited twice
    for m in input.split(',') {
        let next_pos = move_on_grid(current_pos, m);

        // "draw" a line from current to next
        let inc = match next_pos.dir {
            Dir::North => (1, 0),
            Dir::East => (0, 1),
            Dir::South => (-1, 0),
            Dir::West => (0, -1),
        };

        let mut current = (current_pos.row, current_pos.col);
        while current != (next_pos.row, next_pos.col) {
            if visited.insert(current) {
                current.0 += inc.0;
                current.1 += inc.1;
                current_pos = next_pos;
            } else {
                println!("Reached {:?}", current);
                // Manhattan distance from (0,0)
                return (current.0.abs() + current.1.abs()) as usize;
            }
        }
    }

    unreachable!("There is no easter bunny");
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("R2, L3", 5)]
    #[case("R2, R2, R2", 2)]
    #[case("R5, L5, R5, R3", 12)]
    #[case("R5, R5, R5, R5", 0)]
    #[case("L5, L5, L5, L5", 0)]
    #[case("L500", 500)]
    fn aoc_2016_01_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2016_01_a(input), expected);
    }

    #[test]
    fn aoc_2016_01_a() {
        assert_eq!(super::aoc_2016_01_a(INPUT), 234);
    }

    #[rstest]
    //     ^
    //     ^
    //     ^
    //   >>x>>>>
    //     ^   v
    //     ^   v
    //     ^   v
    //     <<<<v
    #[case("R8, R4, R4, R8", 4)]
    fn aoc_2016_01_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2016_01_b(input), expected);
    }

    #[test]
    fn aoc_2016_01_b() {
        assert_eq!(super::aoc_2016_01_b(INPUT), 0);
    }

    const INPUT: &str = include_str!("input.txt");
}
