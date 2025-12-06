// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/06
    Solution idea:

*/

use aoc_utils::grid::Grid;
use glam::IVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Number(u64),
    Plus,
    Asterisk,
}

fn parse1(input: &str) -> Vec<Vec<Entry>> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(' ')
                .flat_map(|s| match s {
                    "+" => Some(Entry::Plus),
                    "*" => Some(Entry::Asterisk),
                    "" => None,
                    n => Some(Entry::Number(n.parse::<u64>().expect("invalid number"))),
                })
                .collect()
        })
        .collect::<Vec<Vec<Entry>>>()
}

#[tracing::instrument]
pub fn aoc_2025_06_a(input: &str) -> u64 {
    let x = parse1(input);

    debug_assert!(
        x.iter().all(|r| r.len() == x[0].len()),
        "all rows must have the same number of columns"
    );

    let ops_row = x.len() - 1;
    // define operations now to ensure they are not created in the loop and both have the same type
    let add: fn(u64, u64) -> u64 = |a, b| a + b;
    let multiply: fn(u64, u64) -> u64 = |a, b| a * b;

    let results = (0..x[0].len())
        .map(|col| {
            let (f, neutral) = match x[ops_row][col] {
                Entry::Plus => (add, 0u64),
                Entry::Asterisk => (multiply, 1u64),
                _ => panic!("Invalid operator in ops_row"),
            };

            (0..x.len() - 1).fold(neutral, |acc, row| match x[row][col] {
                Entry::Number(n) => f(acc, n),
                _ => panic!("Invalid entry in number rows"),
            })
        })
        .collect::<Vec<u64>>();

    results.iter().sum::<u64>()
}

#[tracing::instrument]
pub fn aoc_2025_06_b(input: &str) -> u64 {
    let x = input.replace(" ", ".").parse::<Grid<char>>().expect("valid grid");
    println!("{}", x);

    // parse all columns right to left as numbers till we hit a space only column, then apply operator
    // numbers start at the top but they may be positioned randomly in the column. Collect in a vec and then build the number
    let mut col = x.width as i32 - 1;
    let ops_row = x.height as i32 - 1;
    let mut values = Vec::new();    
    let mut result = 0u64;

    let mut right_limit = col;
    while col > -1 {
        // collect number in this column
        let col_value = (0..x.height as i32 - 1) // ignore last row with operators
            .flat_map(|row| {
                if let Some(c) = x.get(IVec2 {
                    x: col,
                    y: row,
                }) && c.is_ascii_digit()
                {
                    Some(*c)
                } else {
                    None
                }
            })
            .collect::<String>();
            println!("col {} value '{}'", col, col_value);

        if ! col_value.is_empty()  {
            values.push(col_value.parse::<u64>().expect("valid number"));
        }

        // process collected value or apply operation to collected values if empty
        if col_value.is_empty() || col == 0 {
            // empty column, indicates end of group
            // get operator from ops_row and apply to all values collected so far
            result += (col..right_limit)
                .map(|op_col| 
                match x.get(IVec2::new(op_col, ops_row)) {
                    Some('+') => {println!("Adding values: {:?}", values); values.iter().sum()},
                    Some('*') => { println!("Multiplying values: {:?}", values); values.iter().product()},
                    None => {println!("out of bounds row {} col {}", ops_row, op_col); 0u64},
                   _ => 0u64
                }).sum::<u64>();
            // reset values for next group
            values.clear();
            right_limit = col;

        } 

        // move left to next column
        col -= 1;
    }

    dbg!(result)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 4277556)]
    fn aoc_2025_06_a_example(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(super::aoc_2025_06_a(input), expected);
    }

    #[test]
    fn aoc_2025_06_a() {
        assert_eq!(super::aoc_2025_06_a(super::INPUT), 6100348226985);
    }

    #[rstest]
    #[case(TEST_INPUT, 3263827)]
    fn aoc_2025_06_b_example(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(super::aoc_2025_06_b(input), expected);
    }

    #[test]
    fn aoc_2025_06_b() {
        assert_eq!(super::aoc_2025_06_b(super::INPUT), 12377473011151);
    }

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
