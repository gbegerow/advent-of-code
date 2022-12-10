pub fn aoc_2021_01_a(input: &str) -> u32 {
    let numbers = parse_numbers(input);
    let mut count = 0;
    for i in 1..numbers.len() - 1 {
        if numbers[i-1] < numbers[i] { count += 1; }
    }
    count
}

pub fn aoc_2021_01_b(input: &str) -> u32 {
    let numbers = parse_numbers(input);
    let mut count = 0;
        let mut prev : Option<i32> = None;
        
        for slice in numbers.windows(3) {
            let sum = slice.iter().sum();
            
            if let Some(p) = prev { 
                if sum > p {
                    count += 1;
                }
            }
            prev = Some(sum);
        }
        count
}

fn parse_numbers(input: &str) -> Vec<i32>{
    input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_01_a_example() {
        assert_eq!(super::aoc_2021_01_a(TEST_INPUT), 6);
    }

    #[test]
    fn aoc_2021_01_a() {
       assert_eq!(super::aoc_2021_01_a(include_str!("input.txt")), 1400);
    }
    
    #[test]
    fn aoc_2021_01_b_example() {
        assert_eq!(super::aoc_2021_01_b(TEST_INPUT), 5);
    }

    #[test]
    fn aoc_2021_01_b() {
        assert_eq!(super::aoc_2021_01_b(include_str!("input.txt")), 1429);
    }

    const TEST_INPUT: &str = "199
    200
    208
    210
    200
    207
    240
    269
    260
    263";
}



