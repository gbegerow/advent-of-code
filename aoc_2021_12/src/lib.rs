use std::collections::HashMap;

/*
   Solution idea: modify recursive DFS. How to mark lowercase letters as visited? Just remember the path aka stack
   We are only interested in the count of different paths NOT in the length
*/
// TODO: move this to a struct? Too many parameters
fn bfs<'a>(
    edges: &HashMap<&'a str, Vec<&'a str>>,
    current: &'a str,
    path: &mut Vec<&'a str>,
    allow_twice_once: bool,
    mut has_seen_twice: bool,
) -> u32 {
    if current == "end" {
        // println!("{:?}", path);

        return 1; // We reached end, count this, now unroll the stack
    }

    if current.chars().all(|c| c.is_lowercase()) {
        // start and all lowercase nodes
        if path.contains(&current) {
            // do not visit it twice
            if !allow_twice_once || has_seen_twice {
                return 0; // ignore this path
            }

            // is this the first time a lowercase node was visited twice? We don't care about which node, it might even be the same
            // a flag is sufficient
            has_seen_twice = true;
        }
    }

    path.push(current);
    // dfs, visit every child node, act on the return
    let num_successfull_paths = edges[current]
        .iter()
        .map(|child_node| bfs(edges, child_node, path, allow_twice_once, has_seen_twice))
        .sum(); // sum  1 for every successfull path, 0 for every ignored one

    path.pop();

    num_successfull_paths
}

fn aoc_2021_12(input: &str, allow_twice_once: bool) -> u32 {
    let mut edges = HashMap::new();
    for line in input.trim().lines() {
        if let Some((from, to)) = line.trim().split_once("-") {
            //    println!("{from} -> {to}");
            edges.entry(from).or_insert(Vec::new()).push(to);
            if from != "start" {// easier to model it bidirectional but never go back to start
                edges.entry(to).or_insert(Vec::new()).push(from); 
            }
        }
    }
    // println!("Edges {:?}", edges);

    bfs(&edges, "start", &mut Vec::new(), allow_twice_once, false)
}

pub fn aoc_2021_12_a(input: &str) -> u32 {
    aoc_2021_12(input, false)
}

pub fn aoc_2021_12_b(input: &str) -> u32 {
    aoc_2021_12(input, true)
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_12_a_example() {
        assert_eq!(super::aoc_2021_12_a(TEST_INPUT), 10);
    }

    #[test]
    fn aoc_2021_12_a_example2() {
        assert_eq!(super::aoc_2021_12_a(TEST_INPUT2), 19);
    }

    #[test]
    fn aoc_2021_12_a() {
        assert_eq!(super::aoc_2021_12_a(INPUT), 4413);
    }

    #[test]
    fn aoc_2021_12_b_example() {
        assert_eq!(super::aoc_2021_12_b(TEST_INPUT), 36);
    }

    #[test]
    fn aoc_2021_12_b() {
        assert_eq!(super::aoc_2021_12_b(INPUT), 118803);
    }

    const INPUT: &str = include_str!("input.txt");

    // 10 Paths
    const TEST_INPUT: &str = "
    start-A
    start-b
    A-c
    A-b
    b-d
    A-end
    b-end";

    // 19 Paths
    const TEST_INPUT2: &str = "
    dc-end
    HN-start
    start-kj
    dc-start
    dc-HN
    LN-dc
    HN-end
    kj-sa
    kj-HN
    kj-dc";
}
