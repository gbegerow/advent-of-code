// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/9999/day/99
    Solution idea:

*/

pub fn aoc_2020_01_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn aoc_2020_01_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2020_01_a_example() {
        assert_eq!(super::aoc_2020_01_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2020_01_a() {
       assert_eq!(super::aoc_2020_01_a(INPUT), 0);
    }
    
    #[test]
    fn aoc_2020_01_b_example() {
        assert_eq!(super::aoc_2020_01_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2020_01_b() {
        assert_eq!(super::aoc_2020_01_b(INPUT), 0);
    }

    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}



