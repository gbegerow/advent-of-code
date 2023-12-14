// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/10
    Solution idea:

*/

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn next_at(pipe: &char, last_direction: &Direction) -> Option<Direction> {
    match (pipe, last_direction) {
        ('-', Direction::West) => Some(Direction::West),
        ('-', Direction::East) => Some(Direction::East),
        ('|', Direction::North) => Some(Direction::North),
        ('|', Direction::South) => Some(Direction::South),
        ('L', Direction::South) => Some(Direction::East),
        ('L', Direction::West) => Some(Direction::North),
        ('F', Direction::South) => Some(Direction::East),
        ('F', Direction::West) => Some(Direction::South),
        ('J', Direction::South) => Some(Direction::West),
        ('J', Direction::East) => Some(Direction::North),
        ('7', Direction::East) => Some(Direction::South),
        ('7', Direction::North) => Some(Direction::West),

        _ => None,
    }
}

fn walk_from(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    dir: Direction,
) -> (usize, usize, Direction) {
    grid.get(row)
        .and_then(|rw| rw.get(col))
        .and_then(|field| next_at(field, &dir))
        .and_then(|dir| {
            Some(match dir {
                Direction::North => (row - 1, col, dir),
                Direction::East => (row, col + 1, dir),
                Direction::South => (row + 1, col, dir),
                Direction::West => (row, col - 1, dir),
            })
        })
        .expect("broken pipe")
}

fn scan(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<Direction> {
    // get will take care of overflow
    let mut neighbours = vec![
        (row, col + 1, Direction::East),
        (row + 1, col, Direction::South),
    ];
    if row > 0 {
        neighbours.push((row - 1, col, Direction::North));
    };
    if col > 0 {
        neighbours.push((row, col - 1, Direction::West));
    };

    neighbours
        .iter()
        .flat_map(|(r, c, dir)| {
            grid.get(*r)
                .and_then(|rw| rw.get(*c))
                .and_then(|field| next_at(field, dir))
                .and_then(|_| Some(dir.clone())) // we are not interessted where to go after scan, only which directions are valid
        })
        .collect()
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize){
    for (row_num, row) in grid.iter().enumerate()  {
        if let Some(col_num) = row.iter().position(|&c| c == 'S') {
            return (row_num, col_num);
        }
    }
    unreachable!();
}

fn parse(input: &str) -> (Vec<Vec<char>>, usize, usize) {
    assert!(input.trim().len() > 0);
    let grid = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (start_row, start_col) = find_start(&grid);

    (grid, start_row, start_col)
}

#[allow(unused_variables)]
pub fn aoc_2023_10_a(input: &str) -> usize {
    let (grid, start_row, start_col) = parse(input);
    let mut pipes = scan(&grid, start_row, start_col)
        .iter()
        .map(|d| (start_row, start_col, *d))
        .collect::<Vec<_>>();
    let mut counter = 0;
    // longest path is found if both ways reach the same tile
    while pipes[0] != pipes[1] && counter < grid.len() * grid[0].len() {
        pipes = dbg!(pipes
            .iter()
            .map(|&(row, col, dir)| walk_from(&grid, row, col, dir))
            .collect());
        counter += 1;
    }
    counter
}

#[allow(unused_variables)]
pub fn aoc_2023_10_b(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use crate::Direction;
    use rstest::rstest;

    #[test]
    fn aoc_2023_10_a_example() {
        assert_eq!(super::aoc_2023_10_a(TEST_INPUT), 8);
    }

    #[test]
    fn aoc_2023_10_a() {
        assert_eq!(super::aoc_2023_10_a(INPUT), 0);
    }

    #[test]
    fn aoc_2023_10_b_example() {
        assert_eq!(super::aoc_2023_10_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_10_b() {
        assert_eq!(super::aoc_2023_10_b(INPUT), 0);
    }

    #[rstest]
    #[case("...\n.S.\n...", vec![])]
    #[case(".|.\n.S.\n.J.", vec![Direction::North, Direction::South])]
    #[case("...\nLS-\n.-.", vec![Direction::East, Direction::West])]
    fn scan_should_give_direction(#[case] input: &str, #[case] expected: Vec<Direction>) {
        let (grid, start_row, start_col) = super::parse(input);

        assert_eq!(start_row, 1);
        assert_eq!(start_col, 1);
        assert_eq!(super::scan(&grid, start_row, start_col), expected);
    }

    #[rstest]
    #[case(Direction::West, 2, 2, Direction::North)]
    #[case(Direction::South, 3, 0, Direction::South)]
    fn walk_should_give_direction(
        #[case] go: Direction,
        #[case] row: usize,
        #[case] col: usize,
        #[case] dir: Direction,
    ) {
        let (grid, start_row, start_col) = super::parse(TEST_INPUT);

        assert_eq!(
            super::walk_from(&grid, start_row, start_col, go),
            (row, col, dir)
        );
    }
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    7-F7-
    .FJ|7
    SJLL7
    |F--J
    LJ.LJ";
}
