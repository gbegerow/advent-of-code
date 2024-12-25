use std::collections::{BinaryHeap, HashMap};

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/16
    Solution idea:

*/
use aoc_utils::grid::{Grid, EAST};
use glam::IVec2;

#[derive(Debug, Eq, PartialEq, Clone)]
struct PriorityEntry {
    priority: i32,
    pos: IVec2,
    dir: IVec2,
}

impl PriorityEntry {
    fn new(priority: i32, pos: IVec2, dir: IVec2) -> Self {
        Self { priority, pos, dir }
    }
}

impl PartialOrd for PriorityEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min heap wanted
        other
            .priority
            .cmp(&self.priority)
            .then(self.pos.x.cmp(&other.pos.x))
            .then(self.pos.y.cmp(&other.pos.y))
            .then(self.dir.x.cmp(&other.dir.x))
            .then(self.dir.y.cmp(&other.dir.y))
    }
}

// based on https://www.redblobgames.com/pathfinding/a-star/introduction.html better readable than wikipedia article
fn a_star(grid: Grid<char>) -> i32 {
    let mut frontier = BinaryHeap::new();
    frontier.push(PriorityEntry::new(0, grid.cursor, EAST)); // prio of start doesn't matter
    let mut came_from = HashMap::new();
    // start has no came_from, no need to insert
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(grid.cursor, 0);
    let end = grid.find('E').expect("No end found");

    // estimated cost/priority most be first in tuple as it is the ordering criteria of the queue
    while let Some(PriorityEntry {
        priority: _,
        pos,
        dir: direction,
    }) = frontier.pop()
    {
        // end reached, path must be minimal
        if pos == end {
            // calculate cost of cheapest path
            // is this just cost_so_far[predecessor] + last?
            // or do we need to go back the whole chain?
        }

        // for all axial neighbours
        for (next, c) in grid.iter_axis_neighbours_with_positions(pos) {
            let new_dir = next - pos;
            // back is not a legal move
            if new_dir == direction * -1 {
                continue;
            }
            // straight ahead costs 1, turn 90° costs 1000
            let next_cost = if new_dir == direction { 1 } else { 1000 };
            let new_cost = cost_so_far[&next] + next_cost;
            if match cost_so_far.get(&next) {
                Some(c) => &new_cost < c, // found a better way to this node?
                None => true,             // there was no known way yet
            } {
                cost_so_far.insert(next.clone(), new_cost);
                // grid without diagonal movement, so use manhattan distance next to end as heuristic
                // assume a single turn
                let heuristic = end.x - next.x + end.y - next.y + 1000;
                let prio = new_cost + heuristic;
                frontier.push(PriorityEntry::new(prio, next, new_dir)); //-prio because min heap needed
                came_from.insert(next, pos);
            }
        }
    }
    // println!("Frontier: {:?}", frontier);

    unreachable!()
}

#[tracing::instrument]
pub fn aoc_2024_16_a(input: &str) -> usize {
    let mut grid = input.parse::<Grid<char>>().expect("valid grid");
    grid.find_cursor('S', '.');

    0
}

#[tracing::instrument]
pub fn aoc_2024_16_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_16_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_16_a(input), expected);
    }

    #[test]
    fn aoc_2024_16_a() {
        assert_eq!(super::aoc_2024_16_a(super::INPUT), 0);
    }

    #[rstest]
    #[case("X, X", 0)]
    fn aoc_2024_16_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_16_b(input), expected);
    }

    #[test]
    fn aoc_2024_16_b() {
        assert_eq!(super::aoc_2024_16_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "";
}
