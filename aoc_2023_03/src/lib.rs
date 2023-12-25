use std::collections::HashMap;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/03
    Solution idea:

*/
#[derive(Debug)]
enum State {
    Outside,
    Inside {
        symbols_seen: Vec<(usize, usize, char)>,
        num: u32,
    },
}

#[derive(PartialEq, Eq)]
enum ScanRange {
    Start,
    Middle,
    End,
}

fn scan_for_symbols(
    field: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    range: ScanRange,
) -> Vec<(usize, usize, char)> {
    let (width, height) = (field[0].len(), field.len());
    let bounded_range = if x == 0 || x == width - 1 {
        ScanRange::Middle
    } else {
        range
    };
    // 0 - 1 is invalid in usize
    // match bounded_range {
    //     ScanRange::Start => vec![(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1),], // [
    //     ScanRange::Middle => vec![(x, y - 1), (x, y + 1)], // :
    //     ScanRange::End => vec![(x, y - 1), (x, y), (x, y + 1)], // |
    // };

    let mut neighbours = Vec::new();
    // Start
    if bounded_range == ScanRange::Start && y > 0 {
        neighbours.push((x - 1, y - 1));
    }
    if bounded_range == ScanRange::Start {
        neighbours.push((x - 1, y));
    }
    if bounded_range == ScanRange::Start && y < height - 1 {
        neighbours.push((x - 1, y + 1));
    }
    // Middle for all states
    if y > 0 {
        neighbours.push((x, y - 1));
    }
    if y < height - 1 {
        neighbours.push((x, y + 1));
    }
    // End
    if bounded_range == ScanRange::End {
        neighbours.push((x, y));
    }

    // we now know for sure neighbours contains only valid indexes
    // part b: if symbol is star memorize at which position the number has seen it
    neighbours
        .iter()
        .map(|(nx, ny)| (nx.clone(), ny.clone(), field[*ny][*nx].clone()))
        .filter(|&(_, _, c)| c != '.' && !c.is_numeric())
        .collect::<Vec<_>>()
}

fn add_if_adjacent(symbol_adjacent: bool, num: u32) -> u32 {
    // println!("number {} symbol {}", num, symbol_adjacent);

    if symbol_adjacent {
        num
    } else {
        0
    }
}

fn parse(input: &str) -> Vec<Vec<char>> {
    let field = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    // field.iter().for_each(|r| r.iter().for_each(|c| {print!("{}",c); println!();}));
    // println!("{:?}", field[0]);
    field
}

pub fn calc(input: &str) -> (u32, u32){
    let field = parse(input);
    let (width, height) = (field[0].len(), field.len());
    let mut sum_of_parts = 0;
    let mut symbols = HashMap::new();

    // scan all chars
    for y in 0..height {
        let mut state = State::Outside; // start every line outside

        for x in 0..width {
            let c = field[y][x].clone();
            // if c != '.' { println!("[{},{}]: '{}' {:?}", x, y, c, state);}

            let next_state: State;
            match state {
                State::Outside if c.is_numeric() => {
                    // println!("Start number");
                    let symbols_seen = scan_for_symbols(&field, x, y, ScanRange::Start);
                    let num = c.to_digit(10).unwrap();
                    next_state = State::Inside { symbols_seen, num };
                }

                State::Outside => next_state = State::Outside, // ignore any other character

                State::Inside { mut symbols_seen, num } if c.is_numeric() => {
                    // println!("Inside number");
                   symbols_seen.extend(scan_for_symbols(&field, x, y, ScanRange::Middle).iter());
                    let num = num * 10 + c.to_digit(10).unwrap();
                    next_state = State::Inside { symbols_seen, num };
                }

                State::Inside { mut symbols_seen, num } => {
                    // println!("End of number");
                    symbols_seen.extend(scan_for_symbols(&field, x, y, ScanRange::End).iter());

                    //part a
                    sum_of_parts += add_if_adjacent(symbols_seen.len() > 0, num);

                    // part b
                    for (x, y, c) in symbols_seen {
                        if c == '*' {
                            symbols.entry((x, y)).or_insert(Vec::new()).push(num);
                        }
                    }

                    next_state = State::Outside;
                }
            }
            state = next_state
        }
        // special case number at end of line
        match state {
            State::Inside { symbols_seen, num } => {
                // println!("End of number at end of line");
                //part a
                sum_of_parts += add_if_adjacent(symbols_seen.len() > 0, num);

                // part b
                for (x, y, c) in symbols_seen {
                    if c == '*' {
                        symbols.entry((x, y)).or_insert(Vec::new()).push(num);
                    }
                }
            }
            _ => (),
        }

        // println!("sum: {}", sum_of_parts);
    }

    println!("{:?}", symbols);
    let gears = symbols
        .values()
        .filter(|&v| v.len() == 2)
        .fold(0, |acc, v| acc + v[0] * v[1]);

    (sum_of_parts, gears)
}

pub fn aoc_2023_03_a(input: &str) -> u32 {
    let (sum_of_parts, _) = calc(input);

    sum_of_parts
}


pub fn aoc_2023_03_b(input: &str) -> u32 {
    // idea: collect symbols with positon in scan
    // in end of number collect every star in a HashMap<(x,y), Vec<number>>
    // at return get all entries where exactly two number has been memorized
    let (_, gears) = calc(input);

    gears
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn aoc_2023_03_a_example() {
        assert_eq!(super::aoc_2023_03_a(TEST_INPUT), 4361);
    }

    #[test]
    fn aoc_2023_03_a() {
        assert_eq!(super::aoc_2023_03_a(INPUT), 544433);
    }

    #[test]
    fn aoc_2023_03_b_example() {
        assert_eq!(super::aoc_2023_03_b(TEST_INPUT), 467835);
    }

    #[test]
    fn aoc_2023_03_b() {
        assert_eq!(super::aoc_2023_03_b(INPUT), 0);
    }

    #[rstest]
    #[case(0, 0)]
    #[case(0, 1)]
    #[case(0, 2)]
    #[case(0, 3)]
    fn field_correctly_parsed(#[case] row: usize, #[case] col: usize) {
        let field = super::parse(TEST_INPUT);
        assert!(field[row][col].is_numeric());
    }


    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";
}
