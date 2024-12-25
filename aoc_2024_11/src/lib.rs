// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/11
    Solution idea:
    idea: all number expand independently
    expand till it is some length say 32
    then take every number and expand on its own till 32 or max blinks are reached
    sum lengths of all parts

    can we somehow predict how much a number will grow in n steps?
    No, but remember, what we already calculated

    recursive definition => memoize
*/

use std::collections::HashMap;

fn blink_recursive(
    depth: u64,
    max_depth: u64,
    numbers: &mut Vec<u64>,
    known: &mut HashMap<(u64, u64), usize>,
) -> usize {
    // do not keep everything in memory at once, but there are still a lot of calls...
    // memoize results for known sequences esp. 0, 1

    if depth == max_depth {
        return numbers.len();
    }

    blink_naive(numbers);

    let next = depth + 1;
    let sum = numbers
        .iter()
        .map(|n| {
            let key = &(next, *n); // can't remember why we need the depth in the cache key, but we need it
                                   // we need both because it is different of we want the len of 5 iterations or 75, stupid!
            if known.contains_key(key) {
                known[key]
            } else {
                // cannot use entry here or we will get a double mut borrow
                let len = blink_recursive(next, max_depth, &mut vec![*n], known);
                known.insert(*key, len);
                len
            }
        })
        .sum();
    sum
}

fn blink_naive(numbers: &mut Vec<u64>) {
    // let's guess, will this blow in memory, u64 range or time or all together in b?

    // go backwards as we modify the vector on the right
    for i in (0..numbers.len()).rev() {
        if numbers[i] == 0 {
            // rule 1, does not change length, but goes 0->1->2024->20 24->2 0 2 4->4048 1 4048 8096->brr
            numbers[i] = 1;
        } else {
            let s = numbers[i].to_string();
            // could have used numbers[i].checked_ilog10 % 2 == 0 and s.split_at
            if 0 == s.len() % 2 {
                // rule 2, length +1
                let n1 = s[..s.len() / 2].parse::<u64>().expect("How?");
                let n2 = s[s.len() / 2..].parse::<u64>().expect("How?");
                numbers[i] = n1;
                numbers.insert(i + 1, n2);
            } else {
                // rule 3, does not change length
                numbers[i] *= 2024;
            }
        }
    }
}

fn parse(input: &str) -> Vec<u64> {
    let mut numbers = Vec::with_capacity(100_000);
    numbers.extend(
        input
            .trim()
            .split_ascii_whitespace()
            .flat_map(|s| s.parse::<u64>())
            .collect::<Vec<_>>(),
    );
    numbers
}

#[tracing::instrument]
pub fn aoc_2024_11_a(input: &str) -> usize {
    // linked list in rust are pita, so...
    let mut numbers = parse(input);

    for _i in 0..25 {
        blink_naive(&mut numbers);
        if numbers.len() < 10 {
            println!("{:?}", numbers);
        }
        println!("{}", numbers.len());
    }

    numbers.len()
}

#[tracing::instrument]
pub fn aoc_2024_11_b(input: &str) -> usize {
    let mut numbers = parse(input);
    let mut known = HashMap::with_capacity(10000);

    blink_recursive(0, 75, &mut numbers, &mut known)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 55312)]
    fn aoc_2024_11_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_11_a(input), expected);
    }

    #[test]
    fn aoc_2024_11_a() {
        assert_eq!(super::aoc_2024_11_a(super::INPUT), 224529);
    }

    #[rstest]
    #[case(TEST_INPUT, 25, 55312)]
    #[case(INPUT, 25, 224529)]
    fn blink_recursive_should(#[case] input: &str, #[case] blinks: u64, #[case] expected: usize) {
        let mut numbers = parse(input);
        let mut known = HashMap::with_capacity(10000);

        assert_eq!(
            blink_recursive(0, blinks, &mut numbers, &mut known),
            expected
        );
    }

    #[test]
    fn aoc_2024_11_b() {
        assert_eq!(super::aoc_2024_11_b(super::INPUT), 266820198587914);
    }

    // const TEST_INPUT2: &str = "0 1 10 99 999";
    const TEST_INPUT: &str = "125 17";
}
