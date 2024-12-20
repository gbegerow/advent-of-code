// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/20
    Solution idea:

*/
use aoc_utils::grid::{Grid, EAST, NORTH, SOUTH, WEST};
use glam::IVec2;
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, Eq)]
struct PathTile {
    display: char,
    id: i32,
    visited: bool,
    /// cheats from this position, as position for first path tile after wall
    cheats: Vec<IVec2>,
}

impl PartialEq for PathTile {
    fn eq(&self, other: &Self) -> bool {
        self.display == other.display && self.id == other.id //&& self.cheats == other.cheats
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Wall,
    Path(PathTile),
}
impl Tile {
    fn visited(&self) -> bool {
        match self {
            // can't visit a wall (?)
            Tile::Wall => false,
            // but a path tile
            Tile::Path(path_tile) => path_tile.visited,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Wall => '#',
                // display number of cheats if any
                // Tile::Path(path_tile)
                //     if path_tile.display == '.'
                //         && path_tile.visited
                //         && !path_tile.cheats.is_empty() =>
                //     char::from_digit(path_tile.id as u32 % 10, 10).unwrap(),
                // char::from_digit((path_tile.cheats.len() % 10) as u32, 10).unwrap(),
                // otherwise '*' if it is a visited tile and not start or end
                Tile::Path(path_tile) if path_tile.display == '.' && path_tile.visited =>
                    char::from_digit(path_tile.id as u32 % 10, 10).unwrap(), //'*',

                // if not visited the tile display
                Tile::Path(path_tile) => path_tile.display,
            },
        )
    }
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            '.' | 'S' | 'E' => Ok(Tile::Path(PathTile {
                display: value,
                id: 0,
                visited: false,
                cheats: vec![],
            })),
            _ => Err(()),
        }
    }
}

impl From<Tile> for char {
    fn from(value: Tile) -> Self {
        match value {
            Tile::Wall => '#',
            Tile::Path(path_tile) => path_tile.display,
        }
    }
}



#[tracing::instrument]
pub fn find_cheats(input: &str, WALL_HACK_LEN: i32, threshold: i32) -> usize {
    let mut grid = input.parse::<Grid<Tile>>().expect("valid grid");
    let start = grid
        .find('S'.try_into().unwrap())
        .expect("no start (S) found");
    let end = grid
        .find('E'.try_into().unwrap())
        .expect("no end (E) found");

    // Follow the path with DFS and scan for possible cheats
    //we only have one possible next tile, so we could have gotten away with a single scalar for next...
    let mut frontier = VecDeque::from([start]);
    let mut at = 0;
    while let Some(cursor) = frontier.pop_back() {
        if grid[cursor].visited() {
            continue;
        }

        // scan for possible cheats before we take a mutable reference to grid
        let mut cheats = Vec::new();
        // part b is no longer a line but a path from pathtile a to pathtile b
        // paths starting and ending on same tiles are only counted once.
        // So candidates are all path tiles in WALL_HACK_LEN radius around cursor
        // distance is Manhattan distance
        // test if part a  stays same with possible 90° turn

        for dir in [NORTH, EAST, SOUTH, WEST] {
            let scan_line = (0..WALL_HACK_LEN)
                .flat_map(|i| grid.get(cursor + i * dir).map(|t| char::from(t.clone())))
                .collect::<String>();
            // a cheat must start at a wall
            if !scan_line.starts_with(".#") {
                continue;
            }

            // println!("From {} -> {}: {}", at + 1, dir, scan_line);
            // it is a valid cheat if there is a wall followed by an empty space or end
            // scanline = ".#.|.##.|.#E|.##E"
            if let Some(pos) = scan_line.find("#E") {
                // AoC uses ASCII so pos is a valid position
                // println!("Shortcut to end from {}", at + 1);
                // end tile after wall
                cheats.push(cursor + (pos as i32 + 1) * dir);
            } else if let Some(pos) = scan_line.find("#.") {
                // AoC uses ASCII so pos is a valid position
                // first path tile after wall
                cheats.push(cursor + (pos as i32 + 1) * dir);
            }
        }

        {
            // write back (short term mutable borrow)
            let Tile::Path(tile) = &mut grid[cursor] else {
                panic!("suffocating in wall.");
            };

            // bookkeepiing
            at += 1;
            assert_eq!(tile.id, 0, "tile visited multiple times");
            assert!(!tile.visited, "tile visited multiple times");

            tile.id = at;
            tile.visited = true;
            tile.cheats = cheats;
        }

        if cursor == end {
            break;
        }
        // find next waypoint
        for (next, tile) in grid.iter_axis_neighbours_with_positions(cursor) {
            if let Tile::Path(_) = tile {
                frontier.push_back(next);
            }
        }
    }

    println!("Last id: {at}");
    println!("{grid:#}");

    // Now we know all possible cheats, get the distances
    let cheat_candidates = grid
        .iter()
        .flat_map(|tile| match tile {
            Tile::Path(path_tile) if !path_tile.cheats.is_empty() => Some(path_tile.clone()),
            _ => None,
        })
        // .inspect(|tile| println!("{tile:?}"))
        // map cheats to distance
        .flat_map(|tile| {
            tile.cheats
                .iter()
                .flat_map(|c| match &grid[*c] {
                    Tile::Wall => panic!("cheat should end on path tile"),
                    // only cheats from early to late are shortcuts
                    Tile::Path(path_tile) if path_tile.id > tile.id => {
                        // println!(
                        //     "Cheat from {}'{}' -> {}'{}' ({})",
                        //     tile.id,
                        //     tile.display,
                        //     path_tile.id,
                        //     path_tile.display,
                        //     path_tile.id - tile.id
                        // );
                        // we visit path on both sides of the cheat
                        Some(path_tile.id - tile.id - 2)
                    }
                    // cheat ends on unvisited pathtile or leads back, useless
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // println!("{cheat_candidates:?}");

    cheat_candidates.iter().filter(|d| d >= &&threshold).count()
}

pub fn aoc_2024_20_a(input: &str, threshold: i32) -> usize {
find_cheats(input,2+2, threshold)

#[tracing::instrument]
pub fn aoc_2024_20_b(input: &str,threshold: i32) -> usize {
    find_cheats(input, 20+2, threshold)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 5)]
    fn aoc_2024_20_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_20_a(input, 15), exepected);
    }

    #[test]
    fn aoc_2024_20_a() {
        assert_eq!(super::aoc_2024_20_a(super::INPUT, 100), 1293);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_20_b_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_20_b(input, 50), exepected);
    }

    #[test]
    fn aoc_2024_20_b() {
        assert_eq!(super::aoc_2024_20_b(super::INPUT, 100), 0);
    }

    const TEST_INPUT: &str = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
