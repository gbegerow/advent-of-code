// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/21
    Solution idea:
    Cellular automata toggle state between (.|S) and O for steps waves

    Of course in part b everything explodes
    Infinity =  mod by width and height of grid
    We can only store all fields of few waves. Can we derive a cycle? Seems like game of life
    What about the hint? a list of his favorite numbers that are both perfect squares and perfect cubes
    26501365^(1/2) = 5147.947  26501365^(1/3) = 298.142
*/

use std::collections::BTreeSet;


/// find position of 'S' (start), panics if not found
fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize){
    for (row_num, row) in grid.iter().enumerate()  {
        if let Some(col_num) = row.iter().position(|&c| c == 'S') {
            return (row_num, col_num);
        }
    }
    unreachable!();
}

fn is_valid(grid: &Vec<Vec<char>>, row:usize, col:usize) -> bool {
    match grid[row][col] {
        '.' => true,
        'S' => true,
        _ => false
    }
}
fn get_valid_neighbours(grid: &Vec<Vec<char>>, pos: (usize, usize)) -> Vec<(usize, usize)>{
    let (row, col) =pos; 
    let (width, height) = (grid[0].len(), grid.len());
    let mut neighbours = Vec::new();
    if row > 0 && is_valid(grid, row-1, col){
        neighbours.push((row - 1, col));
    };
    if col<width && is_valid(grid, row, col+1){
        neighbours.push((row, col+1));
    }
    if row < height && is_valid(grid, row+1, col) {
        neighbours.push((row+1, col))
    }
    if col > 0 && is_valid(grid, row, col-1) {
        neighbours.push((row, col - 1));
    };

    neighbours
}

fn step_in(input: &str, num_of_waves: u32) -> usize {
    let grid: Vec<Vec<char>> = input.trim().lines().map(|l| l.trim().chars().collect()).collect();
    let start = find_start(&grid);
    let mut fields = BTreeSet::from([start;1]);
    
    // bfs / cellular automata
    let mut wave = 0;
    while wave < num_of_waves {
        wave += 1;
        let fields_of_wave = fields.iter()
            .flat_map(|&f| get_valid_neighbours(&grid, f))
            .collect::<BTreeSet<_>>();        
        fields = fields_of_wave;
        
        // vizualize (no fancy tui this time)
        // println!("Wave {}: {}", wave, fields.len());
        // for (row_num, row) in grid.iter().enumerate()  {
        //     for (col_num, field) in row.iter().enumerate()  {
        //         if fields.contains(&(row_num, col_num)) {
        //             print!("O");
        //         } else {
        //             print!("{}", field);
        //         }
        //     }
        //     println!();
        // }
    }
    
    fields.len()
}

pub fn aoc_2023_21_a(input: &str) -> usize {
    step_in(input, 64)
}

pub fn aoc_2023_21_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    use crate::step_in;

    #[test]
    fn aoc_2023_21_a_example() {
        assert_eq!(step_in(TEST_INPUT, 6), 16);
    }

    #[test]
    fn aoc_2023_21_a() {
       assert_eq!(super::aoc_2023_21_a(INPUT), 0);
    }
    
    #[test]
    fn aoc_2023_21_b_example() {
        assert_eq!(super::aoc_2023_21_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_21_b() {
        assert_eq!(super::aoc_2023_21_b(INPUT), 0);
    }

    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    ...........
    .....###.#.
    .###.##..#.
    ..#.#...#..
    ....#.#....
    .##..S####.
    .##..#...#.
    .......##..
    .##.#.####.
    .##..##.##.
    ...........";
}



