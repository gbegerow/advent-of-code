// #[allow(dead_code)]
/* Find the task un*der https://adventofcode.com/2016/day/10
    Solution idea:

    Not sure what this is? Are the instructions dynamic routing a token issued at one input routed following commands?
    Or is this a static definition of an I/O network / graph?

    Assumption: Static directed graph from
    * Values (Nodes without input and 1 output),
    * Bots (Nodes with an unknown amount of input and 2 outputs)
    * Outputs (Nodes with an uknown amount of input and no output)
    Find node on path from Value to Output wich is common for value 61 and 17

    Assertions:
    * Bots have only 1 definition of High / Low
    * Values have only 1 definition of target
    * A bot can only have 2 different values (?)
    * Bots starts empty

    Add input to petgraph and look at graphvz output

    Assumption holds. Some kind of layered flow network.

    Use petgraph for the flow or roll own

    Store bot in array 2|bots|, even is low, odd is high
    Store values in array 2|bots| Option(valtype)
    Output does not need to be stored (at least for a)
    => bot (low target, high target, low value?, high value?, [layer])
*/
mod petgraph_solution;
mod plain_solution;

// #[cfg_attr(feature = "graph")]
// use petgraph_solution::distribute_chips;
// #[cfg_attr(feature != "graph")]
use plain_solution::distribute_chips;

pub fn aoc_2016_10_a(input: &str) -> usize {
    let bots = distribute_chips(input);
    bots.bot_17_61
}

pub fn aoc_2016_10_b(input: &str) -> usize {
    let bots = distribute_chips(input);
    bots.get_part_b()
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn aoc_2016_10_a() {
        assert_eq!(super::aoc_2016_10_a(INPUT), 98);
    }

    #[test]
    fn aoc_2016_10_b() {
        assert_eq!(super::aoc_2016_10_b(INPUT), 4042);
    }

    const INPUT: &str = include_str!("input.txt");
}
