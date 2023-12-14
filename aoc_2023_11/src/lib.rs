use std::collections::HashSet;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/11
    Solution idea:
    Sparse storage not grid. For every empty line during parsing increase y coordinate by 2. 
    2nd pass for every column after empty column, increase x by spacing.
    Distance is just straight line manhattan distance (or ordered manhattan distance?). No need for any graph algo.
    itertools catesian product
*/
use itertools::Itertools;
use glam::IVec2;

pub fn aoc_2023_11_a(input: &str) -> usize {
    todo!(); // just a sketch not real code

    let mut x = 0; 
    let mut y = 0;
    let mut galaxy_map = input.trim().lines()
    .flat_map(|l| l.trim().chars().map(|c| {
        match c {
            '#' => Some( IVec2::new(x, y)),
            _ => None,
        }
    })).collect::<HashSet<_>>();

    // empty lines/columns = all.except(existing)
    let existing = galaxy_map.iter().fold((HashSet::new(), HashSet::new(), i64, i64), 
        |(x_set, y_set, max_x, max_y) , coord|{
        x_set.insert(coord.x);
        y_set.insert(coord.y);
        (x_set, y_set, max_x: max_x.max(coord.x), max_y:max_y.max(coord.y))
    });
    let empty_x = HashSet::from_iter(0..existing.2).difference(existing.0).collect::Vec<_>();
    let empty_y = HashSet::from_iter(0..existing.3).difference(existing.1).collect::Vec<_>();

    // expand universe: map every empty row/column to one spacing more than its predecessor. 
    // From highest to lowest add spacing to galaxy if coord is higher than empty
    
    0
}

pub fn aoc_2023_11_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2023_11_a_example() {
        assert_eq!(super::aoc_2023_11_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_11_a() {
       assert_eq!(super::aoc_2023_11_a(INPUT), 0);
    }
    
    #[test]
    fn aoc_2023_11_b_example() {
        assert_eq!(super::aoc_2023_11_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2023_11_b() {
        assert_eq!(super::aoc_2023_11_b(INPUT), 0);
    }

    
    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    ...#......
    .......#..
    #.........
    ..........
    ......#...
    .#........
    .........#
    ..........
    .......#..
    #...#.....";
}



