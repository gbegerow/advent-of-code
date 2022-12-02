pub fn part_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn part_b(input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
fn aoc_2022_02a_example() {
        // paste test input
        assert_eq!(super::part_a(TEST_INPUT), 0);
    }


    #[test]
fn aoc_2022_02a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 0);
    }

    
    #[test]
fn aoc_2022_02b_example() {
        // paste test input
        assert_eq!(super::part_b(TEST_INPUT), 0);
    }


    #[test]
fn aoc_2022_02b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "";
}



