// #[allow(dead_code)]

// use ahash::HashMap;
use std::collections::VecDeque;

pub fn aoc_2022_20_a(input: &str) -> i64 {
    let original: Vec<i64> = input
        .trim()
        .lines()
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    println!("{original:?}");

    let len = original.len() as i64;
    let mut mixing_bowl = VecDeque::from(original.clone());

    let circular_index = |ci: i64, offset: i64| ((ci + offset).rem_euclid(len) as i64) as usize;
    for i in original {
        // naive take the first occurence of the number. How to handle doubles?
        if let Some(index) = mixing_bowl.iter().position(|j| *j == i) {
            let new_index = circular_index(index as i64, i);
            mixing_bowl.remove(index);
            mixing_bowl.insert(new_index, i);
            println!("{mixing_bowl:?}");
        }
    }

    let zero = mixing_bowl.iter().position(|j| j == &0).unwrap() as i64;
    mixing_bowl[circular_index(zero, 1000)]
        + mixing_bowl[circular_index(zero, 2000)]
        + mixing_bowl[circular_index(zero, 3000)]

    // Operations:
    // Move: add n to position
    // Find: get number at position
    // Data structure:
    //  Map<n, pos>
    //      Move: O(1), Find: O(n)
    //  Array<n>
    //      Move: O(n) but highly optimized, Find: O(1)
    // let mut positions = input
    //     .trim()
    //     .lines()
    //     .flat_map(|l| l.trim().parse::<i32>().ok())
    //     .enumerate()
    //     .map(|(p, n)| (n, p as i32))
    //     .collect::<HashMap<_, _>>();
    // println!("{positions:?}");

    // let len = positions.len() as i32;
    // let numbers: Vec<_> = positions.keys().copied().collect();

    // // Mix

    // for n in &numbers {
    //     let new_pos = (positions[n] + n).rem_euclid(len);
    //     for e in positions.iter_mut() {
    //         match e {
    //             // set new position of n
    //             (k, v) if k == n => *v = new_pos,
    //             // If we would insert, we had to move all on higher pos by one
    //             (_, v) if *v >= new_pos => *v = (*v + 1).rem_euclid(len),
    //             _ => (),
    //         }
    //     }

    //     // positions
    //     //     .entry(*n)
    //     //     .and_modify(|pos| *pos = (*pos + n).rem_euclid(len));

    //     println!("Moved {n:2}: {positions:?}");
    // }

    // let mut sorted = numbers.clone();
    // sorted.sort_by(|a, b| positions[a].cmp(&positions[b]));
    // println!("{sorted:?}");
    // 0
}

pub fn aoc_2022_20_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    #[test]
    fn aoc_2022_20_a_example() {
        assert_eq!(super::aoc_2022_20_a(TEST_INPUT), 3);
    }

    #[test]
    fn aoc_2022_20_a() {
        assert_eq!(super::aoc_2022_20_a(include_str!("input.txt")), 0);
    }

    #[test]
    fn aoc_2022_20_b_example() {
        assert_eq!(super::aoc_2022_20_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_20_b() {
        assert_eq!(super::aoc_2022_20_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "1
    2
    -3
    3
    -2
    0
    4";

    // ------------------------ Unit Tests ------------------------------------------
    #[test]
    fn input_has_doubles() {
        let input = include_str!("input.txt");
        assert_ne!(
            input
                .trim()
                .lines()
                .filter_map(|s| s.trim().parse().ok())
                .collect::<Vec<i32>>()
                .len(), // 5000
            input
                .trim()
                .lines()
                .filter_map(|s| s.trim().parse().ok())
                .collect::<HashSet<i32>>()
                .len() // 3606
        )
    }
}
