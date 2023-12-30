// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/12
    Solution idea:
    mask pattern
    2 possiibilies: 
    - numbers define regex on mask -> create Deterministic Finite Automata from pattern. 
        permutate all ? between . and # and count matches
        what kind of shortcuts are possible?
    - interpret as error correction code on binary numbers. 
        Convert mask to binary number. Iterate over binary. .=0, #=1 
        And Mask: ?=1 => skip if current & mask != current ???
        Or Mask: ?=0 => skip if mask | current != mask ???
        Permutate over all ? bits
        How to test for pattern?
*/





pub fn aoc_2023_12_a(input: &str) -> usize {
     let condition_records = input.trim().lines().flat_map(|l| {
        let (ps, gs) = l.trim().split_once(' ');
        // what data structure to use for the pattern? Array, binary number?
        let pattern = ps.trim().chars().collect::<Vec<_>>();
        let groups = gs.split(",").flat_map(|g| g.trim().parse::<u32>()).collect::<Vec<_>>(); 
        Some((pattern, groups))
    }
    ).collect::<Vec<_>>();

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


    const TEST_INPUT: &str = "
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";

    const TEST_INPUT_VALD: &str = "
    #.#.### 1,1,3
    .#...#....###. 1,1,3
    .#.###.#.###### 1,3,1,6
    ####.#...#... 4,1,1
    #....######..#####. 1,6,5
    .###.##....# 3,2,1";
}



