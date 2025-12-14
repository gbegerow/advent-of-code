// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/10
    Solution idea:
    enumerate variations of n elments via bfs
    from minimal number of elements upwards
    stop at first valid solution
    back to Informatik I :-)

    parser with winnow

    variations enumeration is far to slow for part b, np complete is kicking in here...

    analytical solution?
    system of linear equations s. https://docs.rs/ndarray-linalg/latest/ndarray_linalg/solve/
    has glam a similar functionality?
       
    button mask (1,3) -> [0,1,0,1,0]  increase counter 1 and 3 by 1
    press button 3 times -> increase counter 1 and 3 by 3*1 aka multiply button mask with number of presses

    a1 * b1 + a2 * b2 + ... + an * bn = r
    ai = number of presses of button i
    bi = button mask vector for button i
    r = requirement vector  


    B a = r but B is not square
    System is underdetermined, may have multiple solutions or no solution

    How to solve this? 
    least_squares seems a possibility. Can linalg handle integer only solutions?
*/

use std::str::FromStr;
// use winnow::prelude::*;
// use winnow::{
//     ascii::{digit1, multispace0, multispace1},
//     combinator::{alt, delimited, separated, repeat, seq},
//     Result,
// };
use bitvec::prelude::*;
use regex::Regex;


use ndarray::{array, Array1, Array2};
use ndarray_linalg::LeastSquaresSvdInto; // for least squares solving, consumes input matrix


#[derive(Clone, PartialEq, Eq)]
/// Representation of the machine state
/// [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
struct Machine {
    light_pattern: BitVec, // bitvec should work better than Vec<bool>, maybe BitArray?
    button_wiring: Vec<BitVec>, // button -> affected lights, probably bitvec for XOR
    requirement: Vec<usize>,
}

impl std::fmt::Debug for Machine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Machine {{ light_pattern: {:?}, button_wiring: {:?}, requirement: {:?} }}",
            self.light_pattern
                .iter()
                .by_vals()
                .map(|b| if b { '#' } else { '.' })
                .collect::<String>(),
            self.button_wiring
                .iter()
                .map(|bv| bv
                    .iter()
                    .by_vals()
                    .map(|b| if b { '#' } else { '.' })
                    .collect::<String>())
                .collect::<Vec<String>>(),
            self.requirement,
        )
    }
}

impl Machine {
    const MACHINE_RE: &str = r"(?x)
    ^\s*
    \[ (?P<lights>[\.\#]+) \]
    (?P<buttons>
        (\s+ \( (?P<indices> [0-9,]+ ) \) )+
        )
    \s+ \{ (?P<requirements>[0-9,]+ )\}
    \s*$";

    fn apply_button_seqence_to_lights(&self, button_sequence: &[usize]) -> BitVec {
        let mut state = bitvec![0; self.light_pattern.len()]; // initial state all off
        for &button_index in button_sequence {
            let button = &self.button_wiring[button_index];
            state ^= button; // toggle lights according to button wiring
        }
        state
    }

    fn is_init_sequence(&self, button_sequence: &[usize]) -> bool {
        let result_state = self.apply_button_seqence_to_lights(button_sequence);
        result_state == self.light_pattern
    }

    fn apply_button_seqence_to_counters(&self, button_sequence: &[usize]) -> Vec<usize> {
        let mut state = vec![0; self.requirement.len()]; // initial state all off
        for &button_index in button_sequence {
            let button = &self.button_wiring[button_index];
            for (i, bit) in button.iter().by_vals().enumerate() {
                if bit {
                    state[i] += 1;
                }
            }
        }
        state
    }

    fn solve_with_least_squares(&self) -> Result<Vec<usize>, String> {
        let m = self.requirement.len();
        let n = self.button_wiring.len();

       
        let B = build_B(m, &self.button_wiring);

        // Build requirement vector r
        let r = Array1::from_vec(
            self.requirement
                .iter()
                .map(|&x| x as f64)
                .collect::<Vec<f64>>(),
        );
        println!("B: {:?}\nr: {:?}", B, r);

        // Solve least squares problem B * a = r
        let a = B.least_squares(&r).map_err(|e| e.to_string())?.solution;
        println!("Least squares solution a: {:?}", a);

        // Round solution to nearest integers
        let a_int: Vec<usize> = a.iter().map(|&x| x.round() as usize).collect();
        println!("Rounded integer solution a_int: {:?}", a_int);

        Ok(a_int)
    }
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
        let re = Regex::new(Self::MACHINE_RE).map_err(|e| e.to_string())?;
        let caps = re
            .captures(s)
            .ok_or_else(|| format!("Failed to parse machine: {}", s))?;

        // parse light pattern
        let light_str = caps.name("lights").ok_or("Missing lights")?.as_str();
        let light_count = light_str.len();
        let light_pattern: BitVec = light_str
            .chars()
            .map(|c| match c {
                '#' => true,
                '.' => false,
                _ => false,
            })
            .collect();

        // parse button wiring
        let button_wiring: Vec<BitVec> = if let Some(buttons_str) = caps.name("buttons") {
            buttons_str
                .as_str()
                .split_whitespace()
                .map(|s| {
                    s.trim_matches(|c| c == '(' || c == ')')
                        .split(',')
                        .filter_map(|num_str| num_str.parse::<usize>().ok())
                        .inspect(|i| debug_assert!(*i < light_count, "Button index out of range"))
                        .fold(bitvec![0; light_count], |mut acc, x| {
                            acc.set(x, true);
                            acc
                        })
                })
                .collect::<Vec<BitVec>>()
        } else {
            return Err("Missing buttons".to_string());
        };

        // parse requirement
        let requirement_str = caps
            .name("requirements")
            .ok_or("Missing requirements")?
            .as_str();
        let requirement: Vec<usize> = requirement_str
            .split(',')
            .filter_map(|num_str| num_str.parse::<usize>().ok())
            .collect();

        Ok(Machine {
            light_pattern,
            button_wiring,
            requirement,
        })
    }
}

fn parse(input: &str) -> Result<Vec<Machine>, String> {
    input
        .trim()
        .lines()
        .map(|line| line.parse::<Machine>().map_err(|e| e.to_string()))
        .collect::<Result<Vec<Machine>, String>>()
}

// how does winnow work??? What does error \n          ^\n mean? Not worth it
// impl FromStr for Machine {
//     type Err = String;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         machine.parse(s).map_err(|e| e.to_string())
//     }
// }

// fn machines(i: &mut &str) -> Result<Vec<Machine>> {
//     repeat::<_, _, Vec<Machine>, _, _>(
//         1..,
//         delimited(
//             multispace0,
//             machine,
//             multispace1,
//         )
//     )
//     .parse_next(i)
// }

// fn machine(i: &mut &str) -> Result<Machine> {
//     // [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
//     seq!(Machine {
//         _ : multispace0,
//         light_pattern: light_pattern,
//         _ : multispace1,
//         button_wiring: button_wiring,
//         _ : multispace1,
//         requirement: requirement,
//     })
//     .parse_next(i)
// }

// fn light_pattern(i: &mut &str) -> Result<BitVec> {
//     // [.##.]
//     delimited(
//         "[",

//         repeat(1.., alt(("#".map(|_| true), ".".map(|_| false)))),
//         "]",
//     ).map(|v: Vec<bool>| v.into_iter().collect())
//     .parse_next(i)
// }

// fn button_wiring(i: &mut &str) -> Result<Vec<BitVec>> {
//     // (3) (1,3) (2) (2,3) (0,2) (0,1)
//     repeat::<_, _, Vec<BitVec>, _, _>(
//         1..,
//         delimited(
//             "(",
//             separated::<_, _, Vec<usize>, _, _, _, _>(1..,
//                 digit1.map(|s: &str| s.parse::<usize>().unwrap()), ","),
//             ")",
//         ).map(|v: Vec<usize>| {
//             let max_index = *v.iter().max().unwrap_or(&0);
//             let mut bv = bitvec![0; max_index + 1];
//             for index in v {
//                 bv.set(index, true);
//             }
//             bv
//         })
//     )
//     .parse_next(i)
// }

// fn requirement(i: &mut &str) -> Result<Vec<usize>> {
//     // {3,5,4,7}
//     delimited(
//         "{",
//         separated::<_, _, Vec<usize>, _, _, _, _>(1.., digit1.map(|s: &str| s.parse::<usize>().unwrap()), ","),
//         "}",
//     )
//     .parse_next(i)
// }

/**
 * Iterate over variations with repetition via BFS and take the first solution found
 * This would be a real good case for generators...
 *
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
**/
fn find_minimal_variation<F>(n: usize, is_goal: F, max_len: usize) -> Option<Vec<usize>>
where
    F: Fn(&[usize]) -> bool,
{
    use std::collections::VecDeque;

    assert!(n >= 1, "n must be at least 1");

    let mut queue: VecDeque<Vec<usize>> = VecDeque::new();

    // Initial Frontier: all sequences of length 1
    for v in 0..n {
        queue.push_back(vec![v]);
    }

    let mut depth = 1;
    while !queue.is_empty() && depth <= max_len {
        let layer_size = queue.len(); // remember current layer size, as queue will grow
        for _ in 0..layer_size {
            let seq = queue.pop_front().unwrap();

            if is_goal(&seq) {
                return Some(seq); // first found solution has minimal length
            }

            // otherwise extend for next depth by appending each possible element including duplicates
            for v in 0..n {
                let mut new_seq = seq.clone();
                new_seq.push(v);
                queue.push_back(new_seq);
            }
        }
        // queue should now contain all sequences of length depth + 1 as eveything in current layer is processed
        depth += 1;
    }

    None // No solution found up to max_len
}

// Solving use the hard weapon: MATH
// Solve system of linear equations via ndarray-linalg

/// Build a 0/1 column vector of length `m` with ones at the given indices.
fn button_to_vector(m: usize, ones: &[usize]) -> Array1<f64> {
    let mut v = Array1::<f64>::zeros(m);
    for &idx in ones {
        v[idx] = 1.0;
    }
    v
}


/// Build matrix B by concatenating button vectors as columns
fn build_B(m: usize, button_masks: &[Vec<usize>]) -> Array2<f64> {
    let n = button_masks.len();
    let mut B = Array2::<f64>::zeros((m, n));
    for (j, ones) in button_masks.iter().enumerate() {
        let col = button_to_vector(m, ones);
        B.column_mut(j).assign(&col);
    }
    B
}



#[tracing::instrument]
pub fn aoc_2025_10_a(input: &str) -> Result<usize, String> {
    let machines = parse(input)?;

    let mut button_presses = 0;

    for machine in machines {
        // let press1 = machine.light_pattern.clone() ^ &machine.button_wiring[0];
        // println!("{:?} pressed button 1 {:?}", machine, press1);
        let is_goal = |seq: &[usize]| machine.is_init_sequence(seq);
        let max_len = 10; // arbitrary limit to avoid infinite loops in case of no solution. Is there a better way?
        let result = find_minimal_variation(machine.button_wiring.len(), is_goal, max_len);
        if let Some(button_sequence) = result {
                // println!(
                //     "Found init sequence for machine {:?}: {:?} (length {})",
                //     machine,
                //     button_sequence,
                //     button_sequence.len()
                // );
                button_presses += button_sequence.len();
        } else {
            println!("No init sequence found for machine {:?}", machine);
        }

    }

    Ok(button_presses)
}

#[tracing::instrument]
pub fn aoc_2025_10_b(input: &str) -> Result<usize, String> {
      let machines = parse(input)?;

    let mut button_presses = 0;

    for machine in machines {
        let is_goal = |seq: &[usize]| &machine.requirement ==seq;
        let max_len = machine.requirement.iter().max().ok_or("Empty requirement")? + 1; // arbitrary limit to avoid infinite loops in case of no solution. Is there a better way?
        
        // iterate over variations of button presses if far too slow
        // np complete is kicking in here...
        let result = find_minimal_variation(machine.button_wiring.len(), is_goal, max_len);
        if let Some(button_sequence) = result {
                // println!(
                //     "Found init sequence for machine {:?}: {:?} (length {})",
                //     machine,
                //     button_sequence,
                //     button_sequence.len()
                // );
                button_presses += button_sequence.len();
        } else {
            println!("No init sequence found for machine {:?}", machine);
        }

    }

    Ok(button_presses)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::find_minimal_variation;
    use super::Machine;
    use bitvec::prelude::*;
    use rstest::rstest;
    use ndarray::prelude::*;

    #[rstest]
    #[case(TEST_INPUT_LINE_1, Ok(2))]
    #[case(TEST_INPUT_LINE_2, Ok(3))]
    #[case(TEST_INPUT_LINE_3, Ok(2))]
    #[case(TEST_INPUT, Ok(7))]
    fn aoc_2025_10_a_example(#[case] input: &str, #[case] expected: Result<usize, String>) {
        assert_eq!(super::aoc_2025_10_a(input), expected);
    }

    #[test]
    fn aoc_2025_10_a() {
        assert_eq!(super::aoc_2025_10_a(super::INPUT), Ok(481));
    }

    #[rstest]
    #[case(TEST_INPUT_LINE_1, Ok(10))]
    #[case(TEST_INPUT_LINE_2, Ok(12))]
    #[case(TEST_INPUT_LINE_3, Ok(11))]
    #[case(TEST_INPUT, Ok(33))]
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
                bitvec![0,0,0,1], // vec![3],
                bitvec![0,1,0,1], // vec![1,3],
                bitvec![0,0,1,0], // vec![2],
                bitvec![0,0,1,1], // vec![2,3],
                bitvec![1,0,1,0], // vec![0,2],
                bitvec![1,1,0,0], // vec![0,1],
            ],
            requirement: vec![3,5,4,7],
        })
    )]
    fn parse_machine_example(#[case] input: &str, #[case] expected: Result<Machine, String>) {
        let result = input.trim().parse::<Machine>();
        assert_eq!(result, expected);
    }

    #[rstest]
    #[case( TEST_INPUT_LINE_1, vec![4, 5])]
    fn apply_button_seqence_to_lights(#[case] input: &str, #[case] button_sequence: Vec<usize>) {
        let machine: Machine = input.trim().parse().unwrap();
        let result_state = machine.apply_button_seqence_to_lights(&button_sequence);
        let expected_state = machine.light_pattern.clone();

        assert_eq!(result_state, expected_state);
    }

    #[rstest]
    #[case( TEST_INPUT_LINE_1, vec![4, 5])]
    #[case( TEST_INPUT_LINE_1, vec![1, 3])]
    fn is_init_sequence(#[case] input: &str, #[case] button_sequence: Vec<usize>) {
        let machine: Machine = input.trim().parse().unwrap();

        assert!(machine.is_init_sequence(&button_sequence));
    }

    #[rstest]
    #[case( TEST_INPUT_LINE_1, Some(vec![1, 3]), 3 )]
    #[case( TEST_INPUT_LINE_2, Some(vec![2,3,4]), 3 )]
    #[case( TEST_INPUT_LINE_3, Some(vec![1,2]), 3 )]
    #[case( TEST_INPUT_LINE_1, None, 1 )]
    fn find_minimal_init_sequence(#[case] input: &str, #[case] button_sequence: Option<Vec<usize>>, #[case] max_len: usize) {
        let machine: Machine = input.trim().parse().unwrap();
        let is_goal = |seq: &[usize]| machine.is_init_sequence(seq);

        let result = find_minimal_variation(machine.button_wiring.len(), is_goal, max_len);

        assert_eq!(result, button_sequence);
    }

    #[rstest]
    #[case( TEST_INPUT_LINE_1, vec![0, 1,1,1, 3,3,3, 4, 5,5])]
    #[case( TEST_INPUT_LINE_2, vec![0,0, 1,1,1,1,1, 3,3,3,3,3])]
    fn apply_button_seqence_to_counters(#[case] input: &str, #[case] button_sequence: Vec<usize>) {
        let machine: Machine = input.trim().parse().unwrap();
        let result_state = machine.apply_button_seqence_to_counters(&button_sequence);
        let expected_state = machine.requirement.clone();

        assert_eq!(result_state, expected_state);
    }

    #[rstest]
    #[case( TEST_INPUT_LINE_1, vec![3], Array1::from_vec(vec![0.0,0.0,0.0,1.0]) )]
    fn button_to_vector_example(#[case] input: &str, #[case] button: Vec<usize>, #[case] expected: Array1<f64>) {
        let machine: Machine = input.trim().parse().unwrap();
        let m = machine.requirement.len();
        let result = super::button_to_vector(m, &button);
        assert_eq!(result, expected);
    }

    const TEST_INPUT: &str = "
    [.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
    [...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
    [.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
    ";

    const TEST_INPUT_LINE_1: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";
    const TEST_INPUT_LINE_2: &str = "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}";
    const TEST_INPUT_LINE_3: &str =
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}

