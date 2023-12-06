// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/06
    Solution idea:
    Symmetric Bell Curve
15
14
13
12    **
11
10   *  *
 9   ^       <--record r
 8   l
 7         
 6  *    *
 5
 4
 3
 2
 1
 0 *      *

Find lower bound l where t(n-t) > r then n-2l+1 (for odd values at least) winning values exist

*/


fn calc_winning_moves(times: &Vec<usize>, dist: &Vec<usize>) -> usize {
    let mut wins:usize = 1;
    for (n,r) in times.iter().zip(dist.iter()){
        let mut t = 0;
        // println!("n: {} r: {} t: {} t(n-t): {}", n, r, t, t*(n-t));

        while t*(n-t) <= *r {
            t+=1;
        }
        // t is now lower bound
        wins *= n-2*t +1;
    }
    wins
}

pub fn aoc_2023_06_a(input: &str) -> usize {
    let [ref times, ref dist]  = input.trim().lines()
    .map(|l| l.split_whitespace()
        .flat_map(|s| s.parse::<usize>())
        .collect::<Vec<_>>()
    ).collect::<Vec<_>>()[..] else { panic!("not enough lines")};

    // println!("Times: {:?}\nDists: {:?}", times, dist);
    calc_winning_moves(times, dist)
}

pub fn aoc_2023_06_b(input: &str) -> usize {
    let [ref times, ref dist]  = input.trim().lines()
    .flat_map(|l| 
        l.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<usize>()
    ).map(|u| vec![u]) // make it a single element vector so type fits
    .collect::<Vec<_>>()[..] else { panic!("not enough lines")};

    calc_winning_moves(times, dist)
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_06_a_example() {
        assert_eq!(super::aoc_2023_06_a(TEST_INPUT), 288);
    }

    #[test]
    fn aoc_2023_06_a() {
       assert_eq!(super::aoc_2023_06_a(INPUT), 1195150);
    }
    
    #[test]
    fn aoc_2023_06_b_example() {
        assert_eq!(super::aoc_2023_06_b(TEST_INPUT), 71503);
    }

    #[test]
    fn aoc_2023_06_b() {
        assert_eq!(super::aoc_2023_06_b(INPUT), 0);
    }

    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
Time:      7  15   30
Distance:  9  40  200";
}



