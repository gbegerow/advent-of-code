use std::collections::HashSet;

// #[allow(dead_code)]
// Find the task under https://adventofcode.com/2021/day/13
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate { x: i32, y: i32 }



fn parse(input: &str) -> Result<(HashSet<Coordinate>, Vec<Coordinate>),()>{
    if let Some((coord_defs, fold_defs)) = input.trim().split_once("\n\n") {
        let coords = coord_defs.lines()
            .filter_map(|l| l.split_once(","))
            .map(|(xs, ys)| Coordinate { x: xs.trim().parse().unwrap(), y: ys.trim().parse().unwrap() })
            .collect::<HashSet<_>>();

        let folds = fold_defs.lines()
            .filter_map(|l| l.split_once("="))
            .filter_map(|(f, s)| match f {
                "fold along y" => Some(Coordinate { x: 0, y: s.trim().parse().unwrap() }),
                "fold along x" => Some(Coordinate { x: s.trim().parse().unwrap(), y: 0 }),
                _ => None
            })
            .collect::<Vec<_>>();

        // println!("Coords {:?}\nFolds {:?}", coords, folds);
        Ok((coords, folds))
    } else {
        Err(())
    }
}

fn folded_points(mut coords:  HashSet<Coordinate>, folds: Vec<Coordinate>, take: usize) -> HashSet<Coordinate> {
    for fold in folds.iter().take(take) {
        let mut folded = HashSet::with_capacity(coords.len());

        if fold.x > 0 {
            let fold_x2 = fold.x + fold.x;
            for c in coords {
                if c.x > fold.x {
                    folded.insert(Coordinate { x: fold_x2 - c.x, y: c.y });
                } else {
                    folded.insert(c.clone());
                }
            }
        } else if fold.y > 0 {
            let fold_x2 = fold.y + fold.y;
            for c in coords {
                if c.y > fold.y {
                    folded.insert(Coordinate { x: c.x, y: fold_x2 - c.y });
                } else {
                    folded.insert(c.clone());
                }
            }
        }
        coords = folded
    }
    coords
}

fn draw(coords:HashSet<Coordinate>) {
    println!("{:?}", coords);
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    let max_y = coords.iter().map(|c| c.y).max().unwrap();

    println!("x: {} y: {}", max_x, max_y);

    for y in 0..=max_y{
        for x in 0..=max_x{
            match coords.get(&Coordinate {x: x, y: y}) {
                Some(_) => print!("#"),
                None => print!("."),
            }
            // if x % 5 == 0 { print!(" ");}
        }
        println!();
    }
}

pub fn aoc_2021_13_a(input: &str) -> usize {
    let (coords, folds) = parse(input).unwrap();
    let folded = folded_points(coords, folds, 1);
    folded.len()
}

pub fn aoc_2021_13_b(input: &str) -> usize {
    let (coords, folds) = parse(input).unwrap();
    let take = folds.len();
    let folded = folded_points(coords, folds, take);
    draw(folded);
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_13_a_example() {
        assert_eq!(super::aoc_2021_13_a(TEST_INPUT), 17);
    }

    #[test]
    fn aoc_2021_13_a() {
       assert_eq!(super::aoc_2021_13_a(INPUT), 781);
    }
    
    #[test]
    fn aoc_2021_13_b_example() {
        assert_eq!(super::aoc_2021_13_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2021_13_b() {
        assert_eq!(super::aoc_2021_13_b(INPUT), 0);
        /* PERCGJPB

###.. ####. ###.. .##.. .##.. ..##. ###.. ###.
#..#. #.... #..#. #..#. #..#. ...#. #..#. #..#
#..#. ###.. #..#. #.... #.... ...#. #..#. ###.
###.. #.... ###.. #.... #.##. ...#. ###.. #..#
#.... #.... #.#.. #..#. #..#. #..#. #.... #..#
#.... ####. #..#. .##.. .###. .##.. #.... ###.


         */

    }

    
    const INPUT: &str  = include_str!("input.txt");

    const TEST_INPUT: &str = "
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";
}



