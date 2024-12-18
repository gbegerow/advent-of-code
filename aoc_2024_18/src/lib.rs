use std::{
    collections::{BinaryHeap, HashMap},
    iter::successors,
};

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/18
    Solution idea:
    just temporal A*
*/
use aoc_utils::grid::Grid;
use glam::{IVec2, IVec3};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Prio {
    priority: i32,
    // temporal coordinates
    pos: IVec3,
}

impl Prio {
    fn new(priority: i32, pos: IVec3) -> Self {
        Self { priority, pos }
    }
}

impl PartialOrd for Prio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Prio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min heap wanted
        other
            .priority
            .cmp(&self.priority)
            .then(self.pos.z.cmp(&other.pos.z)) // time
            .then(self.pos.x.cmp(&other.pos.x))
            .then(self.pos.y.cmp(&other.pos.y))
    }
}

fn a_star(end: IVec2, corrupted_after: HashMap<IVec2, i32>) -> usize {
    let mut grid = Grid::from_upper_bound(end, '.');
    let start = IVec3::ZERO;

    let mut frontier = BinaryHeap::new();
    let mut came_from: HashMap<IVec3, IVec3> = HashMap::new();
    let mut cost_so_far: HashMap<IVec3, i32> = HashMap::new();

    frontier.push(Prio::new(0, start));
    cost_so_far.insert(start, 0);

    while let Some(Prio { priority: _, pos }) = frontier.pop() {
        // println!("{} [{:?}]", pos, cost_so_far.get(&pos));
        let pos2 = pos.truncate();
        let t = pos.z;

        // end reached, path must be minimal
        if pos2 == end {
            let mut path = successors(Some(pos), |p| (p != &start).then(|| came_from[p]))
                // we have to convert from Option<&IVec2> to Option<IVec>
                //came_from.get(p).copied()) // runs endless. Why? start should yield None as there is no entry
                .collect::<Vec<_>>();
            path.reverse();

            for p in corrupted_after.keys() {
                grid[*p] = '#';
            }
            for p in &path {
                grid[p.truncate()] = char::from_digit(p.z as u32 % 10, 10).expect("digit?");
                // 'O';
            }

            println!("{grid:#}");

            return path.len() - 1; // a step costs 1 so no need to calculate cost, but steps not tiles
        }

        for (next, _) in grid.iter_axis_neighbours_with_positions(pos2) {
            // is next a valid tile at time t+1?
            if *corrupted_after.get(&next).unwrap_or(&i32::MAX) > t {
                let next3 = next.extend(t + 1);
                let new_cost: i32 = *cost_so_far.get(&pos).unwrap_or(&0) + 1; // it always cost 1 to go to a neighbour

                if !cost_so_far.contains_key(&next3) || cost_so_far[&next3] < new_cost {
                    cost_so_far.insert(next3, new_cost);

                    // heuristic is simply manhattan distance in space. Ignore temporal distance or we will overestimate aka bad
                    let priority = new_cost + (end.x - next.x + end.y - next.y);
                    frontier.push(Prio {
                        priority,
                        pos: next3,
                    });

                    came_from.insert(next3, pos);
                }
            }
        }
    }
    0
}

#[tracing::instrument]
pub fn aoc_2024_18_a(input: &str, end: IVec2, fallen: usize) -> usize {
    let corrupted_after = input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(_t, l)| {
            l.split_once(",").map(|n| {
                (
                    IVec2::new(n.0.parse::<i32>().unwrap(), n.1.parse::<i32>().unwrap()),
                    0, //t as i32,
                )
            })
        })
        .take(fallen)
        .collect::<HashMap<_, _>>();

    a_star(end, corrupted_after)
}

#[tracing::instrument]
pub fn aoc_2024_18_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use glam::IVec2;
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, IVec2::new(6, 6), 12, 22)]
    fn aoc_2024_18_a_example(
        #[case] input: &str,
        #[case] bounds: IVec2,
        #[case] fallen: usize,
        #[case] exepected: usize,
    ) {
        assert_eq!(super::aoc_2024_18_a(input, bounds, fallen), exepected);
    }

    #[test]
    fn aoc_2024_18_a() {
        assert_eq!(
            super::aoc_2024_18_a(super::INPUT, IVec2::new(70, 70), 1024),
            0 // 141 < x < 311
        );
    }

    #[rstest]
    #[case("X, X", 0)]
    fn aoc_2024_18_b_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_18_b(input), exepected);
    }

    #[test]
    fn aoc_2024_18_b() {
        assert_eq!(super::aoc_2024_18_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
}
