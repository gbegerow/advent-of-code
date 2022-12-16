use petgraph::graph::Graph;
// #[allow(dead_code)]

pub fn aoc_2022_16_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn aoc_2022_16_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_16_a_example() {
        assert_eq!(super::aoc_2022_16_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_16_a() {
       assert_eq!(super::aoc_2022_16_a(include_str!("input.txt")), 0);
    }
    
    #[test]
    fn aoc_2022_16_b_example() {
        assert_eq!(super::aoc_2022_16_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_16_b() {
        assert_eq!(super::aoc_2022_16_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "";
}



