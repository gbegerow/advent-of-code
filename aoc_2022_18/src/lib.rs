// #[allow(dead_code)]

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, PartialOrd)]
struct Voxel {
    x: i16,
    y: i16,
    z: i16,
}

impl Voxel {
    fn new(x: i16, y: i16, z: i16) -> Self { Self { x, y, z } }
    fn from(v:Vec<i16>) -> Self { Voxel { x:v[0], y: v[1], z: v[2] }}
}

pub fn aoc_2022_18_a(input: &str) -> usize {
    let neighbours: [[i16; 3]; 6] = [
        [0, 0, -1], // behind
        [0, 0, 1], // front
        [0, 1, 0],  // top
        [0, -1, 0], // below
        [-1, 0, 0], // left 
        [1, 0, 0], // right
    ];

    let mut map = HashSet::with_capacity(512);
    let mut sides = 0;
    for coord in input.trim().lines().map(|l| {
        l.split(",")
            .map(|d| d.trim().parse::<i16>().unwrap())
            .collect::<Vec<_>>()
    }).map(|v| Voxel::from(v)) {
        for delta in neighbours {
            let n = Voxel::new(
                coord.x + delta[0],
                coord.y + delta[1],
                coord.z + delta[2],
            );
            if map.contains(&n) {
                // neighbour exists, remove 1 side
                sides -= 1; print!("-");
            } else {
                // open surface, add 1 side
                sides += 1; print!("|");
            }
            map.insert(coord);
        }
    }
    sides
}

pub fn aoc_2022_18_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_18_a_example() {
        assert_eq!(super::aoc_2022_18_a(TEST_INPUT), 64);
    }

    #[test]
    fn aoc_2022_18_a() {
        assert_eq!(super::aoc_2022_18_a(include_str!("input.txt")), 4482);
    }

    #[test]
    fn aoc_2022_18_b_example() {
        assert_eq!(super::aoc_2022_18_b(TEST_INPUT), 58);
    }

    #[test]
    fn aoc_2022_18_b() {
        assert_eq!(super::aoc_2022_18_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "2,2,2
    1,2,2
    3,2,2
    2,1,2
    2,3,2
    2,2,1
    2,2,3
    2,2,4
    2,2,6
    1,2,5
    3,2,5
    2,1,5
    2,3,5";
}
