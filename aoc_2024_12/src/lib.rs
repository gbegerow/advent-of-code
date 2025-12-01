// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/12
    Solution idea:
    linear equations with 2 unknown
*/
use aoc_utils::grid::Grid;
use glam::IVec2;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
struct Region {
    id: u32,
    symbol: char,
    area: u32,
    perimeter: u32,
}

impl Region {
    fn score(&self) -> u32 {
        (self.area * self.perimeter) as u32
    }
}
impl fmt::Debug for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}: [a: {}, p: {}] => {}",
            self.id,
            self.symbol
            self.area,
            self.perimeter,
            self.score()
        )
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}: [a: {}, p: {}]", self.id, self.symbol, self.area, self.perimeter)
    }
}


#[tracing::instrument]
pub fn aoc_2024_12_a(input: &str) -> usize {
    // Grid is just a convenient way to parse and visualize the input
    // a two line buffer and adding one character at a time would be sufficient
    let grid = input.parse::<Grid<char>>().expect("valid grid");
    println!("{}", grid);


    // each region counts for its own, so we can't use char as index
    let mut regions = Vec<Region>::with_capacity(200);
    let mut current: Region = Region { id: 0, symbol: grid.get(IVec2::new(0, 0)).copied().unwrap_or('*'), area: 0, perimeter: 0 };

    // We only need to check left and up, because we will visit every cell
    let NEIGHBOURS: [IVec2; 4] = [
        IVec2::new(-1, 0), // left
        IVec2::new(0, -1), // up
        ];

    for (pos, c) in grid.iter_with_positions() {
        if !c.is_ascii_uppercase() {
            panic!("unexpected character in input: {c}");
        }
        

        for delta in &NEIGHBOURS {
            if let Some(neighbour) = grid.get(pos + *delta) {
                if *neighbour != *c {
                    measurements[to_index(*c)].perimeter += 1;
                    // perimeter goes both ways, so we need to update the neighbour as well
                    // we know it is a capital letter, because of the earlier check
                    measurements[to_index(*neighbour)].perimeter += 1;
                }
            } else {
                // edge of the grid, so we have a perimeter
                measurements[to_index(*c)].perimeter += 1;
            }
        }

        // the last cell in a row has no right neighbour, so we need to add those perimeters
        if pos.x as usize == grid.width - 1 {
            measurements[to_index(*c)].perimeter += 1;
        }
        // bottom row needs an additional perimeter too
        if pos.y as usize == grid.height - 1 {
            measurements[to_index(*c)].perimeter += 1;
        }

        // println!(
        //     "{} at {} => {:?}",
        //     c,
        //     pos,
        //     measurements[to_index(*c)]
        // );
    }

    println!(
        "{}",
        measurements
            .iter()
            .enumerate()
            .filter(|(_, m)| m.area > 0)
            .map(|(i, m)| format!("'{}': {} => {}", (b'A' + i as u8) as char, m, m.score()))
            .collect::<Vec<_>>()
            .join("; ")
    );

    return measurements
        .iter()
        .filter(|m| m.area > 0)
        .map(|m| (m.area * m.perimeter) as usize)
        .sum();
}

#[tracing::instrument]
pub fn aoc_2024_12_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 140)]
    #[case(TEST_INPUT_2, 1930)]
    #[case(TEST_INPUT_3, 772)]
    fn aoc_2024_12_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_12_a(input), expected);
    }

    #[test]
    fn aoc_2024_12_a() {
        assert_eq!(super::aoc_2024_12_a(super::INPUT), 0);
    }

    #[rstest]
    #[case("X, X", 0)]
    fn aoc_2024_12_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_12_b(input), expected);
    }

    #[test]
    fn aoc_2024_12_b() {
        assert_eq!(super::aoc_2024_12_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "
        AAAA
        BBCD
        BBCC
        EEEC";

    const TEST_INPUT_2: &str = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    const TEST_INPUT_3: &str = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
}
