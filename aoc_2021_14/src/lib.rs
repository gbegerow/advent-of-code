use std::collections::HashMap;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2021/day/14
    Solution idea:
    Just do it
    Plan B: Find a pattern without building the polymer code itself
    Rule XY -> Z means there will be a pair XZ and a ZY in the next round
    if XY is n times in current round there will be n times XZ and n times ZY in next round but n less XY(?)
    this rules adds n to the total count of the insert, initialized with the count from template
    is it enough to count the result pairs which are source of a rule? Assume all pairs have rule
    Test if all cartesian pairs of the initial alphabet have rules
    On finish split each pair and add count to bucket
    Memory is constant, multiplication instead of string allocation / operation
*/

struct Rule {

    ins: char,
    first: String,
    second: String,
}

impl Rule {
    fn new(key: &str, insert: &str) -> Self {
        let mut k = key.chars();
        let a = k.next().unwrap();
        let b = k.next().unwrap();
        let ins = insert.chars().next().unwrap();
        let first = String::from_iter([a, ins]);
        let second = String::from_iter([ins, b]);

        Self {
            ins,
            first,
            second,
        }
    }
}

fn aoc_2014_14_less_naive(input: &str, rounds: i32) -> i64 {
    let mut lines = input.trim().lines();

    let template = lines.next().unwrap();
    lines.next();

    // parse rules
    let rules = lines
        .filter_map(|l| l.trim().split_once(" -> "))
        .map(|(k, ins)| (k, Rule::new(k, ins)))
        .collect::<HashMap<_, _>>();

    // init char counter
    let mut counter = HashMap::with_capacity(26);
    for c in template.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }

    // init pair counter
    let mut key_counter: HashMap<_, _> = rules.keys().map(|&k| (k, 0)).collect();
    for part in 0..template.len() - 1 {
        let src = &template[part..part + 2];
        *key_counter.entry(src).or_insert(0) += 1;
    }

    // println!("{:?}", key_counter);

    // 'Nuff said, go for a round
    for _ in 0..rounds {
        let mut next = key_counter.clone();
        for (k, n) in key_counter {
            if n == 0 {
                continue;
            }
            let rule = &rules[k];

            next.entry(&rule.first[0..2]).and_modify(|c| *c += n);
            next.entry(&rule.second[0..2]).and_modify(|c| *c += n);
            next.entry(k).and_modify(|c| *c -= n);

            counter.entry(rule.ins).and_modify(|c| *c += n);
        }
        key_counter = next
    }

    let bucket_count = counter.len();
    let mut sorted: Vec<_> = counter.into_iter().collect();
    sorted.sort_by_key(|h| -h.1);
    // by value descending

    println!("{:?}", sorted);

    // most common - least common
    sorted[0].1 - sorted[bucket_count - 1].1
}

fn aoc_2021_14_naive(input: &str, rounds: i32) -> i64 {
    let mut lines = input.trim().lines();
    let template = lines.next().unwrap();
    lines.next();

    let rules = lines
        .filter_map(|l| l.trim().split_once(" -> "))
        .collect::<HashMap<_, _>>();

    // println!("{:?}", rules);

    let mut polymer = template.to_string();
    for _ in 0..rounds {
        let len = polymer.len();
        let mut next = String::with_capacity(len / 2 * 3);

        for part in 0..polymer.len() - 1 {
            // unconditional append first char
            next.push_str(&polymer[part..part + 1]);

            // does a rule apply?
            let src = &polymer[part..part + 2];
            if let Some(&insert) = rules.get(&src) {
                // apply it by appendig/inserting
                next.push_str(&insert);
                // 2nd char is appended in the next window
            }
        }
        // do not forget the last character (never changing through rules)
        next.push_str(&polymer[len - 1..]);

        polymer = next;
        // println!("{}", polymer);
    }

    // Do the statistics
    let mut counter = HashMap::with_capacity(26);
    for c in polymer.chars() {
        *counter.entry(c).or_insert(0) += 1;
    }
    let bucket_count = counter.len();
    let mut sorted: Vec<_> = counter.into_iter().collect();
    sorted.sort_by_key(|h| -h.1);
    // by value descending

    println!("{:?}", sorted);

    // most common - least common
    sorted[0].1 - sorted[bucket_count - 1].1
}

pub fn aoc_2021_14_a(input: &str) -> i64 {
    aoc_2021_14_naive(input, 10)
}

pub fn aoc_2021_14_b(input: &str) -> i64 {
    aoc_2014_14_less_naive(input, 40) // naive busts with memory allocation error. who thought?
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_14_a_example() {
        assert_eq!(super::aoc_2021_14_a(TEST_INPUT), 1588);
    }

    #[test]
    fn aoc_2021_14_a() {
        assert_eq!(super::aoc_2021_14_a(INPUT), 3009);
    }

    #[test]
    fn aoc_2021_14_b_example() {
        assert_eq!(super::aoc_2021_14_b(TEST_INPUT), 2188189693529);
    }

    #[test]
    fn aoc_2021_14_b() {
        assert_eq!(super::aoc_2021_14_b(INPUT), 3459822539451);
    }

    #[test]
    fn naive_vs_less_naive() {
        assert_eq!(
            super::aoc_2021_14_naive(INPUT, 10),
            super::aoc_2014_14_less_naive(INPUT, 10)
        );
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C";
}
