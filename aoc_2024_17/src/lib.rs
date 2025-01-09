// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/17
    Solution idea:

*/
// use nom::IResult;

use std::fmt::Display;

const ADV: u64 = 0;
const BXL: u64 = 1;
const BST: u64 = 2;
const JNZ: u64 = 3;
const BXC: u64 = 4;
const OUT: u64 = 5;
const BDV: u64 = 6;
const CDV: u64 = 7;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Cpu {
    pc: usize,
    a: u64,
    b: u64,
    c: u64,
    halted: bool,
}

#[derive(Debug)]
struct Device {
    cpu: Cpu,
    instructions: Vec<u64>,
    output: Vec<u64>,
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
                let operand: u64 = tokens[2].parse().expect("valid program");
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
                    .flat_map(|i| i.parse::<u64>())
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

fn format_combo(i: &u64) -> String {
    match i {
        0..=3 => i.to_string(),
        4 => "A".to_string(),
        5 => "B".to_string(),
        6 => "C".to_string(),

        _ => "reserved".to_string(),
    }
}
fn format_instruction(inst: &[u64]) -> String {
    let [opcode, operand, ..] = inst else {
        return "invalid".to_string();
        //panic!("invalid instructions {:?}", inst)
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
        writeln!(f, "A {:#o}  B {:#o}  C {:#o}", self.a, self.b, self.c)?;
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

fn combo(cpu: &Cpu, operand: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => cpu.a,
        5 => cpu.b,
        6 => cpu.c,

        _ => unreachable!("reserved"),
    }
}
fn div(cpu: &Cpu, operand: u64) -> u64 {
    let c = combo(cpu, operand);
    // can never be 0
    let denominator = 1 << c;

    cpu.a / denominator
}

fn execute(device: &Device) -> (Cpu, Option<u64>) {
    let mut cpu = device.cpu.clone();
    let mut output = None;

    if cpu.halted {
        return (cpu, output);
    }

    let [opcode, operand, ..] = device.instructions[cpu.pc..] else {
        panic!("invalid instructions in execute")
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
            } else {
                cpu.pc += 2;
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

    if cpu.pc >= device.instructions.len() {
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
        let inst = format_instruction(&device.instructions[device.cpu.pc..]);
        println!("{:05} s PC {} {}", ticks, device.cpu.pc, inst);

        let (cpu, output) = execute(&device);

        println!("{}, {:?}", cpu, output);

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
pub fn aoc_2024_17_b(input: &str) -> u64 {
    // see follow_the_road test for solution, don't care to transfer it

    // find value for A so that output is copy of intructions
    // brute force is probably not a good idea
    // looks like a crypto algorithm with A is the key (enigma with a single or two rotors?)
    // how can we break the enigma?
    let org_device = parse(input);

    for a in 1..u64::MAX {
        let mut device = Device {
            cpu: Cpu {
                pc: 0,
                a,
                b: 0,
                c: 0,
                halted: false,
            },
            instructions: org_device.instructions.clone(),
            output: Vec::new(),
        };
        let mut ticks = 0;
        while !device.cpu.halted && ticks < 100 {
            // get the instruction for output before executing it
            // let inst = format_instruction(&device.instructions[device.cpu.pc..]);
            // println!(
            //     "{:05} s PC {} {} {}",
            //     ticks, device.cpu.pc, device.cpu.halted, inst
            // );

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
            ticks += 1;
        }
        if 0 == a % 10000 {
            print!(". ");
        }
        if 0 == a % 250_000 {
            println!();
        }

        if device.output == device.instructions {
            println!("found! {device}");
            return a;
        }
    }
    99
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use std::u64;

    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, "4,6,3,5,6,3,5,2,1,0")]
    // #[case(TEST_INPUT_2, "4,6,3,5,6,3,5,2,1,0")]
    fn aoc_2024_17_a_example(#[case] input: &str, #[case] expected: String) {
        assert_eq!(super::aoc_2024_17_a(input), expected);
    }

    #[test]
    fn aoc_2024_17_a() {
        assert_eq!(super::aoc_2024_17_a(super::INPUT), "6,2,7,2,3,1,6,0,5");
    }

    #[rstest]
    #[case(TEST_INPUT, 117440)]
    fn aoc_2024_17_b_example(#[case] input: &str, #[case] expected: u64) {
        assert_eq!(super::aoc_2024_17_b(input), expected);
    }

    #[test]
    fn aoc_2024_17_b() {
        assert_eq!(super::aoc_2024_17_b(super::INPUT), 0);
    }

    #[rstest]
    #[case(crate::INPUT, "6,2,7,2,3,1,6,0,5?")]
    fn algo_in_rust(#[case] input: &str, #[case] expected: String) {
        let device = super::parse(input);
        println!("{device}");

        // A 0o263240543  B 0o0  C 0o0
        // PC: 0000 Halted: false

        // reimplementation of input code in rust to make it more readable
        let mut a = device.cpu.a;
        let mut b: u64;
        let mut c: u64;
        let mut output = Vec::new();

        // jnz 0
        while a > 0 {
            // bst A
            b = a & 0b111;
            //  bxl 3
            b ^= 0b011;
            //  cdv B
            c = a / (1 << b);
            //  bxl 5
            b ^= 0b101;
            //  adv 3
            a = a / (1 << 3);
            //  bxc
            b ^= c;
            //  out B
            output.push(b & 0b111);

            // println!("A {:#o}\tB {:#o}\tC {:#o}\tOut: {:?}", a, b, c, output);
            println!(
                "A {:#010b}\tB {:#010b}\tC {:#010b}",
                a & 0x3FF,
                b & 0x3FF,
                c & 0x3FF
            );
        }
        // todo: write an inverse

        let result = output
            .iter()
            .map(|o| format!("{}", o))
            .collect::<Vec<_>>()
            .join(",");

        assert_eq!(result, expected);
    }

    #[rstest]
    #[case(crate::INPUT, 0)]
    fn brute_force_in_rust(#[case] input: &str, #[case] expected: u64) {
        let device = super::parse(input);
        println!("{device}");

        // solution probably arount 2^(16 digits *3 bits) = 48 bit
        let mut register_a = 1 << 46; //device.cpu.a;
                                      // A 0o263240543  B 0o0  C 0o0
                                      // PC: 0000 Halted: false

        while register_a < (1 << 49) {
            // reimplementation of input code in rust to make it more readable
            let mut a = register_a;
            let mut b: u64;
            let mut c: u64;
            let mut output = Vec::new();

            // jnz 0
            while a > 0 {
                // bst A
                b = a & 0b111;
                //  bxl 3
                b ^= 0b011;
                //  cdv B
                c = a / (1 << b);
                //  bxl 5
                b ^= 0b101;
                //  adv 3
                a = a / (1 << 3);
                //  bxc
                b ^= c;
                //  out B
                output.push(b & 0b111);

                // Do we deviate from expected output? break early
                if &output[..] != &device.instructions[..output.len()] {
                    break;
                }
            }
            // todo: write an inverse
            if &output[..] == &device.instructions[..] {
                println!("Quine found for {register_a}");
                break;
            }

            register_a += 1;
            if 0 == register_a % 100_000_000 {
                println!("{register_a}")
            };
        }

        // 4_294_967_295 too low = u32::MAX
        assert_eq!(register_a, expected);
    }

    #[rstest]
    #[case(crate::INPUT, 236548287712877)]
    fn follow_the_road(#[case] input: &str, #[case] expected: u64) {
        let device = super::parse(input);
        println!("{device}");

        let mut seeds = Vec::with_capacity(1000);
        seeds.push(0);
        let mut min_solution = u64::MAX;
        let program_len = device.instructions.len();

        while let Some(register_a) = seeds.pop() {
            // solution probably around 2^(16 digits *3 bits) = 48 bit
            // 1 octal digit(3 bit) in Register A correspond to 1 output
            // test all 8 possible values for digit
            // if current yields a postfix of instructions, add to seeds
            // if current yields complete instructions, take minimum as solution
            // estimate 16 to 16*5=80 seeds / full calculations instead of 2^49
            for digit in 0..8 {
                // reimplementation of input code in rust to make it more readable
                let current = digit | register_a << 3;
                let mut a = current;
                let mut b: u64;
                let mut c: u64;
                let mut output = Vec::with_capacity(program_len);

                // jnz 0
                while a > 0 {
                    // bst A
                    b = a & 0b111;
                    //  bxl 3
                    b ^= 0b011;
                    //  cdv B
                    c = a / (1 << b);
                    //  bxl 5
                    b ^= 0b101;
                    //  adv 3
                    a = a / (1 << 3);
                    //  bxc
                    b ^= c;
                    //  out B
                    output.push(b & 0b111);

                    // early break does not work??? Don't care
                }

                // ignore empty from early break
                if output.len() == 0 {
                    continue;
                }
                // complete solution?
                else if &output[..] == &device.instructions[..] {
                    min_solution = min_solution.min(current);
                    println!("Quine found for {current} solution: {min_solution}");
                }
                // Do we have a postfix candidate for next round
                else if device.instructions.ends_with(&output[..]) {
                    // there can never be a duplicate seed
                    seeds.push(current);
                    println!(
                        "Seed {:#o} output {:?} inst {:?}",
                        current,
                        output,
                        &device.instructions[program_len - output.len()..]
                    );
                }
                // else not a candidate
            }
        }

        assert_ne!(min_solution, u64::MAX, "No solution");
        assert_eq!(min_solution, expected);
    }

    const TEST_INPUT: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    // What do we get if we take our output of a and execute it as a program?
    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "
Register A: 729
Register B: 0
Register C: 0

Program: 6,2,7,2,3,1,6,0,5";
}
