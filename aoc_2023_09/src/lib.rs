use core::panic;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/09
    Solution idea:
    Only last values of difference is needed
*/
fn difference(org: &Vec<i64>) -> Vec<i64> {
    org.windows(2).map(|slice| slice[1] - slice[0]).collect()
}

fn predict(seq: &Vec<i64>) -> (i64, i64) {
    if seq.len() < 2 {
        panic!("Sequence must be at least 2 elements long");
    }

    // If all elements are the same, return that element
    // no need to go down to 0
    let first = seq[0];
    if seq.iter().all(|&i| i == first) {
        return (first, first);
    }

    let diff = difference(seq);
    let predicted = predict(&diff);

    (
        // part b: fist element of sequence - predict(diff).first
        seq[0] - predicted.0,
        // part a: last element of sequence + predict(diff).last
        seq.last().unwrap() + predicted.1,
    )
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .flat_map(|s| s.parse::<i64>())
                .collect()
        })
        .collect()
}

pub fn aoc_2023_09_a(input: &str) -> i64 {
    parse(input).iter().map(|seq| predict(seq).1).sum()
}

pub fn aoc_2023_09_b(input: &str) -> i64 {
    parse(input).iter().map(|seq| predict(seq).0).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_09_a_example() {
        assert_eq!(super::aoc_2023_09_a(TEST_INPUT), 114);
    }

    #[test]
    fn aoc_2023_09_a() {
        assert_eq!(super::aoc_2023_09_a(INPUT), 1806615041);
    }

    #[test]
    fn aoc_2023_09_b_example() {
        assert_eq!(super::aoc_2023_09_b(TEST_INPUT), 2);
    }

    #[test]
    fn aoc_2023_09_b() {
        assert_eq!(super::aoc_2023_09_b(INPUT), 1211);
    }

    #[test]
    fn difference_should() {
        assert_eq!(
            super::difference(&vec![0, 3, 6, 9, 12, 15]),
            vec![3, 3, 3, 3, 3]
        );
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45";
}
