// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2021/day/16
    Solution idea:

*/

pub fn aoc_2021_16_a(input: &str) -> usize {
    let bit_stream = input.trim().chars()
    .filter_map(|h| u8::from_str_radix(&h.to_string(), 16).ok())
    .collect::<Vec<_>>();
    0
}

pub fn aoc_2021_16_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_16_a_example() {
        assert_eq!(super::aoc_2021_16_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2021_16_a() {
       assert_eq!(super::aoc_2021_16_a(INPUT), 0);
    }
    
    #[test]
    fn aoc_2021_16_b_example() {
        assert_eq!(super::aoc_2021_16_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2021_16_b() {
        assert_eq!(super::aoc_2021_16_b(INPUT), 0);
    }

    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}



