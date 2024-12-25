use std::collections::{BTreeMap, VecDeque};

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/24
    Solution idea:
    Possible half adder?
    Part a: Exceute in topological order

    Part b: store each path in topological order
    Test each path wether or not it is faulty by setting each bit only in input
    Can we determine the pairs by looking, which output bit is set instead of which?
    foreach faulty pair swap each two wires and test if still faulty
*/
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, multispace0, multispace1, space1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

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
}

impl<'a> Gate<'a> {
    #[allow(dead_code)]
    fn new(name: &'a str, input1: &'a str, input2: &'a str, op: Op) -> Self {
        Self {
            name,
            input1,
            input2,
            op,
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
            .inspect(|p| println!("{p:?}"))
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

    fn execute(&mut self) {
        // execute_in_topological_order
        // kind of Kahn' algorithm https://en.wikipedia.org/wiki/Topological_sorting

        // nodes with a value are incoming without dependency
        let mut has_value = VecDeque::from_iter(self.values.keys().cloned());

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
        |x| Gate {
            name: x.6,
            input1: x.0,
            input2: x.4,
            op: match x.2 {
                "AND" => Op::And,
                "OR" => Op::Or,
                "XOR" => Op::Xor,
                _ => unreachable!("Unknown op"),
            },
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
