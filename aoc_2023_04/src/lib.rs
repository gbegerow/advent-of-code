// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/04
    Solution idea:

*/

use std::collections::{HashSet, HashMap};
use itertools::Itertools;

fn parse(input: &str) -> HashMap<u32, usize> {
    input
        .trim()
        .lines()
        .flat_map(|l| l.split_once(":"))
        .map(|(game_num, g)| {
            (game_num.split(" ").flat_map(|s| s.parse()).next().unwrap(),
            g.split("|")
                .map(|h| {
                    h.trim()
                        .split(" ")
                        .flat_map(|n| n.parse::<u32>())
                        .collect::<HashSet<_>>()
                }).collect_tuple::<(_,_)>().expect("winning | own")
            )
        })
        .map(|(g, (w, o))| (g, w.intersection(&o).count()))
        .collect()
}

pub fn aoc_2023_04_a(input: &str) -> usize {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    parse(input).iter()
        .map(|(_, &n)| if n > 0 {1 << n-1} else {0})
        .sum()
                
}


pub fn aoc_2023_04_b(input: &str) -> u32 {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        let group_mapping = parse(input);
        
        let mut games:Vec<_> = group_mapping.keys().collect();
        games.sort();
        let mut counter = games.iter().map(|&g| (g.clone(), 1)).collect::<HashMap<u32,u32>>();
        // println!("Before game: {:?}  counter: {:?}", group_mapping, counter);

        // iterate ordered
        for game_num in games {
            let gain = group_mapping[game_num];

            for i in game_num+1..game_num + 1 + gain as u32 {
                *counter.entry(i).or_insert(0) += counter[game_num];
            }
            // println!("After game {}[{}]: {:?} ∑ {}", game_num, counter[game_num] , counter, counter.values().sum::<u32>());
        }
        counter.values().sum()
}


#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_04_a_example() {
        assert_eq!(super::aoc_2023_04_a(TEST_INPUT), 13);
    }

    #[test]
    fn aoc_2023_04_a() {
        assert_eq!(super::aoc_2023_04_a(INPUT), 22674);
    }

    #[test]
    fn aoc_2023_04_b_example() {
        assert_eq!(super::aoc_2023_04_b(TEST_INPUT), 30);
    }

    #[test]
    fn aoc_2023_04_b() {
        assert_eq!(super::aoc_2023_04_b(INPUT), 5747443);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
}
