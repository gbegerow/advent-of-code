// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/08
Solution idea:

Postmortem: not a good choice of datastructure, lot of linear search

Better would be a HashMap<IVec2, Vec<Cell>> with enum Cell {Antenna(char), Antinode}
Use a temporary HashMap<char, Vec<IVec2>> for construction of the Antinodes
*/

use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

use glam::IVec2;

#[derive(Debug, Clone)]
struct Group {
    id: char,
    antennas: Vec<IVec2>,
    antinodes: Vec<IVec2>,
}

impl Group {
    fn find_antinodes(&mut self, bounds: IVec2) {
        if !self.antinodes.is_empty() {
            return;
        }

        println!("Group {}:{:?}", self.id, self.antennas);
        // general form of cross product:  let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
        let antinodes = self
            .antennas
            .iter()
            .flat_map(|a| {
                self.antennas
                    .iter()
                    // no antinodes by antenna itself
                    .flat_map(move |b| (*a != *b).then(|| (*b + (*b - *a), *a, *b)))
            })
            .inspect(|(anti, a, b)| println!("a{a} b{b} -> #{anti}"))
            .map(|(anti, _, _)| anti)
            .filter(|p| p.min(IVec2::ZERO) == IVec2::ZERO && p.max(bounds) == bounds)
            .collect();

        self.antinodes = antinodes;
    }
}
struct SparseGrid {
    groups: HashMap<char, Group>,
    bounds: IVec2,
}

impl SparseGrid {
    fn get_all_antinodes(&mut self) -> HashSet<IVec2> {
        self.groups
            .values_mut()
            .flat_map(|g| {
                g.find_antinodes(self.bounds);
                &g.antinodes
            })
            .filter(|p| p.min(IVec2::ZERO) == IVec2::ZERO && p.max(self.bounds) == self.bounds)
            .cloned()
            .collect()
    }

    #[allow(dead_code)]
    fn is_antenna(&self, pos: &IVec2) -> bool {
        self.groups.values().any(|g| g.antennas.contains(pos))
    }

    fn print_grid(&self) {
        // scale
        for x in 0..self.bounds.x {
            print!(
                "{}",
                match x % 10 {
                    0 => '|',
                    5 => '\'',
                    _ => ' ',
                }
            );
        }
        println!("");

        let mut multiple = Vec::<String>::new();

        // grid
        for row in 0..self.bounds.y {
            for col in 0..self.bounds.x {
                let pos = IVec2::new(row, col);

                let symbols = self
                    .groups
                    .values()
                    .flat_map(|g| {
                        g.antinodes
                            .iter()
                            .filter(|v| **v == pos)
                            .map(|_| Some('#'))
                            .chain(once(g.antennas.contains(&pos).then(|| g.id)))
                    })
                    .flat_map(|o| o)
                    .collect::<Vec<_>>();

                let symbol = match symbols.len() {
                    0 => '.',
                    1 => symbols[0],
                    _ => {
                        multiple.push(format!("{}: {:?}", pos, symbols));

                        // if multiple node on one pos, print antenna id
                        symbols
                            .iter()
                            .filter(|c| c.is_ascii_alphanumeric())
                            .last()
                            .unwrap_or(&'#')
                            .clone()
                    }
                };
                print!("{symbol}")
            }
            println!(" |{row:2}");
        }
        println!("");
        println!("{:?}", multiple);
    }
}

fn parse(input: &str) -> SparseGrid {
    let input = input.trim();
    let bounds = IVec2::new(
        input
            .lines()
            .next()
            .expect("invalid input")
            .len()
            .try_into()
            .expect("reasonable grid"),
        input.lines().count().try_into().expect("reasonable grid"),
    );

    let mut points = HashMap::with_capacity(input.len());
    for (id, point) in input.lines().enumerate().flat_map(|(row, l)| {
        l.chars().enumerate().flat_map(move |(col, c)| match c {
            '.' => None,
            // allow testoutput as input
            '#' => None,
            _ => Some((c, IVec2::new(row as i32, col as i32))),
        })
    }) {
        points
            .entry(id)
            .and_modify(|e: &mut Group| e.antennas.push(point))
            .or_insert(Group {
                id: id,
                antennas: vec![point],
                antinodes: vec![],
            });
    }

    SparseGrid {
        groups: points,
        bounds,
    }
}

#[tracing::instrument]
pub fn aoc_2024_08_a(input: &str) -> usize {
    let mut grid = parse(input);
    let all_antinodes = grid.get_all_antinodes();
    let mut antinodes = all_antinodes
        .iter()
        .filter(|p| !grid.is_antenna(p))
        .collect::<Vec<_>>();

    grid.print_grid();

    // antinodes.sort_by(|a, b| a.x.cmp(&b.x).then_with(|| a.y.cmp(&b.y)));
    // println!("all ({}): {:?}\n filtered ({}): {:?}",
    // all_antinodes.len(), all_antinodes, antinodes.len(), antinodes);

    // 425 is too high
    // 388 is too low (no overlap of node and antenna)
    antinodes.len()
}

#[tracing::instrument]
pub fn aoc_2024_08_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use glam::IVec2;
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT2, vec![IVec2::new(3,4), IVec2::new(4,8), IVec2::new(5,5),])]
    fn antenna_should_have_antennas(#[case] input: &str, #[case] expected: Vec<IVec2>) {
        let grid = super::parse(input);
        let Some(group) = grid.groups.get(&'a') else {
            panic!("no group a");
        };

        println!("{:?}", group);
        assert_eq!(group.antennas, expected);
    }

    #[rstest]
    #[case(TEST_INPUT2, 4)]
    fn antenna_should_have_n_antinodes(#[case] input: &str, #[case] expected: usize) {
        let mut grid = super::parse(input);
        let Some(group) = grid.groups.get_mut(&'a') else {
            panic!("no group a");
        };
        group.find_antinodes(IVec2::new(9, 9));

        println!("{:?}", group);
        assert_eq!(group.antinodes.len(), expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 14)]
    #[case(TEST_INPUT2, 4)]
    #[case(TEST_INPUT3, 3)]
    fn aoc_2024_08_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_08_a(input), expected);
    }

    #[test]
    fn aoc_2024_08_a() {
        assert_eq!(super::aoc_2024_08_a(super::INPUT), 0);
    }

    #[rstest]
    #[case("X, X", 0)]
    fn aoc_2024_08_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_08_b(input), expected);
    }

    #[test]
    fn aoc_2024_08_b() {
        assert_eq!(super::aoc_2024_08_b(super::INPUT), 0);
    }

    #[test]
    fn self_cross_should() {
        let an = vec![IVec2::new(3, 4), IVec2::new(4, 8), IVec2::new(5, 5)];
        let x = an
            .iter()
            .flat_map(|a| an.iter().flat_map(move |b| (*a != *b).then(|| (*a, *b))))
            .collect::<Vec<_>>();

        assert_eq!(
            x,
            vec![
                (IVec2::new(3, 4), IVec2::new(4, 8)),
                (IVec2::new(3, 4), IVec2::new(5, 5)),
                (IVec2::new(4, 8), IVec2::new(3, 4)),
                (IVec2::new(4, 8), IVec2::new(5, 5)),
                (IVec2::new(5, 5), IVec2::new(3, 4)),
                (IVec2::new(5, 5), IVec2::new(4, 8)),
            ]
        );
    }

    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const TEST_INPUT2: &str = "..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........";

    const TEST_INPUT3: &str = "..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........";
}
