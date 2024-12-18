use std::{
    collections::{BinaryHeap, HashMap},
    iter::successors,
};

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/18
    Solution idea:
    just temporal A*

    The misunderstanding that the bytes will fall one by one WHILE we are going will cost a lot of time...
*/
use aoc_utils::grid::Grid;
use glam::IVec2;

#[derive(Debug, Eq, PartialEq, Clone)]
struct Prio {
    priority: i32,
    // temporal coordinates
    pos: IVec2,
}

impl Prio {
    fn new(priority: i32, pos: IVec2) -> Self {
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
            // .then(self.pos.z.cmp(&other.pos.z)) // time
            .then(self.pos.x.cmp(&other.pos.x))
            .then(self.pos.y.cmp(&other.pos.y))
    }
}

fn parse(input: &str) -> HashMap<IVec2, i32> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(t, l)| {
            l.split_once(",").map(|n| {
                (
                    IVec2::new(n.0.parse::<i32>().unwrap(), n.1.parse::<i32>().unwrap()),
                    t as i32,
                )
            })
        })
        .collect::<HashMap<_, _>>()
}

fn a_star(end: IVec2, corrupted_after: &HashMap<IVec2, i32>, threshold: i32) -> usize {
    let grid = Grid::from_upper_bound(end, '.');
    let start = IVec2::ZERO;

    let mut frontier = BinaryHeap::new();
    let mut came_from: HashMap<IVec2, IVec2> = HashMap::new();
    let mut cost_so_far: HashMap<IVec2, i32> = HashMap::new();

    frontier.push(Prio::new(0, start));
    cost_so_far.insert(start, 0);

    // println!("{end} {threshold}");

    while let Some(Prio { priority: _, pos }) = frontier.pop() {
        // println!(
        //     "{} [{:?}] frontier {}",
        //     pos,
        //     cost_so_far.get(&pos),
        //     frontier.len()
        // );

        // end reached, path must be minimal
        if pos == end {
            // println!("came_from: {came_from:?}");
            // println!("Frontier: {frontier:?}");

            let mut path =
                successors(Some(pos), |p| (p != &start).then(|| came_from[p])).collect::<Vec<_>>();
            path.reverse();

            // for p in corrupted_after
            //     .iter()
            //     .filter(|(_p, t)| **t < threshold)
            //     .map(|(p, _t)| p)
            // {
            //     grid[*p] = '#';
            // }
            // for p in &path {
            //     grid[*p] = 'O'; //char::from_digit(p.z as u32 % 10, 10).expect("digit?");
            // }

            // println!("{grid:#}");

            return path.len() - 1; // a step costs 1 so no need to calculate cost, but steps not tiles
        }

        for (next, _) in grid.iter_axis_neighbours_with_positions(pos) {
            // is next a valid tile at time t+1?
            // Brrb. All bytes are already fallen at once before we start!
            // but we use t as a threshold if it has been fallen this run.
            if *corrupted_after.get(&next).unwrap_or(&i32::MAX) >= threshold {
                let new_cost: i32 = *cost_so_far.get(&pos).unwrap_or(&0) + 1; // it always cost 1 to go to a neighbour

                if !cost_so_far.contains_key(&next) || new_cost < cost_so_far[&next] {
                    cost_so_far.insert(next, new_cost);

                    // heuristic is simply manhattan distance in space. Ignore temporal distance or we will overestimate aka bad
                    let priority = new_cost + (end.x - next.x + end.y - next.y);
                    frontier.push(Prio {
                        priority,
                        pos: next,
                    });

                    came_from.insert(next, pos);
                }
            }
        }
    }
    usize::MAX
}

#[tracing::instrument]
pub fn aoc_2024_18_a(input: &str, end: IVec2, fallen: i32) -> usize {
    let corrupted_after = parse(input);

    a_star(end, &corrupted_after, fallen)
}

#[tracing::instrument]
pub fn aoc_2024_18_b(input: &str, end: IVec2, fallen: i32) -> String {
    let corrupted_after = parse(input);

    // Optimiize: binary search instead of linear
    let mut range = fallen..(1 + corrupted_after.len() as i32);
    let mut threshold: i32;
    // we can come from both directions to the border,
    // so remeber the lowest threshhold without path
    let mut not_found_min = range.end;

    // find first run that returns usize::MAX for no path found
    while !range.is_empty() {
        threshold = range.start + (range.end - range.start) / 2;

        if a_star(end, &corrupted_after, threshold) == usize::MAX {
            range = range.start..threshold;
            not_found_min = not_found_min.min(threshold);
        } else {
            range = (threshold + 1)..range.end;
        }
    }
    // println!(
    //     "range: {:?} threshold: {} min: {}",
    //     range, threshold, not_found_min
    // );

    corrupted_after
        .iter()
        .filter(|(_p, t)| **t == not_found_min - 1)
        .map(|(p, _)| format!("{},{}", p.x, p.y))
        .next()
        .unwrap()
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
        #[case] fallen: i32,
        #[case] exepected: usize,
    ) {
        assert_eq!(super::aoc_2024_18_a(input, bounds, fallen), exepected);
    }

    #[test]
    fn aoc_2024_18_a() {
        assert_eq!(
            super::aoc_2024_18_a(super::INPUT, IVec2::new(70, 70), 1024),
            306
        );
    }

    #[rstest]
    #[case(TEST_INPUT, IVec2::new(6, 6), 12, "6,1")]
    fn aoc_2024_18_b_example(
        #[case] input: &str,
        #[case] bounds: IVec2,
        #[case] fallen: i32,
        #[case] exepected: String,
    ) {
        assert_eq!(super::aoc_2024_18_b(input, bounds, fallen), exepected);
    }

    #[test]
    fn aoc_2024_18_b() {
        assert_eq!(
            super::aoc_2024_18_b(super::INPUT, IVec2::new(70, 70), 1024),
            "38,63"
        );
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
