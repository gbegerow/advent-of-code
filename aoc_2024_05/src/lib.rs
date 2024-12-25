// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/05
Solution idea:
rules build a directed graph wich we could toplogical sort for the right order
unccesary complex? after solving b, YES!!!

find all rule violation for print
for sorting in b we need topological order

input rules seems to have cycles 😱
Lots of cycles

 make dot from input:
    Insert by "digraph {", remove prints,  append "}"
    node labels off with  node[label=""];
    Replace | with ->, insert ; at end of lines

    dot -Tsvg -O -Kcirco rules.dot


*/

use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

fn parse(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let Some((rules_def, pages_def)) = input.split_once("\n\n") else {
        panic!("invalid input")
    };

    let rules = rules_def
        .lines()
        .flat_map(|l| l.trim().split_once('|'))
        .flat_map(|(l, r)| match (l.parse::<u32>(), r.parse::<u32>()) {
            (Ok(a), Ok(b)) => Some((a, b)),
            _ => None, // yes we ignore the error in AoC
        })
        .collect::<Vec<_>>();

    let prints = pages_def
        .lines()
        .map(|l| {
            l.split(',')
                .flat_map(|n| n.parse::<u32>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (rules, prints)
}

#[allow(dead_code)]
fn get_without_incoming(g: &Vec<(u32, u32)>) -> Vec<u32> {
    let on_right = g.iter().map(|(_, r)| r).collect::<HashSet<_>>();
    g.iter()
        .filter(|(l, _)| !on_right.contains(l))
        .map(|(l, _)| *l)
        .collect()
}

#[allow(dead_code)]
fn topo_sort(rules: &Vec<(u32, u32)>) -> Vec<u32> {
    // Kahn' algorithm https://en.wikipedia.org/wiki/Topological_sorting

    let mut g = rules.clone();
    let mut sorted = Vec::with_capacity(rules.len() * 2);
    let mut no_incoming = VecDeque::from(get_without_incoming(&g));

    while let Some(n) = no_incoming.pop_front() {
        if !sorted.contains(&n) {
            sorted.push(n);
        }

        // remove outgoing edges of n from graph
        g.retain(|&(l, _)| l != n);
        // add new top level to queue
        no_incoming.extend(get_without_incoming(&g));
    }

    // the real input does contain a cycle! Do we need to care? Unfoutunately ayes
    //assert!(g.len() == 0, "graph contains cycle!");
    println!("sorted: {}  {:?} \nrest: {:?}", sorted.len(), sorted, g);
    sorted
}

#[allow(dead_code)]
fn get_rule_violations(print: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Vec<(u32, u32)> {
    rules
        .iter()
        .filter(|rule| !check_rule(rule, print))
        .cloned()
        .collect()
}

fn get_first_violation(print: &Vec<u32>, rules: &Vec<(u32, u32)>) -> Option<(u32, u32)> {
    rules
        .iter()
        .filter(|rule| !check_rule(rule, print))
        .cloned()
        .next()
}

fn check_all(print: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    get_first_violation(print, rules).is_none()
    // get_rule_violations(print, rules).len() == 0

    // rules.iter().all(|rule| check_rule(rule, print))
}

fn check_rule(rule: &(u32, u32), print: &Vec<u32>) -> bool {
    let (l, r) = rule;
    let mut r_seen = false;
    for page in print {
        // have we already found r and now on l? Rule violation
        if page == l && r_seen {
            return false;
        }
        // on l but r was not seen. Valid for this rule
        if page == l && !r_seen {
            return true;
        }
        // we see r but l will either come later (violation) or come never (valid)
        if page == r {
            r_seen = true;
        }
    }
    // l was never seen, so no violation
    return true;
}

#[tracing::instrument]
pub fn aoc_2024_05_a(input: &str) -> u32 {
    let (rules, prints) = parse(input);

    // check which prints are already in correct order
    // todo: how?
    // might be easier to get rule violations (r appears before l)
    let mut res = 0;
    for print in prints {
        if check_all(&print, &rules) {
            // find the middle pagenumber
            res += print[print.len() / 2];
        }
    }

    // sum of middle pagenumbers
    res
}

// #[tracing::instrument]
// pub fn aoc_2024_05_b(input: &str) -> u32 {
//     let (rules, prints) = parse(input);
//     let sorted = topo_sort(&rules);
//     let sorted_index = sorted
//         .iter()
//         .enumerate()
//         .map(|(i, n)| (n, i as u32))
//         .collect::<HashMap<_, _>>();

//     let mut res = 0;
//     for mut print in prints {
//         // rules definition contains lots of circular definitions
//         // filter the rules by use pages does not solve the circles.
//         // what now???

//         if !print.is_sorted_by_key(|page| sorted_index.get(page).unwrap_or(page)) {
//             // if page is constraint use index else use page number itself as order is not important
//             print.sort_by_key(|page| sorted_index.get(page).unwrap_or(page).clone());
//             // find the middle pagenumber
//             res += print[print.len() / 2];
//         }
//     }
//     res
// }

#[tracing::instrument]
pub fn aoc_2024_05_b(input: &str) -> u32 {
    let (rules, prints) = parse(input);
    let rule_set = rules.iter().cloned().collect::<HashSet<_>>();

    let mut res = 0;
    for print in prints {
        // is ok, already handled in part 1?
        if check_all(&print, &rules) {
            continue;
        }

        let mut print = print;
        print.sort_by(|a, b| {
            // is there a rule that says a must be before b? The rules are just sorting rules 🤦
            // optimizing candidate, make contains test faster, eg. hashset.
            // just about double the performance with standard HashSet (aka HashBrown) vs linear search (1.349 ms vs 3.689 ms)
            if rule_set.contains(&(*a, *b)) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        // was faulted but now it is corrected
        res += print[print.len() / 2];
    }
    res
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 143)]
    fn aoc_2024_05_a_example(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(super::aoc_2024_05_a(input), expected);
    }

    #[test]
    fn aoc_2024_05_a() {
        assert_eq!(super::aoc_2024_05_a(super::INPUT), 6242);
    }

    #[rstest]
    #[case(TEST_INPUT, 123)]
    fn aoc_2024_05_b_example(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(super::aoc_2024_05_b(input), expected);
    }

    #[test]
    fn aoc_2024_05_b() {
        assert_eq!(super::aoc_2024_05_b(super::INPUT), 5169);
    }

    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
}
