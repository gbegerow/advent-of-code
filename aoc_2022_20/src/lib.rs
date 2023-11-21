// #[allow(dead_code)]

use std::collections::VecDeque;
pub fn aoc_2022_20_a(input: &str) -> isize {
    let original: Vec<isize> = input.trim().lines().filter_map(|s| s.trim().parse().ok()).collect();
    let len = original.len();
    let mut mixing_bowl = VecDeque::from(original.clone());

    let circular_index = |ci, offset| ((ci as isize + offset) % len as isize) as usize;
    for i in original{
        // naive take the first occurence of the number. How to handle doubles?
        if let Some(index) = mixing_bowl.iter().position(|j| *j==i){
            let new_index = circular_index(index, i);
            mixing_bowl.remove(index);
            mixing_bowl.insert(new_index, i);
        }
    }
    
    let zero = mixing_bowl.iter().position(|j| j==&0).unwrap();
    mixing_bowl[circular_index(zero, 1000)] +
    mixing_bowl[circular_index(zero, 2000)] +
    mixing_bowl[circular_index(zero, 3000)] 

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
    fn input_has_doubles(){
        let input = include_str!("input.txt");
        assert_ne!(
            input.trim().lines().filter_map(|s| s.trim().parse().ok()).collect::<Vec<i32>>().len(), // 5000
            input.trim().lines().filter_map(|s| s.trim().parse().ok()).collect::<HashSet<i32>>().len() // 3606
        )
    } 
}



