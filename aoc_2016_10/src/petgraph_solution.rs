#![cfg(feature = "graph")]
// little experiment how to do it in petgraph after we hat it for a view anyway
use petgraph::dot::{Config, Dot};
use petgraph::{Algo, Graph, Incoming};
use regex::Regex;

use crate::solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Target {
    Output(usize),
    Bot(usize),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Value(usize, usize),
    LowHigh(usize, Target, Target),
}

fn parse(input: &str) -> Vec<Token> {
    let value_rx = Regex::new(r"\s*value (?P<val>\d+) goes to bot (?P<bot>\d+)").unwrap();
    let lowhigh_rx =
        Regex::new(r"\s*bot (?P<bot>\d+) gives low to (?P<low_to>bot|output) (?P<low_id>\d+) and high to (?P<high_to>bot|output) (?P<high_id>\d+)")
            .unwrap();

    let mut tokens = Vec::new();
    for line in input.lines() {
        println!("{line}");

        if let Some(caps) = value_rx.captures(line) {
            let val = caps["val"].parse().expect("val should be numeric");
            let bot = caps["bot"].parse().expect("target bot should be numeric");
            tokens.push(Token::Value(val, bot));
        }

        if let Some(caps) = lowhigh_rx.captures(line) {
            let bot = caps["bot"].parse().expect("target bot should be numeric");
            let low_to = parse_target(&caps["low_to"], &caps["low_id"]);
            let high_to = parse_target(&caps["high_to"], &caps["high_id"]);
            tokens.push(Token::LowHigh(bot, low_to, high_to));
        }
    }
    tokens
}

fn parse_target(to: &str, id_str: &str) -> Target {
    let id = id_str.parse().expect("id should be numeric");

    match to {
        "bot" => Target::Bot(id),
        "output" => Target::Output(id),
        _ => unreachable!("invalid target"),
    }
}

const HIGH: &str = "high";
const LOW: &str = "low";

fn to_graph(tokens: Vec<Token>) -> Graph<String, &'static str> {
    let mut gr = Graph::new();

    // remember the nodes
    let mut values = vec![None; 255];
    let mut bots = vec![None; 255];
    let mut outputs = vec![None; 255];

    for t in tokens {
        match t {
            Token::Value(val, bot) => {
                if values[val].is_none() {
                    values[val] = Some(gr.add_node(format!("Val {val}")));
                }
                if bots[bot].is_none() {
                    bots[bot] = Some(gr.add_node(format!("Bot {bot}")));
                }
                gr.add_edge(values[val].unwrap(), bots[bot].unwrap(), "");
            }
            Token::LowHigh(bot, target_low, target_high) => {
                if bots[bot].is_none() {
                    bots[bot] = Some(gr.add_node(format!("Bot {bot}")));
                }

                match target_low {
                    Target::Output(o) => {
                        if outputs[o].is_none() {
                            outputs[o] = Some(gr.add_node(format!("Output {o}")));
                        }
                        gr.add_edge(bots[bot].unwrap(), outputs[o].unwrap(), LOW);
                    }
                    Target::Bot(b) => {
                        if bots[b].is_none() {
                            bots[b] = Some(gr.add_node(format!("Bot {b}")));
                        }
                        gr.add_edge(bots[bot].unwrap(), bots[b].unwrap(), LOW);
                    }
                }

                match target_high {
                    Target::Output(o) => {
                        if outputs[o].is_none() {
                            outputs[o] = Some(gr.add_node(format!("Output {o}")));
                        }
                        gr.add_edge(bots[bot].unwrap(), outputs[o].unwrap(), HIGH);
                    }
                    Target::Bot(b) => {
                        if bots[b].is_none() {
                            bots[b] = Some(gr.add_node(format!("Bot {b}")));
                        }
                        gr.add_edge(bots[bot].unwrap(), bots[b].unwrap(), HIGH);
                    }
                }
            }
        }
    }

    // Output the tree to `graphviz` `DOT` format
    // println!("{:?}", Dot::with_config(&gr, &[Config::EdgeNoLabel]));
    println!("{:?}", Dot::new(&gr));

    gr
}

impl solution {
    pub fn distribute_chips(input: &str) -> usize {
        use petgraph::{
            algo::toposort,
            graph::Node,
            visit::{NodeFiltered, Topo},
        };

        let g = to_graph(parse(input));
        let topo = Topo::new(&g);

        // start from value nodes
        match toposort(g, None) {
            Ok(order) => {
                print!("Sorted: ");
                for i in order {
                    g.node_weight(i).map(|weight| {
                        print!("{}, ", weight);
                        weight
                    });
                }
            }
            Err(err) => {
                g.node_weight(err.node_id())
                    .map(|weight| println!("Error graph has cycle at node {}", weight));
            }
        }
        println!("");
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("value 5 goes to bot 2", Token::Value(5, 2))]
    #[case(
        "bot 2 gives low to bot 1 and high to bot 0",
        Token::LowHigh(2, Target::Bot(1), Target::Bot(0))
    )]
    #[case("value 3 goes to bot 1", Token::Value(3, 1))]
    #[case(
        "bot 1 gives low to output 1 and high to bot 0",
        Token::LowHigh(1, Target::Output(1), Target::Bot(0))
    )]
    #[case(
        "bot 0 gives low to output 2 and high to output 0",
        Token::LowHigh(0, Target::Output(2), Target::Output(0))
    )]
    #[case("value 2 goes to bot 2 ", Token::Value(2, 2))]
    fn token_parser_should(#[case] input: &str, #[case] exepected: Token) {
        assert_eq!(parse(input)[0], exepected);
    }

    #[test]
    fn get_graph() {
        let tokens = parse(INPUT);
        let gr = to_graph(tokens);

        assert!(gr.node_count() > 0);

        panic!("show it")
    }
}