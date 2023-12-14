// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/14
    Solution idea:
    Just roll with it :)
*/

use std::collections::BTreeSet;

struct Panel{ 
    grid: Vec<Vec<char>>,
    rolling_stones: BTreeSet<(usize, usize)>,
}

impl Panel {
    fn get(&self, row: usize, col: usize) -> Option<&char> {
        self.grid.get(row).and_then(|r| r.get(col))
    }

}

impl FromStr for Panel {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut rolling_stones = BTreeSet::new();
        let grid = 
        input
            .lines().enumerate()
            .map(|(row, l)| l.trim().chars().enumerate().map(|(col, c)| match c {
                'O' => {
                    // record moving stones
                    rolling_stones.insert((row.clone(), col.clone()));

                    '.' // record position as free
                }
                '.' => '.',
                '#' => '#',
                _ => unreachable!(),
            }).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Ok(Panel(grid, rolling_stones ))
    }
}

pub fn aoc_2023_14_a(input: &str) -> usize {
    let panel : Panel = input.parse();
    0
}

pub fn aoc_2023_14_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_14_a_example() {
        assert_eq!(super::aoc_2023_14_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_14_a() {
       assert_eq!(super::aoc_2023_14_a(INPUT), 0);
    }
    
    #[test]
    fn aoc_2023_14_b_example() {
        assert_eq!(super::aoc_2023_14_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_14_b() {
        assert_eq!(super::aoc_2023_14_b(INPUT), 0);
    }

    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}



