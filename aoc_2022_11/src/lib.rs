
use std::collections::VecDeque;

// use itertools::Itertools;
use regex::Regex;
#[allow(dead_code)]
#[derive(Debug)]
struct Monkey {
    index: usize,
    // items: Vec<u64>, // only mutable part of struct, maybe separate?
    op_mul: u64,     // new = old^power * op_mul + op_add
    op_add: u64,
    op_pow: u64,
    test: u64,         // item divisible by
    target_ok: usize,   // index of Monkey to throw item if test true
    target_fail: usize, // index of Monkey to throw item if test false
}

fn parse_monkeys(input: &str) -> (Vec<Monkey>, Vec<VecDeque<u64>>) {
    let monkey_rx = Regex::new(
        r"(?mx)
        \s* Monkey \s+ (?P<index>\d+):
        \s*    Starting\ items:\ (?P<items>.+)
        \s*    Operation: \s+ new\ =\ (?P<op>.+)
        \s*    Test:\ divisible\ by\ (?P<test>\d+)
        \s*       If\ true:\ throw\ to\ monkey\  (?P<target_ok>\d+)
        \s*       If\ false:\ throw\ to\ monkey\ (?P<target_fail>\d+)
    ",
    )
    .unwrap();

    let mut monkeys = Vec::new();
    let mut items = Vec::new();

    for caps in monkey_rx.captures_iter(input) {
        // println!("{:?}", caps);

        let index = caps["index"].parse().expect("index should be uint");
        let monkey_items: VecDeque<u64> = caps["items"]
            .split(", ")
            .filter_map(|s| s.parse().ok())
            .collect();
        let op = match &caps["op"].split_whitespace().collect::<Vec<&str>>()[..] {
            ["old", "*", "old"] => (1, 0, 2),
            ["old", "*", mul] => (mul.parse().unwrap(), 0, 1),
            ["old", "+", add] => (1, add.parse().unwrap(), 1),
            _ => panic!("unknown expression")
        };
        let test: u64 = caps["test"]
            .parse()
            .expect("test should be int");
        let target_ok: usize = caps["target_ok"]
            .parse()
            .expect("target_ok should be uint");
        let target_fail: usize = caps["target_fail"]
            .parse()
            .expect("target_fail should be uint");

        let monkey = Monkey {
            index,
            op_mul: op.0,
            op_add: op.1,
            op_pow: op.2,
            test,
            target_ok,
            target_fail,
        };
        monkeys.push(monkey);
        
        items.push(monkey_items);
    }

    // for rule in &monkeys {
    //     println!("Rules: {:?}", rule);
    // }
    (monkeys, items)
}

// one round
fn do_monkey_business( monkeys: Vec<Monkey>, mut all_items: Vec<VecDeque<u64>>, rounds: u64, relief: u64) -> Vec<u64> {
    let mut counter = vec![0; all_items.len()];
    let modulo:u64 = monkeys.iter().map(|m| m.test).product();

    for _ in 0..rounds{
        for monkey in &monkeys {
            // why is working with mutable vectors in structures such a pain in rust? I own this, let me mutate it.
            // operate over all items from previous round and all we got in this and consume them
            while let Some(item) = all_items[monkey.index].pop_front() {

                // if relief is no longer keeping this in u32 range it goes out very fast
                // idea: do we realy care about the concrete value or can we use some modulo arithmetimic?
                // but modulo what? if all tests are prime, can we work modulo the product of all? 
                // or just the modulo we take anyway? 

                // new = old^power * op_mul + op_add
                let mut new_level =
                    (0..monkey.op_pow).fold(1, |a, _| item * a) 
                    * monkey.op_mul 
                    + monkey.op_add; 
                
                if relief > 1 {
                    new_level = new_level / relief;
                } else {
                    new_level = new_level % modulo; // % monkey.test; // thx unit tests, it's the product
                }
                
                // println!("Monkey {}: {}^{}*{}+{} => {}",
                //     monkey.index, 
                //     item,
                //     monkey.op_power, monkey.op_mul, monkey.op_add,
                //     new_level
                // );

                let target = if 0 == new_level % monkey.test {
                    monkey.target_ok
                } else {
                    monkey.target_fail
                };

                // throw it to the target
                all_items[target].push_back(new_level);

                // this monkey has done one more inspection    
                counter[monkey.index] += 1;
            }
        }
    }
    counter 
}

pub fn aoc_2022_11_a(input: &str) -> u64 {
    let (rules, items) = parse_monkeys(input);
    let mut counts = do_monkey_business(rules, items, 20, 3);
    // println!("Counts: {:?}", counts);
    counts.sort_unstable();

    let last = counts.len() -1;
    counts[last] * counts[last-1]
}

pub fn aoc_2022_11_b(input: &str) -> u64 {
    let (rules, items) = parse_monkeys(input);
    let mut counts = do_monkey_business(rules, items, 10000, 1);
    // println!("Counts: {:?}", counts);
    counts.sort_unstable();

    let last = counts.len() -1;
    counts[last] * counts[last-1]
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_11_a_example() {
        assert_eq!(super::aoc_2022_11_a(TEST_INPUT), 10605);
    }

    #[test]
    fn aoc_2022_11_a() {
        assert_eq!(super::aoc_2022_11_a(include_str!("input.txt")), 62491);
    }

    #[test]
    fn aoc_2022_11_b_example() {
        assert_eq!(super::aoc_2022_11_b(TEST_INPUT), 2713310158);
    }

    #[test]
    fn aoc_2022_11_b() {
        assert_eq!(super::aoc_2022_11_b(include_str!("input.txt")), 17408399184);
    }

    const TEST_INPUT: &str = "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
";
}
