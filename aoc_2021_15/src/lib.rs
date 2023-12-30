// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2021/day/15
    Solution idea:
    A* with costs
*/
use std::collections::{BinaryHeap, HashMap};

// based on https://www.redblobgames.com/pathfinding/a-star/introduction.html better readable than wikipedia article
fn a_star(grid: Vec<Vec<i32>>) -> i32 {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let start = (0, 0); // start upper left
    let end = (width - 1, height - 1); // end lower right
    let mut frontier = BinaryHeap::new();
    frontier.push((0, start.0, start.1)); // prio of start doesn't matter
    let mut came_from = HashMap::new();
    // start has no came_from, no need to insert
    let mut cost_so_far = HashMap::new();
    cost_so_far.insert(start.clone(), 0);

    // estimated cost/priority most be first in tuple as it is the ordering criteria of the queue
    while let Some((_, x, y)) = frontier.pop() {
        // end reached, path must be minimal
        if x == end.0 && y == end.1 {
            // reconstruct path via came_from
            // let mut path = Vec::new();
            // let mut current = &end;
            // while current != &start  {  // cost of start does not count
            //     path.push(current);
            //     current = &came_from[current];
            // }
            // path.reverse();
            // println!("Path: {:?}", path);

            // accumulate cost of path
            let mut cost_of_path = 0;
            let mut current = &end;
            while current != &start { // can this be written in a closed function i.e. with fold?
                // cost of start does not count
                cost_of_path += grid[current.0 as usize][current.1 as usize];
                current = &came_from[current];
            }
            return cost_of_path;
        }

        // for all axial neighbours
        for (next_x, next_y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
            let next = (next_x, next_y);
            // if let Some()... get takes care of border conditions
            if let Some(next_cost) = grid
                .get(next_x as usize)
                .and_then(|l| l.get(next_y as usize))
            {
                let new_cost = cost_so_far[&(x,y)] // cost_from_start[current]
                    + next_cost; // + cost[current, next]
                if match cost_so_far.get(&next) {
                    Some(c) => &new_cost < c, // found a better way to this node?
                    None => true,             // there was no known way yet
                } {
                    cost_so_far.insert(next.clone(), new_cost);
                    // grid without diagonal movement, so use manhattan distance next to end as heuristic
                    let heuristic = end.0 - next_x + end.1 - next_y;
                    let prio = new_cost + heuristic;
                    frontier.push((-prio, next_x, next_y)); //-prio because min heap needed
                    came_from.insert(next, (x, y));
                }
            }
        }
        // println!("Frontier: {:?}", frontier);
    }

    unreachable!()
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    assert!(input.trim().len() > 0);
    input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .bytes()
                .map(|c| (c - b'0') as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn aoc_2021_15_a(input: &str) -> i32 {
    let grid = parse(input);
    // grid.iter().for_each(|r| println!("{:?}", r));
    a_star(grid)
}

pub fn aoc_2021_15_b(input: &str) -> i32 {
    let tile = parse(input);

    // expand to 5x5 tiles
    let expand_by = 5;
    let (width, height) = (tile[0].len(), tile.len());
    let mut grid = vec![vec![0; width * expand_by]; height * expand_by];
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            // if x < 12 && y < 12 { println!("x: {} y: {} x/w: {} y/h: {} ", x, y, (x / width) as i32, (y / height) as i32); }
            grid[x][y] =
                match tile[x % width][y % height] + (x / width) as i32 + (y / height) as i32 {
                    c if c < 10 => c,
                    c if c >= 10 => c - 9,
                    _ => unreachable!(),
                }; // result is wraped to 1..10
        }
    }
    // grid.iter().for_each(|r| println!("{:?}", r));

    a_star(grid)
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_15_a_example() {
        assert_eq!(super::aoc_2021_15_a(TEST_INPUT), 40);
    }

    #[test]
    fn aoc_2021_15_a() {
        assert_eq!(super::aoc_2021_15_a(INPUT), 741);
    }

    #[test]
    fn aoc_2021_15_b_example() {
        assert_eq!(super::aoc_2021_15_b(TEST_INPUT), 315);
    }

    #[test]
    fn aoc_2021_15_b() {
        assert_eq!(super::aoc_2021_15_b(INPUT), 2976);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    1163751742
    1381373672
    2136511328
    3694931569
    7463417111
    1319128137
    1359912421
    3125421639
    1293138521
    2311944581";
}
