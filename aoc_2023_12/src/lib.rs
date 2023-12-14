// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/12
    Solution idea:
    mask pattern
    2 possiibilies: 
    - numbers define regex on mask -> create Deterministic Finite Automata from pattern. 
        permutate all ? between . and # and count matches
    - interpret as error correction code on binary numbers. 
        Convert mask to binary number. Iterate over binary. .=0, #=1 
        And Mask: ?=1 => skip if current & mask != current ???
        Or Mask: ?=0 => skip if mask | current != mask ???
        Permutate over all ? bits
        How to test for pattern?
*/

pub fn aoc_2023_12_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn aoc_2023_12_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_12_a_example() {
        assert_eq!(super::aoc_2023_12_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_12_a() {
       assert_eq!(super::aoc_2023_12_a(INPUT), 0);
    }
    
    #[test]
    fn aoc_2023_12_b_example() {
        assert_eq!(super::aoc_2023_12_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_12_b() {
        assert_eq!(super::aoc_2023_12_b(INPUT), 0);
    }

    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}



