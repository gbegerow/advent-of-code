use std::collections::HashMap;

pub fn aoc_2021_12_a(input: &str) -> usize {
    let mut edges = HashMap::new();
    for line in input.trim().lines() {
        if let Some((from, to)) = line.trim().split_once("-") {
        //    println!("{from} -> {to}");
            edges.entry(from).or_insert(Vec::new()).push(to);
        }
    }
    println!("{:?}", edges);
    0
}

pub fn aoc_2021_12_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_12_a_example() {
        assert_eq!(super::aoc_2021_12_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2021_12_a() {
       assert_eq!(super::aoc_2021_12_a(include_str!("input.txt")), 0);
    }
    
    #[test]
    fn aoc_2021_12_b_example() {
        assert_eq!(super::aoc_2021_12_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2021_12_b() {
        assert_eq!(super::aoc_2021_12_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";
}



