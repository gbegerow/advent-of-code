use regex::Regex;

#[derive(Debug)]
struct Cmd {
    count: u32,
    from: usize,
    to: usize
}



pub fn aoc_2022_05_a(input: &str) -> String { 
    let (mut crates, cmds) = parse_input(input);

    //println!("Crates: {:?}", crates);
    // println!("Cmds: {:?}", cmds);

    for cmd in cmds {
        for _ in  0.. cmd.count{
            // println!("Move from {} {:?} to {} {:?} {}", 
            //     cmd.from, crates[cmd.from - 1], 
            //     cmd.to, crates[cmd.to-1],
            //     cmd.count );
            let v = crates[cmd.from - 1].pop().expect("Invalid move");
            crates[cmd.to-1].push(v);
        }
    }

    let res: String = crates.iter().map(|stack| stack[stack.len()-1]).collect();
    res
}


pub fn aoc_2022_05_b(input: &str) -> String {
    let (mut crates, cmds) = parse_input(input);

    for cmd in cmds {
        let mut v = Vec::new();

        // there must be a better way to pop the last n elements, could not get splice to work
        // but should be efficient enough for now
        for _ in  0.. cmd.count{
            v.push(crates[cmd.from -1].pop().expect("Invalid move"));            
        }
        v.reverse();
        crates[cmd.to-1].append(&mut v);

    }

    let res: String = crates.iter().map(|stack| stack[stack.len()-1]).collect();
    res
}


fn parse_input(input: &str ) -> (Vec<Vec<char>>, Vec<Cmd>){
    let mut cmds = Vec::new();
    let mut crates:Vec<Vec<char>> = Vec::new();

    let cmd_rx = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for line in input.lines() {
        if line.contains("[") {
            // crates
            // ensure stacks
            let num_crates = line.len()/4 + 1;
            while crates.len() < num_crates {
                crates.push(Vec::new());
            }
            for (col, c) in line.chars().enumerate(){
                if c.is_ascii_alphabetic() {
                    let stack =  col/4;
                    // println!("Stack {stack} ({col}) : '{c}'");
                    crates[stack].insert(0, c)
                }
            }

        } else if cmd_rx.is_match(line) {
            // cmds
            for cap in cmd_rx.captures_iter(line) {
                cmds.push(Cmd {
                    count: cap[1].parse().unwrap(), 
                    from:cap[2].parse().unwrap(), 
                    to: cap[3].parse().unwrap()
                } );
            }

        }
    }

    (crates, cmds)
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_05_a_example() {
        assert_eq!(super::aoc_2022_05_a(TEST_INPUT), "CMZ");
    }

    #[test]
    fn aoc_2022_05_a() {
       assert_eq!(super::aoc_2022_05_a(include_str!("input.txt")), "CVCWCRTVQ");
    }
    
    #[test]
    fn aoc_2022_05_b_example() {
        assert_eq!(super::aoc_2022_05_b(TEST_INPUT), "MCD");
    }

    #[test]
    fn aoc_2022_05_b() {
        assert_eq!(super::aoc_2022_05_b(include_str!("input.txt")), "CNSCZWLVT");
    }

    const TEST_INPUT: &str = "
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
}



