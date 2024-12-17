// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/17
    Solution idea:

*/
// use nom::IResult;

use std::fmt::Display;

const ADV: i64 = 0;
const BXL: i64 = 1;
const BST: i64 = 2;
const JNZ: i64 = 3;
const BXC: i64 = 4;
const OUT: i64 = 5;
const BDV: i64 = 6;
const CDV: i64 = 7;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cpu {
    pc: usize,
    a: i64,
    b: i64,
    c: i64,
    halted: bool,
}

#[derive(Debug)]
struct Device {
    cpu: Cpu,
    instructions: Vec<i64>,
    output: Vec<i64>,
}

fn parse(input: &str) -> Device {
    let (mut a, mut b, mut c) = (0, 0, 0);
    let mut instructions = Vec::new();

    for l in input.lines() {
        let tokens = l.split_ascii_whitespace().collect::<Vec<_>>();
        if tokens.is_empty() {
            continue;
        }
        match tokens[0] {
            "Register" => {
                let operand: i64 = tokens[2].parse().expect("valid program");
                match tokens[1] {
                    "A:" => a = operand,
                    "B:" => b = operand,
                    "C:" => c = operand,
                    _ => unreachable!("unknown register {:?}", tokens),
                }
            }

            "Program:" => {
                // expect jumps to change opcodes to operands and vice versa
                // so delay interpretation to execution
                instructions = tokens[1]
                    .split(',')
                    .flat_map(|i| i.parse::<i64>())
                    .collect();
            }

            _ => (),
        }
    }

    Device {
        cpu: Cpu {
            pc: 0,
            a,
            b,
            c,
            halted: false,
        },
        instructions,
        output: Vec::new(),
    }
}

fn format_combo(i: &i64) -> String {
    match i {
        0..=3 => i.to_string(),
        4 => "A".to_string(),
        5 => "B".to_string(),
        6 => "C".to_string(),

        _ => "reserved".to_string(),
    }
}
fn format_instruction(inst: &[i64]) -> String {
    let [opcode, operand, ..] = inst else {
        panic!("invalid instructions {:?}", inst)
    };

    match *opcode {
        ADV => {
            format!("adv {}   0 {}", format_combo(operand), operand)
        }
        BXL => {
            format!("bxl {}   1 {}", operand, operand)
        }
        BST => {
            format!("bst {}   2 {}", format_combo(operand), operand)
        }
        JNZ => {
            format!("jnz {}   3 {}", operand, operand)
        }
        BXC => {
            format!("bxc     4 {}", operand)
        }
        OUT => {
            format!("out {}   5 {}", format_combo(operand), operand)
        }
        BDV => {
            format!("bdv {}   6 {}", format_combo(operand), operand)
        }
        CDV => {
            format!("cdv {}   7 {}", format_combo(operand), operand)
        }
        _ => unreachable!("invalid instruction {opcode} {operand}"),
    }
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "A {}  B {}  C {}", self.a, self.b, self.c)?;
        writeln!(f, "PC: {:04} Halted: {}", self.pc, self.halted)
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.cpu)?;
        for (index, instruction) in self.instructions[self.cpu.pc..].chunks_exact(2).enumerate() {
            write!(
                f,
                "{:04} {} ",
                index,
                if index == self.cpu.pc { "=>" } else { "  " }
            )?;

            writeln!(f, "{}", format_instruction(instruction))?;
        }
        writeln!(f)
    }
}

fn combo(cpu: &Cpu, operand: i64) -> i64 {
    match operand {
        0..=3 => operand,
        4 => cpu.a,
        5 => cpu.b,
        6 => cpu.c,

        _ => unreachable!("reserved"),
    }
}
fn div(cpu: &Cpu, operand: i64) -> i64 {
    let c = combo(cpu, operand);
    // can never be 0
    let denominator = 1 << c;

    cpu.a / denominator
}

fn execute(device: &Device) -> (Cpu, Option<i64>) {
    let mut cpu = device.cpu.clone();
    let mut output = None;

    if cpu.halted {
        return (cpu, output);
    }

    let [opcode, operand, ..] = device.instructions[cpu.pc..] else {
        panic!("invalid instructions")
    };

    match opcode {
        ADV => {
            cpu.a = div(&cpu, operand);
            cpu.pc += 2;
        }
        BXL => {
            cpu.b ^= operand;
            cpu.pc += 2;
        }
        BST => {
            cpu.b = combo(&cpu, operand) % 8;
            cpu.pc += 2;
        }
        JNZ => {
            if cpu.a != 0 {
                cpu.pc = operand as usize
            }
        }
        BXC => {
            cpu.b ^= cpu.c;
            cpu.pc += 2;
        }
        OUT => {
            output = Some(combo(&cpu, operand) % 8);
            cpu.pc += 2;
        }
        BDV => {
            cpu.b = div(&cpu, operand);
            cpu.pc += 2;
        }
        CDV => {
            cpu.c = div(&cpu, operand);
            cpu.pc += 2;
        }
        _ => unreachable!("invalid instruction {opcode} {operand}"),
    }

    if cpu.pc > device.instructions.len() {
        cpu.halted = true;
    }

    (cpu, output)
}

#[tracing::instrument]
pub fn aoc_2024_17_a(input: &str) -> String {
    // setup
    let mut device = parse(input);
    println!("{device}");

    // update
    let mut ticks = 0;
    while !device.cpu.halted && ticks < 1000 {
        // get the instruction for output before executing it
        // let inst = format_instruction(&device.instructions[device.cpu.pc..]);
        // println!("{:05} s PC {} {}", ticks, device.cpu.pc, inst);

        let (cpu, output) = execute(&device);

        // println!("{}, {:?}", cpu, output);

        device.cpu = cpu;
        if output.is_some() {
            device.output.push(output.unwrap());
        }

        ticks += 1;
    }

    // result
    device
        .output
        .iter()
        .map(|o| format!("{}", o))
        .collect::<Vec<_>>()
        .join(",")
}

#[tracing::instrument]
pub fn aoc_2024_17_b(input: &str) -> i64 {
    // find value for A so that output is copy of intructions
    // brute force is probably not a good idea
    // looks like a crypto algorithm with A is the key (enigma with a single or two rotors?)
    // how can we break the enigma?
    let org_device = parse(input);

    for a in 1..1_000_000 {
        let mut device = Device {
            cpu: Cpu {
                a,
                ..org_device.cpu
            },
            instructions: org_device.instructions.clone(),
            output: Vec::new(),
        };
        let tick = 0;
        while !device.cpu.halted && tick < 1_000_000 {
            // get the instruction for output before executing it
            // let inst = format_instruction(&device.instructions[device.cpu.pc..]);
            // println!("{:05} s PC {} {}", ticks, device.cpu.pc, inst);

            let (cpu, output) = execute(&device);

            // println!("{}, {:?}", cpu, output);

            device.cpu = cpu;
            if output.is_some() {
                let output = output.unwrap();
                // output must match instructions, so we can stop at first difference
                if device.instructions[device.output.len()] == output {
                    device.output.push(output);
                } else {
                    break;
                }
            }
        }
        if device.output == device.instructions {
            println!("found! {device}");
            return device.cpu.a;
        }
    }
    99
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, "4,6,3,5,6,3,5,2,1,0")]
    // #[case(TEST_INPUT_2, "4,6,3,5,6,3,5,2,1,0")]
    fn aoc_2024_17_a_example(#[case] input: &str, #[case] exepected: String) {
        assert_eq!(super::aoc_2024_17_a(input), exepected);
    }

    #[test]
    fn aoc_2024_17_a() {
        assert_eq!(super::aoc_2024_17_a(super::INPUT), "6,2,7,2,3,1,6,0,5");
    }

    #[rstest]
    #[case(TEST_INPUT, 117440)]
    fn aoc_2024_17_b_example(#[case] input: &str, #[case] exepected: i64) {
        assert_eq!(super::aoc_2024_17_b(input), exepected);
    }

    #[test]
    fn aoc_2024_17_b() {
        assert_eq!(super::aoc_2024_17_b(super::INPUT), 0);
    }

    #[test]
    fn slice_pattern() {
        let [a, b, ..] = &[0, 1, 2, 3, 4, 5];
        println!("{a}{b}")
    }

    const TEST_INPUT: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    // What do we get if we take our output of a and execute it as a program?
    const TEST_INPUT_2: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 6,2,7,2,3,1,6,0,5";
}
