// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2020/day/01
    Solution idea:

*/

pub fn aoc_2020_01_a(input: &str) -> i32 {
    let x: Vec<_> = input
        .trim()
        .lines()
        .flat_map(|s| s.trim().parse::<i32>())
        .collect();
    for i in 0..x.len() {
        for j in i + 1..x.len() {
            if x[i] + x[j] == 2020 {
                return x[i] * x[j];
            }
        }
    }
    0
}

pub fn aoc_2020_01_b(input: &str) -> i32 {
    let x: Vec<_> = input
        .trim()
        .lines()
        .flat_map(|s| s.trim().parse::<i32>())
        .collect();
    for i in 0..x.len() {
        for j in i + 1..x.len() {
            for k in j + 1..x.len() {
                if x[i] + x[j] + x[k] == 2020 {
                    return x[i] * x[j] * x[k];
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2020_01_a_example() {
        assert_eq!(super::aoc_2020_01_a(TEST_INPUT), 514579);
    }

    #[test]
    fn aoc_2020_01_a() {
        assert_eq!(super::aoc_2020_01_a(INPUT), 1019371);
    }

    #[test]
    fn aoc_2020_01_b_example() {
        assert_eq!(super::aoc_2020_01_b(TEST_INPUT), 241861950);
    }

    #[test]
    fn aoc_2020_01_b() {
        assert_eq!(super::aoc_2020_01_b(INPUT), 0);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    1721
    979
    366
    299
    675
    1456";
}
