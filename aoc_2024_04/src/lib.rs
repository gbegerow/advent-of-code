// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/04
    Solution idea:

*/
use glam::IVec2;

#[allow(dead_code)]
fn get_adjacent_positions() -> Vec<IVec2> {
    vec![
        IVec2::new(1, 1),
        IVec2::new(0, 1),
        IVec2::new(1, -1),
        IVec2::new(0, -1),
        IVec2::new(-1, 1),
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
        IVec2::new(-1, -1),
    ]
}

#[allow(dead_code)]
fn get_diagonal_positions() -> Vec<IVec2> {
    vec![
        IVec2::new(-1, -1), // NW
        IVec2::new(1, 1),   // SE
        IVec2::new(-1, 1),  // NE
        IVec2::new(1, -1),  // SW
    ]
}

fn scan_for_startpoints(grid: &[Vec<char>], start: char) -> Vec<IVec2> {
    let w = grid[0].len();
    let h = grid.len();
    let mut startpoints = Vec::with_capacity(w * h);

    // scan for startpoints
    #[allow(clippy::needless_range_loop)]
    for row in 0..h {
        for col in 0..w {
            if grid[row][col] == start {
                startpoints.push(IVec2::new(row as i32, col as i32));
            }
        }
    }
    startpoints
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let grid = input
        .trim()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    grid
}

#[tracing::instrument]
pub fn aoc_2024_04_a(input: &str) -> usize {
    const WORD: &str = "XMAS";
    let word: Vec<char> = WORD.chars().collect();
    let word_len = word.len();

    let grid = parse(input);

    let w = grid[0].len();
    let h = grid.len();
    let bounds = IVec2::new(w as i32 - 1, h as i32 - 1);

    // looking for x
    let startpoints = scan_for_startpoints(&grid, word[0]);
    // scan all startpoints
    let neighbours = get_adjacent_positions();
    let mut count = 0;

    for start in startpoints {
        for delta in &neighbours {
            let mut i = 1; // no need to test startpoint again
            let mut is_match = true;
            while i < word_len && is_match {
                let p = start + delta * i as i32;
                let out_of_bounds = p.min(IVec2::ZERO) != IVec2::ZERO || p.max(bounds) != bounds;
                if out_of_bounds {
                    is_match = false;
                    break;
                }

                let (r, c) = (p.x as usize, p.y as usize);
                is_match &= grid[r][c] == word[i];

                i += 1;
            }

            if is_match {
                count += 1;
            }
        }
    }

    count
}

#[tracing::instrument]
pub fn aoc_2024_04_b(input: &str) -> usize {
    // todo: rewrite to utils::grid
    let grid = parse(input);

    let w = grid[0].len();
    let h = grid.len();
    let bounds = IVec2::new(w as i32 - 1, h as i32 - 1);

    // looking for A in center
    let startpoints = scan_for_startpoints(&grid, 'A');

    let neighbours = get_diagonal_positions();
    let mut count = 0;

    // scan all startpoints
    for start in startpoints {
        let x_points: Vec<char> = neighbours
            .iter()
            .map(|d| start + d)
            .filter(|p| p.min(IVec2::ZERO) == IVec2::ZERO && p.max(bounds) == bounds)
            .map(|p| grid[p.x as usize][p.y as usize])
            .filter(|c| *c == 'M' || *c == 'S')
            .collect();

        // out of bounds or different letters on diagonal
        if x_points.len() != 4 {
            continue;
        }

        // don't care about order on diagonal but both endpoints must be different M A S oder S A M
        let is_match = x_points[0] != x_points[1] && x_points[2] != x_points[3];
        if is_match {
            count += 1;
        }
    }

    count
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 18)]
    fn aoc_2024_04_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_04_a(input), expected);
    }

    #[test]
    fn aoc_2024_04_a() {
        assert_eq!(super::aoc_2024_04_a(super::INPUT), 2358);
    }

    #[rstest]
    #[case(TEST_INPUT, 9)]
    fn aoc_2024_04_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_04_b(input), expected);
    }

    #[test]
    fn aoc_2024_04_b() {
        assert_eq!(super::aoc_2024_04_b(super::INPUT), 1737);
    }

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
}
