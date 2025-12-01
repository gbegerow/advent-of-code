// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/24
    Solution idea:
    Possible half adder?
    Part a: Exceute in topological order

    Part b: store each path in topological order
    Test each path wether or not it is faulty by setting each bit only in input
    Can we determine the pairs by looking, which output bit is set instead of which?
    foreach faulty pair swap each two wires and test if still faulty^

    There should be a fulladder build by combination of two halfadder for each x y pair.
    x XOR y => Sum
    x AND y => Carry_1
    Carry_in XOR SUM => z
    Carry_in AND SUM => Carry_2
    Carry_1 OR Carry_2 => Carry

    Test x,y with
        (0,0) => z = 0, Carry = 0
        (0,1) | (1,0) => z=1, Carry=0
        (1,1) => z=0, Carry=1

*/
use std::collections::{BTreeMap, VecDeque};
use std::fmt::{Display, Write};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, multispace0, multispace1, space1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
// use regex::Regex;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
struct Gate<'a> {
    name: &'a str,
    input1: &'a str,
    input2: &'a str,
    op: Op,

    // topological order
    order: u32,
    // rank/level in circuit
    rank: u32,
    // target_bit: &'a str,
}

impl<'a> Display for Gate<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} {:?} {} -> {})",
            self.input1, self.op, self.input2, self.name
        )
    }
}

impl<'a> Gate<'a> {
    #[allow(dead_code)]
    fn new(name: &'a str, input1: &'a str, input2: &'a str, op: Op) -> Self {
        Self {
            name,
            input1,
            input2,
            op,

            order: 0,
            rank: 0,
            // target_bit: ""
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct System<'a> {
    gates: BTreeMap<&'a str, Gate<'a>>,
    values: BTreeMap<&'a str, u64>,
}

impl System<'_> {
    fn value_of(&self, prefix: &str) -> u64 {
        self.values
            .iter()
            .filter(|(k, _v)| k.starts_with(prefix))
            // .inspect(|p| println!("{p:?}"))
            .fold(0, |accu, (k, v)| {
                if let Ok(bit) = k[1..].parse::<u64>() {
                    // set bit to value
                    accu | (v << bit)
                } else {
                    // prefix not followed by bit number, ignore
                    accu
                }
            })
    }

    #[allow(dead_code)]
    fn paths_of_bit_from_sources(&self, bit: u32) -> Option<Vec<&Gate>> {
        // forward search, leads to a lot of unused gates
        // todo: backward search from z
        let mut queue = VecDeque::with_capacity(self.gates.len());
        let mut gates = Vec::new();

        let x = format!("x{bit:02}");
        let y = format!("y{bit:02}");
        if !self.values.contains_key(x.as_str()) {
            return None;
        }

        // put the original name ref in queue or rust thinks we are returning a temporary ref
        queue.extend(
            self.gates
                .iter()
                .filter(|(_, g)| g.input1 == x || g.input2 == x || g.input1 == y || g.input2 == y)
                .map(|(name, _g)| name),
        );

        // inefficient but it is not a big number of gates
        // FIFO
        while let Some(name) = queue.pop_front() {
            let is_input_of = self
                .gates
                .values()
                .filter(|g| !gates.contains(g) && (g.input1 == name || g.input2 == name))
                .collect::<Vec<_>>();

            queue.extend(is_input_of.iter().map(|g| g.name));

            // follows the carry :-)
            let gate = &self.gates[name];
            if !gates.contains(&gate) {
                gates.push(gate);
            }
        }

        // sort topologicly
        gates.sort_by_key(|g| g.order);

        Some(gates)
    }

    #[allow(dead_code)]
    fn paths_of_bit_from_output(&self, bit: u32) -> Option<Vec<&Gate>> {
        let z_name = format!("z{bit:02}");
        if !self.values.contains_key(z_name.as_str()) {
            return None;
        }

        let mut queue = VecDeque::with_capacity(self.gates.len());
        let mut gates = Vec::with_capacity(20); // normal fulladder 5 gates, so this is plenty

        // put the original name ref in queue or rust thinks we are returning a temporary ref
        let output = &self.gates[z_name.as_str()];
        queue.push_back(output.name);

        // bfs upwards.
        while let Some(name) = queue.pop_front() {
            // This follows the incomming carry upwards and gets all input :-(
            // How to break the carry?
            // idea: label gate with the first output it was reached from
            //      if gate is already labeled, do not follow
            if let Some(gate) = self.gates.get(&name) {
                queue.push_back(gate.input1);
                queue.push_back(gate.input2);

                gates.push(gate);
            }; // else values are no gates
        }
        // sort topologicly
        gates.sort_by_key(|g| g.order);

        Some(gates)
    }

    fn execute(&mut self) {
        // execute_in_topological_order
        // kind of Kahn' algorithm https://en.wikipedia.org/wiki/Topological_sorting

        // nodes with a value are incoming without dependency
        let mut has_value = VecDeque::from_iter(self.values.keys().cloned());
        let mut order_no = 0;

        while let Some(n) = has_value.pop_front() {
            // does the value exists or do we need to calculate it
            if !self.values.contains_key(n) {
                let gate = &self.gates[n];
                // both inputs must exist now
                let in1 = self.values[gate.input1];
                let in2 = self.values[gate.input2];

                let value = match gate.op {
                    Op::And => in1 & in2,
                    Op::Or => in1 | in2,
                    Op::Xor => in1 ^ in2,
                };

                self.values.insert(gate.name, value);

                let rank1 = self
                    .gates
                    .get(gate.input1)
                    .map(|g| g.rank)
                    .unwrap_or_default();
                let rank2 = self
                    .gates
                    .get(gate.input2)
                    .map(|g| g.rank)
                    .unwrap_or_default();

                let Some(gate) = self.gates.get_mut(n) else {
                    unreachable!()
                };

                // remember topological order
                gate.order = order_no;
                gate.rank = rank1.max(rank2) + 1;

                order_no += 1;
            }

            // find all gates where the current is input. If both inputs are known, add gate to queue
            // (prune graph in Kahn's algrorithmus)
            for gate in self.gates.values() {
                // do we know both values now?
                if gate.input1 == n && self.values.contains_key(&gate.input2) {
                    has_value.push_back(gate.name);
                }
                if self.values.contains_key(gate.input1) && gate.input2 == n {
                    has_value.push_back(gate.name);
                }
            }
        }

        println!("Topological order: {:?}", self.gates);
    }
}

fn parse_values(input: &str) -> IResult<&str, BTreeMap<&str, u64>> {
    // y04: 1
    let (rest, list) =
        separated_list1(multispace1, separated_pair(alphanumeric1, tag(": "), u64))(input)?;

    Ok((rest, BTreeMap::from_iter(list)))
}

fn parse_gate(input: &str) -> IResult<&str, Gate> {
    // y03 OR y00 -> psh
    map(
        tuple((
            alphanumeric1,
            space1,
            alt((tag("AND"), tag("OR"), tag("XOR"))),
            space1,
            alphanumeric1,
            tag(" -> "),
            alphanumeric1,
        )),
        |x| {
            Gate::new(
                x.6,
                x.0,
                x.4,
                match x.2 {
                    "AND" => Op::And,
                    "OR" => Op::Or,
                    "XOR" => Op::Xor,
                    _ => unreachable!("Unknown op"),
                },
            )
        },
    )(input)
}

fn parse_gates(input: &str) -> IResult<&str, BTreeMap<&str, Gate>> {
    map(separated_list1(line_ending, parse_gate), |l| {
        BTreeMap::from_iter(l.into_iter().map(move |g| (g.name, g)))
    })(input)
}

fn parse(input: &str) -> IResult<&str, System> {
    preceded(
        // allow leading space
        multispace0,
        map(
            separated_pair(parse_values, multispace1, parse_gates),
            |(values, gates)| System { values, gates },
        ),
    )(input)
}

#[tracing::instrument]
pub fn aoc_2024_24_a(input: &str) -> u64 {
    let (_, mut system) = parse(input).expect("invalid input");
    system.execute();

    system.value_of("z")
}

#[tracing::instrument]
pub fn aoc_2024_24_b(input: &str) -> u64 {
    let (_, mut system) = parse(input).expect("invalid input");
    system.execute();

    let x = system.value_of("x");
    let y = system.value_of("y");
    let z = system.value_of("z");
    println!("x: {:064b} \t{}", x, x);
    println!("y: {:064b} \t{}", y, y);
    println!("z: {:064b} \t{}", z, z);
    println!("+: {:064b} \t{}", x + y, x + y);

    // easy test, are any wires crossed at input (cannot be, values are no gates)
    // let re = Regex::new(r"(x|y)(\d+)").unwrap();
    // println!(
    //     "{}",
    //     system
    //         .gates
    //         .values()
    //         //rank 1: x and y should be for the same bit. Nothing, but worth a try.vfgdsffffffffffffffff
    //         .filter(|g| (re.is_match(g.input1) || re.is_match(g.input2))
    //             && g.input1[1..] != g.input2[1..])
    //         .fold("Cross: ".to_string(), |mut s, g| {
    //             write!(s, "{}\n", g).unwrap();
    //             s
    //         })
    // );

    // print all paths an input bit flows to the output
    let mut bit = 0;
    while let Some(path) = system.paths_of_bit_from_output(bit) {
        println!("Bit {}: ({} gates)", bit, path.len());

        println!(
            "{}\n",
            path.iter().fold("".to_string(), |mut s, g| {
                write!(s, "\t{}\n", g).unwrap();
                s
            })
        );

        bit += 1;
    }

    z
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 2024)]
    fn aoc_2024_24_a_example(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(super::aoc_2024_24_a(input), expected);
    }

    #[test]
    fn aoc_2024_24_a() {
        assert_eq!(super::aoc_2024_24_a(super::INPUT), 43559017878162);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_24_b_example(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(super::aoc_2024_24_b(input), expected);
    }

    #[test]
    fn aoc_2024_24_b() {
        assert_eq!(super::aoc_2024_24_b(super::INPUT), 0);
    }

    #[test]
    fn parse_values_should() {
        let sut = parse_values(
            "x00: 1
            x01: 0
            x02: 1",
        );
        assert_eq!(
            sut,
            Ok(("", BTreeMap::from([("x00", 1), ("x01", 0), ("x02", 1),])))
        );
    }

    #[test]
    fn parse_gate_should() {
        let sut = parse_gate("ntg XOR fgs -> mjb");

        assert_eq!(sut, Ok(("", Gate::new("mjb", "ntg", "fgs", Op::Xor))))
    }

    #[test]
    fn parse_gates_should() {
        let sut = parse_gates(
            "ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05",
        );

        assert_eq!(
            sut,
            Ok((
                "",
                BTreeMap::from([
                    ("mjb", Gate::new("mjb", "ntg", "fgs", Op::Xor)),
                    ("tnw", Gate::new("tnw", "y02", "x01", Op::Or)),
                    ("z05", Gate::new("z05", "kwq", "kpj", Op::Or)),
                ])
            ))
        )
    }

    #[test]
    fn parse_should() {
        let sut = parse(
            "x00: 1
x01: 0
x02: 1
            
ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05",
        );

        assert_eq!(
            sut,
            Ok((
                "",
                System {
                    values: BTreeMap::from([("x00", 1), ("x01", 0), ("x02", 1),]),
                    gates: BTreeMap::from([
                        ("mjb", Gate::new("mjb", "ntg", "fgs", Op::Xor)),
                        ("tnw", Gate::new("tnw", "y02", "x01", Op::Or)),
                        ("z05", Gate::new("z05", "kwq", "kpj", Op::Or)),
                    ])
                }
            ))
        )
    }

    const TEST_INPUT: &str = "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
}
