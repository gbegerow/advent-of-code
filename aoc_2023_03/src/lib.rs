// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/03
    Solution idea:

*/
#[derive(Debug)]
enum State {
    Outside,
    Inside { symbol_adjacent: bool, num: u32 },
}

#[derive(PartialEq, Eq)]
enum ScanRange {
    Start,
    Middle,
    End,
}

fn scan_for_symbols(field: &Vec<Vec<char>>, x: usize, y: usize, range: ScanRange) -> bool {
    let (width, height) = (field[0].len(), field.len());
    let bounded_range = if x == 0 || x == width -1 { ScanRange::Middle } else {range};
    // 0 - 1 is invalid in usize    
    // match bounded_range {
    //     ScanRange::Start => vec![(x - 1, y - 1), (x - 1, y), (x - 1, y + 1), (x, y - 1), (x, y + 1),], // [
    //     ScanRange::Middle => vec![(x, y - 1), (x, y + 1)], // :
    //     ScanRange::End => vec![(x, y - 1), (x, y), (x, y + 1)], // |
    // };

    let mut neighbours = Vec::new();
    // Start 
    if bounded_range == ScanRange::Start && y > 0        { neighbours.push((x - 1, y - 1));}
    if bounded_range == ScanRange::Start                 { neighbours.push((x - 1, y    ));}
    if bounded_range == ScanRange::Start && y < height-1 { neighbours.push((x - 1, y + 1));}
     // Middle for all states
    if                                      y > 0        { neighbours.push((x,     y - 1));}
    if                                      y < height-1 { neighbours.push((x,     y + 1));}
    // End
    if bounded_range == ScanRange::End                   { neighbours.push((x,     y    ));}
    
    // we now know for sure neighbours contains only valid indexes
    let symbols =neighbours
        .iter()
        .filter(|(nx, ny)| field[*ny][*nx] != '.' && !field[*ny][*nx].is_numeric())
        .collect::<Vec<_>>();

    symbols.len() > 0    
}

fn add_if_adjacent(symbol_adjacent: bool, num: u32) -> u32{
    // println!("number {} symbol {}", num, symbol_adjacent);
   
    if symbol_adjacent {
        num
    } else {0}
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

pub fn aoc_2023_03_a(input: &str) -> u32 {
    let field = parse(input);
    let (width, height) = (field[0].len(), field.len());
    let mut sum_of_parts = 0;

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
                    let symbol_adjacent = scan_for_symbols(&field, x, y, ScanRange::Start);
                    let num = c.to_digit(10).unwrap();
                    next_state = State::Inside { symbol_adjacent, num };
                },
                
                State::Outside => {next_state = State::Outside}, // ignore any other character
                
                State::Inside { symbol_adjacent, num}  if c.is_numeric() => {
                    // println!("Inside number");
                    let symbol_adjacent =
                        symbol_adjacent || scan_for_symbols(&field, x, y, ScanRange::Middle);
                    let num = num * 10 + c.to_digit(10).unwrap();
                    next_state = State::Inside { symbol_adjacent, num };
                },

                State::Inside { symbol_adjacent, num} => {
                    // println!("End of number");
                    let symbol_adjacent = symbol_adjacent 
                            || scan_for_symbols(&field, x, y, ScanRange::End);
                    sum_of_parts += add_if_adjacent(symbol_adjacent, num);
                    next_state = State::Outside;
                },
            }
            state = next_state
        }
        // special case number at end of line
        match state {
                State::Inside { symbol_adjacent, num, } => {
                    // println!("End of number at end of line");
                    sum_of_parts += add_if_adjacent(symbol_adjacent, num);
            },
            _ => (),
        }

        // println!("sum: {}", sum_of_parts);
    }

    sum_of_parts
}



pub fn aoc_2023_03_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_03_a_example() {
        assert_eq!(super::aoc_2023_03_a(TEST_INPUT), 4361);
    }

    #[test]
    fn aoc_2023_03_a() {
        assert_eq!(super::aoc_2023_03_a(INPUT), 0);
    }

    #[test]
    fn aoc_2023_03_b_example() {
        assert_eq!(super::aoc_2023_03_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_03_b() {
        assert_eq!(super::aoc_2023_03_b(INPUT), 0);
    }

    #[test]
    fn field_correctly_parsed(){
        let field = super::parse(TEST_INPUT);
        assert!(field[0][0].is_numeric());
        assert!(field[0][1].is_numeric());
        assert!(field[0][2].is_numeric());
        assert!(!field[0][3].is_numeric());
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
