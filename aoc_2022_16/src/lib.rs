use core::fmt;
use std::collections::HashMap;


use petgraph::prelude::*;
use petgraph::dot::{Dot, Config};
use regex::Regex;
// #[allow(dead_code)]


#[derive(Debug, Clone)]
struct Node {
    label: String,
    rate: u32,
    open: bool,
    visited: bool,
}

impl Node {
    fn new(label: &str, rate: u32) -> Self { Self { label: label.to_string(), rate, open: false, visited: false } }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{} {} {}", 
            self.label, 
            self.rate, 
            if self.open {"| |"} else {">+<"}))
    }
}

pub fn aoc_2022_16_a(input: &str) -> usize {
    // I know how to implement a graph, I want to know how to use petgraph
    let mut graph = Graph::<Node, u16>::new(); // u16 has Display, () not
    let mut map = HashMap::new(); // temporary storage for index and label
    let rx = Regex::new(r"Valve (.+) has flow rate=(\d+); tunnels lead to valves (.+)").unwrap();
    for caps in rx.captures_iter(input) {
        let label = caps.get(1).unwrap().as_str(); 
        let rate = caps.get(2).unwrap().as_str().parse().unwrap(); 
        let index = *map.entry(label).or_insert_with(|| 
            graph.add_node(Node::new(label, rate))
        );
        if let Some(n) = graph.node_weight_mut(index) {
            n.rate = rate; // update rate if already inserted via edge
        }

        for t in caps.get(3).unwrap().as_str().split(", ")  {
            let target = *map.entry(t).or_insert_with(|| 
                graph.add_node(Node::new(t,0))
            );
            graph.add_edge(index, target, 0);
        }
    };

    // let mut dfs = Dfs::new(&graph, start);
    // while let Some(nx) = dfs.next(&graph) {
    //     if graph[nx].rate > 0 {
    //         graph[nx].open = true;
    //     }
    // }
    // DFS  is nice but we need to be in  control of the stack
    let max_minutes = 30;
    let mut flow = 0;
    let mut max_flow = 0;
    let mut path = Vec::with_capacity(max_minutes);
    // let mut max_path: = Vec::new();    
    let mut opened = 0;
    let mut stack = vec![map[&"AA"].clone(); max_minutes];

    while let Some(current) = stack.pop() {
        path.push(current.clone());
        if graph[current].rate > 0 {
            opened += 1;
            flow += graph[current].rate;
        }  

        if flow > max_flow {
            max_flow = flow;
            // max_path = path.clone();
            //println!("Flow {} Path {}", max_flow, path.iter().map(|i| graph[i].label).collect::<Vec<_>>().join(", "));
        }

        if !graph[current].visited {
            graph[current].visited = true;

            // only go further if we have time left
            if path.len() + opened < max_minutes{
                for n in graph.neighbors(current) {
                    stack.push(n);
                }
            } 
        }
    }
    



    println!("{}", Dot::with_attr_getters(&graph, &[Config::EdgeNoLabel],
        &|_, _| String::new(),
        &|_, (_, n)| String::from(if n.visited {"style=filled fillcolor=\"#6891fc\" fontcolor=\"white\""} else {""})
    ));

    // idea: do a BFS, restrict paths to length 30 - #Valves
    
    0
}

pub fn aoc_2022_16_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_16_a_example() {
        assert_eq!(super::aoc_2022_16_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_16_a() {
       assert_eq!(super::aoc_2022_16_a(include_str!("input.txt")), 0);
    }
    
    #[test]
    fn aoc_2022_16_b_example() {
        assert_eq!(super::aoc_2022_16_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_16_b() {
        assert_eq!(super::aoc_2022_16_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = 
    "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    Valve BB has flow rate=13; tunnels lead to valves CC, AA
    Valve CC has flow rate=2; tunnels lead to valves DD, BB
    Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
    Valve EE has flow rate=3; tunnels lead to valves FF, DD
    Valve FF has flow rate=0; tunnels lead to valves EE, GG
    Valve GG has flow rate=0; tunnels lead to valves FF, HH
    Valve HH has flow rate=22; tunnel leads to valve GG
    Valve II has flow rate=0; tunnels lead to valves AA, JJ
    Valve JJ has flow rate=21; tunnel leads to valve II";
}



