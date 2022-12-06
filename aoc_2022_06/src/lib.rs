use std::{collections::HashMap};



pub fn aoc_2022_06_a(input: &str) -> usize {
    if let Some(value) = find_distinct_window(input, 4) {
        return value;
    }
    0
}

fn find_distinct_window(input: &str, win_size:usize) -> Option<usize> {
    let buffer = input.trim().as_bytes(); // all ascii lowercase so bytes is fine
    let mut counter = HashMap::with_capacity(26);

    // slide window over buffer
    for pos in 0..buffer.len() {
        // current goes in
        let current = buffer[pos] as char;
        *counter.entry(current).or_insert(0) +=1;
             
        if pos < win_size { continue; } // not enough input yet, no signol possible

        //first in window goes out 
        let first = buffer[pos - win_size] as char;
        *counter.entry(first).or_insert(0) -=1;

        // invariant sum(values)=win_size
        // println!("Pos {} Current {} Counter {:?}", pos, current, counter);

        // if no counter over 1, we have the signal
        if counter.values().all(|v| v < &2) {
            return Some(pos+1); // report 1 based index
        }
    }
    None
}

pub fn aoc_2022_06_b(input: &str) -> usize {
    if let Some(value) = find_distinct_window(input, 14) {
        return value;
    }
    0
}




#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_06_a_example() {
        assert_eq!(super::aoc_2022_06_a(TEST_INPUT), 7);
    }

    #[test]
    fn aoc_2022_06_a() {
       assert_eq!(super::aoc_2022_06_a(include_str!("input.txt")), 1356);
    }
    
    #[test]
    fn aoc_2022_06_b_example() {
        assert_eq!(super::aoc_2022_06_b(TEST_INPUT), 19);
    }

    #[test]
    fn aoc_2022_06_b() {
        assert_eq!(super::aoc_2022_06_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
}



