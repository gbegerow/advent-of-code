pub fn aoc_2022_08_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn aoc_2022_08_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_08_a_example() {
        assert_eq!(super::aoc_2022_08_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_08_a() {
       assert_eq!(super::aoc_2022_08_a(include_str!("input.txt")), 0);
    }
    
    #[test]
    fn aoc_2022_08_b_example() {
        assert_eq!(super::aoc_2022_08_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_08_b() {
        assert_eq!(super::aoc_2022_08_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "";
}



