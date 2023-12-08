use std::fmt;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/07
    Solution idea:

*/
#[derive(Debug,PartialEq, Eq, PartialOrd, Ord)]
struct Card{
    rank: u8, // A has a rank higher than all other labels, 
    label: char,
}

impl Card {
    fn new(s:&str) -> Self { 
        let label = s.chars().next().unwrap().clone();
        let rank = "23456789TJQKA".find(label).expect("invald card label");

        Self { rank, label } 
    }
}

enum HandType {
    
}

#[derive(Debug, PartialEq, Eq)]
struct Hand{
    cards: [Card;5],

}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cards.partial_cmp(&other.cards)
    }
}



impl Hand {
    fn new(def: &str) -> Self { 
        
        Self { cards } 
    }
}

pub fn aoc_2023_07_a(input: &str) -> usize {
    // for line in input.trim().lines() {
    //     //
    // }
    0
}

pub fn aoc_2023_07_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_07_a_example() {
        assert_eq!(super::aoc_2023_07_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_07_a() {
       assert_eq!(super::aoc_2023_07_a(INPUT), 0);
    }
    
    #[test]
    fn aoc_2023_07_b_example() {
        assert_eq!(super::aoc_2023_07_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_07_b() {
        assert_eq!(super::aoc_2023_07_b(INPUT), 0);
    }

    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "";
}



