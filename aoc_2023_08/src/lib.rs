// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/08
    Solution idea: brute force
    plan B: Analyse paths and use math. Pattern Matching? Cycle detection on paths? Are the paths disjunct or interleaved?
    How many endpoints passes a path?

    make dot from input:
    Replace instructions by "digraph {", append "}"
    node labels off with  node[label=""];
    Replace =,(,) with ->, {, }
    for every node ending A add a line "..A [style=filled, fillcolor=green ]"
    for every node ending Z add a line "..Z [style=filled, fillcolor=red ]"

    dot -Tsvg -O -Kneato input.dot

    6 disjunct circles, start is not on circle, only one endpoint on circle. Its synchronized gears again.
    Lowest common multiple of all circle lengths (and length of instructions?).
    num::integer::lcm :D
    Do we need an offset for start?
*/
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse(input: &str) -> (&str, HashMap<&str, Node<'_>>) {
    let (instuctions, nod) = input.trim().split_once("\n\n").expect("invalid input");

    let nodes = nod
        .lines()
        .flat_map(|l| l.split_once("="))
        .map(|(label, lr)| {
            (
                label,
                lr.trim_matches(|c: char| !c.is_alphanumeric())
                    .split_once(",")
                    .expect("No pair"),
            )
        })
        .map(|(label, (left, right))| {
            (
                label.trim(),
                Node {
                    left: left.trim(),
                    right: right.trim(),
                },
            )
        })
        .collect::<HashMap<_, _>>();
    (instuctions, nodes)
}

fn walk<'a>(instuctions:&'a str, nodes: &HashMap<&'a str, Node<'a>>, start: &'a str) -> (&'a str, usize){
    instuctions
        .trim()
        .chars()
        .cycle()
        .fold_while((start, 0usize), |(node, counter), dir| {
            // dbg!((node, counter, dir));

            if node.ends_with("Z") {
                Done((node, counter))
            } else {
                Continue((
                match dir {
                    'L' => nodes[node].left,
                    'R' => nodes[node].right,
                    _ => unreachable!(),
                }, counter + 1))
            }
        })
        .into_inner()
        
}

pub fn aoc_2023_08_a(input: &str) -> usize {
    let (instuctions, nodes) = parse(input);

    let start = "AAA";
    let (_node, count) = walk(&instuctions, &nodes, start);
    count
}

pub fn aoc_2023_08_b(input: &str) -> usize {
    let (instuctions, nodes) = parse(input);
    let start_nodes: Vec<_> = nodes
        .keys()
        .filter(|&n| n.ends_with("A"))
        .map(|&n| n.clone())
        .collect();
    // println!("{:?}", nodes);

    let cycle_nodes = start_nodes.iter().map(|start| walk(&instuctions, &nodes, start)).collect::<Vec<_>>();

    println!("cycles {:?}", cycle_nodes);

    let count = cycle_nodes
        .iter()
        .fold(1, |acc, (_, cycle_len)| num::Integer::lcm(&acc, cycle_len));

    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_08_a_example() {
        assert_eq!(super::aoc_2023_08_a(TEST_INPUT), 2);
    }

    #[test]
    fn aoc_2023_08_a_example2() {
        assert_eq!(super::aoc_2023_08_a(TEST_INPUT2), 6);
    }

    #[test]
    fn aoc_2023_08_a() {
        assert_eq!(super::aoc_2023_08_a(INPUT), 19667);
    }

    #[test]
    fn aoc_2023_08_b_example() {
        assert_eq!(super::aoc_2023_08_b(TEST_INPUT3), 6);
    }

    #[test]
    fn aoc_2023_08_b() {
        assert_eq!(super::aoc_2023_08_b(INPUT), 19185263738117);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT2: &str = "
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT3: &str = "
    LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";
}
