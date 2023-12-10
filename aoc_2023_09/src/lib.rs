// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/09
    Solution idea:

*/
fn differenc(org : Vec<i64>) -> Vec<i64> {
    org.windows(2).map(|slice| slice[1]-slice[0]).collect()
}

pub fn aoc_2023_09_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn aoc_2023_09_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_09_a_example() {
        assert_eq!(super::aoc_2023_09_a(TEST_INPUT), 114);
    }

    #[test]
    fn aoc_2023_09_a() {
       assert_eq!(super::aoc_2023_09_a(INPUT), 0);
    }
    
    #[test]
    fn aoc_2023_09_b_example() {
        assert_eq!(super::aoc_2023_09_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_09_b() {
        assert_eq!(super::aoc_2023_09_b(INPUT), 0);
    }

    
    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
}



