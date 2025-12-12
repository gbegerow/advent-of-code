// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/11
    Solution idea:

*/

use ahash::{HashMap, HashMapExt};
use std::hash::Hash;

// petgraph variant, not working yet
// use petgraph::{
//     // dot::{Config, Dot},
//     graph::{DiGraph, NodeIndex},
// };

// fn to_graph(input: &str) -> (DiGraph<&str, ()>, HashMap<&str, NodeIndex>) {
//     let mut gr: DiGraph<_, _> = DiGraph::new();

//     let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();

//     for line in input.trim().lines() {
//         let line = line.trim();
//         let parts: Vec<&str> = line.split(':').collect();
//         let parent = parts[0].trim();
//         let children = parts[1].trim().split_whitespace();

//         let parent_index = *nodes.entry(parent).or_insert_with(|| gr.add_node(parent));

//         for child in children {
//             let child_index = *nodes.entry(child).or_insert_with(|| gr.add_node(child));
//             gr.add_edge(parent_index, child_index, ());
//         }
//     }

//     (gr, nodes)
// }
// #[tracing::instrument]
// pub fn aoc_2025_11_a(input: &str) -> usize {
//     let (graph, nodes) = to_graph(input);
//     let node_names: HashMap<NodeIndex, &str> =
//         nodes.iter().map(|(name, idx)| (*idx, *name)).collect();
//     // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

//     let start = nodes["you"];
//     let end = nodes["out"];

//     let mut queue = VecDeque::from(vec![start]);
//     let mut visited = vec![start]; // keep track of visited nodes to avoid cycles and remember processing order
//     let mut seen = HashMap::new();
//     seen.insert(start, 1);

//     while let Some(current) = queue.pop_front() {
//         println!("Visiting node {}", node_names[&current]);

//         // when and how to propagate the number of paths down?
//         visited.push(current);
//         if current == end {
//             continue; // do not expand end node
//         }
//         let path_count = seen[current];

//         for neighbor in graph.neighbors_directed(current, petgraph::Direction::Outgoing) {
//             // did we already visit this node? Beware of cycles!
//             if visited.contains(&neighbor) {
//                 continue;
//             }
//             queue.push_back(neighbor);
//         }
//     }

//     // // end was never added to visited, so add it now
//     // for &current in &visited {
//     //     // keep track how often we visited a node, it might be reachable via different paths
//     //     // count paths on processing, as now the count of predecessors is known
//     //     let predecessors = graph
//     //         .neighbors_directed(current, petgraph::Direction::Incoming)
//     //         .filter_map(|idx| seen.get(&idx))
//     //         .sum::<usize>();
//     //     println!(
//     //         "{} can be reached via {predecessors} paths",
//     //         node_names[&current]
//     //     );
//     //     let path_count = predecessors.min(1);
//     //     seen.insert(current, path_count);

//     //     println!(
//     //         "Node {} was reached {path_count} times",
//     //         node_names[&current]
//     //     );
//     // }

//     // seen.iter()
//     //     .map(|(&index, count)| (node_names[&index], count))
//     //     .for_each(|(name, count)| {
//     //         println!("Node {name} was reached {count} times");
//     //     });

//     // how often did we reach the end node?
//     *seen.get(&end).unwrap_or(&0)
// }

// modified pathfinding variant
// pathfinding uses memoized DFS instead of BFS to find all paths
// Why is this better? No more uncertainty about when to propagate path counts

// based on https://github.com/evenfurther/pathfinding/blob/main/src/directed/count_paths.rs
// ahash seems to work better here than FxHashMap
fn cached_count_paths<T, FN, IN, FS>(
    start: T,
    successors: &mut FN,
    success: &mut FS,
    cache: &mut HashMap<T, usize>,
) -> usize
where
    T: Eq + Hash,
    FN: FnMut(&T) -> IN,
    IN: IntoIterator<Item = T>,
    FS: FnMut(&T) -> bool,
{
    if let Some(&n) = cache.get(&start) {
        return n;
    }

    let count = if success(&start) {
        1
    } else {
        // hhttps://doc.rust-lang.org/std/iter/fn.successors.html
        successors(&start)
            .into_iter()
            .map(|successor| cached_count_paths(successor, successors, success, cache))
            .sum()
    };

    cache.insert(start, count);

    count
}

fn to_adjacency_list(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .trim()
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let parent = parts[0].trim();
            let children: Vec<&str> = parts[1].split_whitespace().collect();
            (parent, children)
        }).chain(std::iter::once(("out", vec![]))) // ensure 'out' node exists
        .collect::<HashMap<_, _>>()
}

#[tracing::instrument]
pub fn aoc_2025_11_a(input: &str) -> usize {
    // adjacency list
    let nodes = to_adjacency_list(input);

    let start = "you";
    let end = "out";

    cached_count_paths(
        start,
        &mut |node: &&str| nodes.get(*node).cloned().unwrap_or_default(),
        &mut |node: &&str| *node == end,
        &mut HashMap::new(),
    )
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
struct NodeData<'a> {
    name: &'a str,
    visited_dac: bool,
    visited_fft: bool,
}

impl<'a> NodeData<'a> {
    fn new(name: &'a str) -> Self {
        Self {
            name,
            visited_dac: false,
            visited_fft: false,
        }
    }
}

#[tracing::instrument]
pub fn aoc_2025_11_b(input: &str) -> usize {
    // adjacency list
    let nodes = to_adjacency_list(input);
    println!("Nodes: {:?}", nodes);

    let start = NodeData::new("svr");
    let end = NodeData {
        name: "out",
        visited_dac: true,
        visited_fft: true,
    };

    cached_count_paths(
        start,
        &mut |node: &NodeData| {
            // move access to node out of closure
            let _parent = node.name;
            let visited_dac = node.visited_dac;
            let visited_fft = node.visited_fft;
            nodes
                .get(node.name)
                .cloned()
                .unwrap_or_default()
                .into_iter()
                // .inspect(move |name| { println!("{} -> {}", parent, name); })  
                .map(move |name| {
                    NodeData {
                        name,
                        // track if we have visited dac or fft or now visiting them
                        visited_dac: visited_dac || name == "dac",
                        visited_fft: visited_fft || name == "fft"
                    }
                })
                // .inspect(move |n| {
                //     println!("{:?} -> {:?}", parent, n);
                // })
        },
        &mut |node: &NodeData| node.name == end.name && node.visited_dac && node.visited_fft,
        &mut HashMap::new(),
    )
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 5)]
    #[case(TEST_INPUT_2, 7)]
    fn aoc_2025_11_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_11_a(input), expected);
    }

    #[test]
    fn aoc_2025_11_a() {
        assert_eq!(super::aoc_2025_11_a(super::INPUT), 796);
    }

    #[rstest]
    #[case(TEST_INPUT_3, 2)]
    fn aoc_2025_11_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_11_b(input), expected);
    }

    #[test]
    fn aoc_2025_11_b() {
        assert_eq!(super::aoc_2025_11_b(super::INPUT), 294053029111296);
    }

    const TEST_INPUT: &str = "
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    // longer way from start to target should propagate number of paths properly
    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "
aaa: you hhh
you: bbb ccc vvv
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
vvv: www
www: xxx yyy
xxx: yyy
yyy: zzz
zzz: out";

    const TEST_INPUT_3: &str = "
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
}
