use std::collections::BTreeSet;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/23
    Solution idea:

*/
// use petgraph::dot::{Config, Dot};
use petgraph::graphmap::UnGraphMap;

fn parse(input: &str) -> UnGraphMap<&str, ()> {
    UnGraphMap::<_, ()>::from_edges(input.trim().lines().flat_map(|l| l.trim().split_once('-')))
}

#[tracing::instrument]
pub fn aoc_2024_23_a(input: &str) -> usize {
    let g = parse(input);
    // println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let t_nodes = &g.nodes().filter(|n| n.starts_with('t')).collect::<Vec<_>>();

    let mut t_sets = BTreeSet::new();
    for t in t_nodes {
        // general form of cross product:  let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
        let sets = g
            .neighbors(t)
            .flat_map(|n1| g.neighbors(t).map(move |n2| (n1, n2)))
            // every pair only once and pairs must be different and both must be neighbors
            // how to use is_adjacent?
            .filter(|pair| g.neighbors(pair.0).any(|b| b == pair.1))
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
pub fn aoc_2024_23_b(input: &str) -> String {
    let g = parse(input);

    // tarjan is not the right tool, it finds all strongly connected components, not a clique

    // find  maximum clique, do not limit to nodes starting with t
    // BTreeSet keeps the sets sorted
    let mut max_clique = BTreeSet::new();
    let nodes = &g.nodes().collect::<Vec<_>>();

    for t in nodes {
        let clique = g
            .neighbors(t)
            // starting with t, add neighbor n if all current members of clique are neighbors of n
            .fold(BTreeSet::from([*t]), |mut clique, n| {
                if clique.iter().all(|in_clique| {
                    g.neighbors(n)
                        .any(|neighbor_of_n| *in_clique == neighbor_of_n)
                }) {
                    clique.insert(n);
                }
                clique
            });

        // println!("Clique from {t}: {clique:?}");

        if clique.len() > max_clique.len() {
            max_clique = clique;
        }
    }
    // println!("{t_sets:?}");

    max_clique.iter().cloned().collect::<Vec<&str>>().join(",")
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
        assert_eq!(super::aoc_2024_23_a(super::INPUT), 1284);
    }

    #[rstest]
    #[case(TEST_INPUT, "co,de,ka,ta")]
    fn aoc_2024_23_b_example(#[case] input: &str, #[case] exepected: String) {
        assert_eq!(super::aoc_2024_23_b(input), exepected);
    }

    #[test]
    fn aoc_2024_23_b() {
        // not "ac,ag,jd,jl,jz,kq,lw,nf,pp,sj,tc,ua", do not limit to t_nodes as start
        assert_eq!(
            super::aoc_2024_23_b(super::INPUT),
            "bv,cm,dk,em,gs,jv,ml,oy,qj,ri,uo,xk,yw"
        );
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
}
