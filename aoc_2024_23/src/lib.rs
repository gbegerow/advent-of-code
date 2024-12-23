use std::collections::HashSet;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/23
    Solution idea:

*/
// use petgraph::dot::{Config, Dot};
use petgraph::graphmap::UnGraphMap;

fn parse<'a>(input: &'a str) -> UnGraphMap<&'a str, ()> {
    UnGraphMap::<_, ()>::from_edges(input.trim().lines().flat_map(|l| l.trim().split_once('-')))
}

#[tracing::instrument]
pub fn aoc_2024_23_a(input: &str) -> usize {
    let g = parse(input);
    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let t_nodes = &g.nodes().filter(|n| n.starts_with('t')).collect::<Vec<_>>();

    let mut t_sets = HashSet::new();
    for t in t_nodes {
        // general form of cross product:  let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
        let sets = g
            .neighbors(t)
            .flat_map(|n1| g.neighbors(t).map(move |n2| (n1, n2)))
            // every pair only once and pairs must be different and both must be neighbors
            // how to use is_adjacent?
            .filter(|pair| pair.0 > pair.1 && g.neighbors(pair.0).find(|b| b == &pair.1).is_some())
            .map(|p| {
                let mut set = vec![t, p.0, p.1];
                set.sort();
                set
            })
            .collect::<Vec<_>>();

        println!("Sets from {t}: {sets:?}");

        t_sets.extend(sets);
    }
    // println!("{t_sets:?}");

    t_sets.len()
}

#[tracing::instrument]
pub fn aoc_2024_23_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 7)]
    fn aoc_2024_23_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_23_a(input), exepected);
    }

    #[test]
    fn aoc_2024_23_a() {
        assert_eq!(super::aoc_2024_23_a(super::INPUT), 0);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_23_b_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_23_b(input), exepected);
    }

    #[test]
    fn aoc_2024_23_b() {
        assert_eq!(super::aoc_2024_23_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "
    kh-tc
    qp-kh
    de-cg
    ka-co
    yn-aq
    qp-ub
    cg-tb
    vc-aq
    tb-ka
    wh-tc
    yn-cg
    kh-ub
    ta-co
    de-co
    tc-td
    tb-wq
    wh-td
    ta-ka
    td-qp
    aq-cg
    wq-ub
    ub-vc
    de-ta
    wq-aq
    wq-vc
    wh-yn
    ka-de
    kh-ta
    co-tc
    wh-qp
    tb-vc
    td-yn";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
