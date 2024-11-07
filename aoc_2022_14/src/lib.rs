use std::collections::HashMap;

// use crossterm::{
//     cursor,
//     style::{self, Stylize},
//     terminal, QueueableCommand
// };
// use std::io::{stdout, Write};

// use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[allow(dead_code)]
enum Tile {
    Air,
    Rock,
    Sand,
}

fn unroll(s: &Coordinate, e: &Coordinate) -> Vec<(Coordinate, Tile)> {
    let dx = (s.x - e.x).signum();
    let dy = (s.y - e.y).signum();

    let mut current = s.clone();
    let mut ret = Vec::with_capacity(if dx > dy { dx } else { dy } as usize);
    while &current != e {
        // TODO Box???
        let next = Coordinate {
            x: &current.x + dx,
            y: &current.y + dy,
        };
        ret.push((current.to_owned(), Tile::Rock));
        current = next;
    }
    ret
}

fn parse(input: &str) -> HashMap<Coordinate, Tile> {
    input
        .trim()
        .lines()
        .flat_map(|l| {
            l.split("->")
                .filter_map(|p| p.split_once(","))
                .map(|(xs, ys)| Coordinate {
                    x: xs.trim().parse().unwrap(),
                    y: ys.trim().parse().unwrap(),
                })
                .collect::<Vec<_>>()
                .chunks(2)
                .flat_map(|slice| match slice {
                    // make pattern
                    &[s, e] => unroll(&s, &e),
                    _ => panic!("invalid chunk size"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

// fn draw_cave(map: &HashMap<Coordinate, Tile>, spawner: &Coordinate, location: Option<&Coordinate>) -> std::io::Result<()> {
//     let mut stdout = stdout();

//     stdout.queue(terminal::Clear(terminal::ClearType::All))?;
//     // stdout.queue(cursor::MoveToNextLine(1))?;
//     // assume row from 460 - 540, column from 0 - 200, origin top left
//     let row = |c:&Coordinate| c.x as u16 - 460;
//     let col = |c:&Coordinate| c.y as u16 ;

//     for (c, t) in map {
//         stdout.queue(cursor::MoveTo(row(c), col(c)))?;
//         match t {
//             Tile::Air => stdout.queue(style::PrintStyledContent("░".black()))?,
//             Tile::Rock => stdout.queue(style::PrintStyledContent("█".grey()))?,
//             Tile::Sand => stdout.queue(style::PrintStyledContent("o".yellow()))?,
//         };
//     }

//     stdout.queue(cursor::MoveTo(row(spawner), col(spawner)))?
//         .queue(style::PrintStyledContent("+".blue()))?;

//     if let Some(current) = location {
//         stdout.queue(cursor::MoveTo(row(current), col(current)))?
//             .queue(style::PrintStyledContent("*".green()))?;
//     }

//     stdout.queue(cursor::MoveToNextLine(1))?;
//     stdout.flush()?;
//     Ok(())
// }

pub fn aoc_2022_14_a(input: &str) -> usize {
    let map = parse(input);
    let spawner = Coordinate { x: 500, y: 0 };

    // draw_cave(&map, &spawner, None).unwrap_or_default();
    0
}

pub fn aoc_2022_14_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_14_a_example() {
        assert_eq!(super::aoc_2022_14_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_14_a() {
        assert_eq!(super::aoc_2022_14_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn aoc_2022_14_b_example() {
        assert_eq!(super::aoc_2022_14_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_14_b() {
        assert_eq!(super::aoc_2022_14_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
    503,4 -> 502,4 -> 502,9 -> 494,9";
}
