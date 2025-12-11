// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/10
    Solution idea:
    enumerate variations of n elments via bfs
    from minimal number of elements upwards
    stop at first valid solution
    back to Informatik I :-)

    parser with winnow

# Input: list of elements (e.g. chars, numbers, ...)
# Goal: find minimal-length sequence of elements that satisfies a given condition (is_goal)
#   n            : number of elements (1..n)
#   is_goal(seq) : predicate that checks if "seq" satisfies the desired condition
#   max_len      : optional upper limit if no solution exists (otherwise infinite)

PROCEDURE BFS_MinLength_OneSolution(n, is_goal, max_len = +∞):
    REQUIRE n >= 1

    # Initial Frontier: all sequences of length 1
    LET Q := empty queue
    FOR v FROM 1 TO n:
        ENQUEUE(Q, [v])

    LET depth := 1
    WHILE NOT EMPTY(Q) AND depth <= max_len:
        # process current layer fully (true BFS at depth "depth")
        LET layer_size := SIZE(Q)
        FOR i FROM 1 TO layer_size:
            seq := DEQUEUE(Q)

            IF is_goal(seq) THEN
                RETURN seq  # first found solution has minimal length

            # otherwise extend for next depth
            FOR v FROM 1 TO n:
                ENQUEUE(Q, seq ⊕ [v])   # ⊕ = append
        depth := depth + 1

    RETURN NONE   # No solution found up to max_len

*/

use std::str::FromStr;
use winnow::prelude::*;
use winnow::{
    ascii::{digit1, multispace0, multispace1},
    combinator::{alt, delimited, separated, repeat, seq},
    Result,
};
use bitvec::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Representation of the machine state
/// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
struct Machine {
    
    light_pattern: BitArray,    // bitvec should work better than Vec<bool>, maybe bitarray?
    button_wiring: Vec<BitArray>, // button -> affected lights, probably bitvec for XOR
    requirement: Vec<u32>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        machine.parse(s).map_err(|e| e.to_string())
    }
}

fn machine(i: &mut &str) -> Result<Machine> {
    /// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    seq!(Machine {
        _ : multispace0,
        light_pattern: light_pattern,
        _ : multispace1,
        button_wiring: button_wiring,
        _ : multispace1,
        requirement: requirement,
    })
    .parse_next(i)
}

fn light_pattern(i: &mut &str) -> Result<BitArray> {
    // [.##.]
    delimited(
        "[",
        
        repeat(1.., alt(("#".map(|_| true), ".".map(|_| false)))),
        "]",
    ).map(|v: Vec<bool>| v.into_iter().collect())
    .parse_next(i)
}

fn button_wiring(i: &mut &str) -> Result<Vec<BitArray>> {
    // (3) (1,3) (2) (2,3) (0,2) (0,1)
    repeat::<_, _, Vec<Vec<usize>>, _, _>(
        1..,
        delimited(
            "(",
            separated::<_, _, Vec<usize>, _, _, _, _>(1.., 
                digit1.map(|s: &str| s.parse::<usize>().unwrap()), ","),
            ")",
        ).fold(BitArray::new(0), |acc, v: Vec<usize>| {for b in v { acc.set(b, true); } acc})
    )
    .parse_next(i)
}

fn requirement(i: &mut &str) -> Result<Vec<u32>> {
    // {3,5,4,7}
    delimited(
        "{",
        separated::<_, _, Vec<u32>, _, _, _, _>(1.., digit1.map(|s: &str| s.parse::<u32>().unwrap()), ","),
        "}",
    )
    .parse_next(i)
}

#[tracing::instrument]
pub fn aoc_2025_10_a(input: &str) -> Result<usize, String> {
    Ok(0)
}

#[tracing::instrument]
pub fn aoc_2025_10_b(_input: &str) -> Result<usize, String> {
    Ok(0)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use bitvec::prelude::*;
    use super::Machine;

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", Ok(2))]
    #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", Ok(3))]
    #[case(
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
        Ok(2)
    )]
    #[case(TEST_INPUT, Ok(7))]
    fn aoc_2025_10_a_example(#[case] input: &str, #[case] expected: Result<usize, String>) {
        assert_eq!(super::aoc_2025_10_a(input), expected);
    }

    #[test]
    fn aoc_2025_10_a() {
        assert_eq!(super::aoc_2025_10_a(super::INPUT), Ok(0));
    }

    #[rstest]
    #[case(TEST_INPUT, Ok(0))]
    fn aoc_2025_10_b_example(#[case] input: &str, #[case] expected: Result<usize, String>) {
        assert_eq!(super::aoc_2025_10_b(input), expected);
    }

    #[test]
    fn aoc_2025_10_b() {
        assert_eq!(super::aoc_2025_10_b(super::INPUT), Ok(0));
    }

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 
        Ok(Machine {
            light_pattern: bitvec![0,1,1,0],
            button_wiring: vec![
                vec![3],
                vec![1,3],
                vec![2],
                vec![2,3],
                vec![0,2],
                vec![0,1],
            ],
            requirement: vec![3,5,4,7],
        })
    )]
    fn parse_machine_example(#[case] input: &str, #[case] expected: Result<Machine, String>) {
        let result = input.parse::<Machine>().map_err(|e| e.to_string());
        assert_eq!(result, expected);
    }

    const TEST_INPUT: &str = "
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
