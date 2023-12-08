// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/08
    Solution idea:

*/

pub fn aoc_2023_08_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn aoc_2023_08_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_08_a_example() {
        assert_eq!(super::aoc_2023_08_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_08_a() {
       assert_eq!(super::aoc_2023_08_a(INPUT), 0);
    }
    
    #[test]
    fn aoc_2023_08_b_example() {
        assert_eq!(super::aoc_2023_08_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_08_b() {
        assert_eq!(super::aoc_2023_08_b(INPUT), 0);
    }

    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}



