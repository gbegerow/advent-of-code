use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn part_a(input: &str) -> u32 {
    let mut sum = 0;
    for line in input
        .trim()
        .lines()
        .map(|l| l.trim())
        .map(|l| l.split_at(l.len() / 2))
    {
        let first: HashSet<char> = HashSet::from_iter(line.0.chars());
        let second: HashSet<char> = HashSet::from_iter(line.1.chars());
        let common = first
            .intersection(&second)
            .next()
            .expect("No common item found");

        // println!("{common}");

        let c = to_code(common);

        sum += c;
    }
    sum
}

fn to_code(common: &char) -> u32 {
    if common.is_ascii_lowercase() {
        1 + *common as u32 - 'a' as u32
    } else if common.is_ascii_uppercase() {
        27 + *common as u32 - 'A' as u32
    } else {
        0
    }
}

pub fn part_b(input: &str) -> u32 {
    let mut sum = 0;

    let group_size = 3;
    for group_members in &input.lines().chunks(group_size) {
        // chained intersection of HashSets get readability into the drain very fast (and my sanity too)
        // just count the chars in the chunks but only count once per chunk...

        let mut counter: HashMap<char, u32> = HashMap::new();
        for chunk in group_members {
            for c in chunk.trim().chars().unique() {                
                *counter.entry(c.clone()).or_insert(0) += 1;
            }
        }

        // println!("{:?}", counter);
        if let Some(common) = counter.into_iter().find(|x| x.1 == group_size as u32) {
            let c = to_code(&common.0);

            sum += c;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn aoc_2022_03a_example() {
        // paste test input
        assert_eq!(super::part_a(TEST_INPUT), 157);
    }

    #[test]
    fn aoc_2022_03a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 8401);
    }

    #[test]
    fn aoc_2022_03b_example() {
        // paste test input
        assert_eq!(super::part_b(TEST_INPUT), 70);
    }

    #[test]
    fn aoc_2022_03b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 2641);
    }

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw";
}
