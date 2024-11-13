// #[allow(dead_code)]

use std::{cmp::Ordering, collections::BTreeSet};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Voxel {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl Voxel {
    fn new(x: i16, y: i16, z: i16) -> Self {
        Self { x, y, z }
    }
    fn from(v: Vec<i16>) -> Self {
        Voxel {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

impl Ord for Voxel {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x
            .cmp(&other.x)
            .then_with(|| self.y.cmp(&other.y))
            .then_with(|| self.z.cmp(&other.z))
    }
}
impl PartialOrd for Voxel {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse_voxel(input: &str) -> BTreeSet<Voxel> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split(",")
                .map(|d| d.trim().parse::<i16>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| Voxel::from(v))
        .collect()
}

fn count_sides(input: &str, max_dist: u16) -> usize {
    let neighbours = get_neigbours();

    let map = parse_voxel(input);
    // let enclave = BTreeSet::new();
    // println!("{} {:?}", map.len(), map);

    // fill an outside shell
    let outside = fill_outside(&map);

    let mut sides = 0; // sides might get negative temporalily if all the first voxel are inside, as we no longer work in insertion order
    for coord in &map {
        for delta in neighbours {
            let mut found = false;
            for dist in 1..max_dist + 1 {
                // let dist = 1;
                let n = Voxel::new(
                    coord.x + (delta[0] * dist as i16),
                    coord.y + (delta[1] * dist as i16),
                    coord.z + (delta[2] * dist as i16),
                );

                if outside.contains(&n) {
                    found = false; // this is outside for sure
                    break;
                }

                if map.contains(&n) {
                    // if dist > 1 {
                    //     println!("From {:?} Found {:?} Distance {}", coord, n, dist);
                    // }

                    found = true;
                    break;
                }
            }

            if !found {
                // open surface, add 1 side
                sides += 1;
            }
        }
    }
    sides as usize
}

fn get_neigbours() -> [[i16; 3]; 6] {
    let neighbours: [[i16; 3]; 6] = [
        [0, 0, -1], // behind
        [0, 0, 1],  // front
        [0, 1, 0],  // top
        [0, -1, 0], // below
        [-1, 0, 0], // left
        [1, 0, 0],  // right
    ];
    neighbours
}

fn fill_outside(map: &BTreeSet<Voxel>) -> BTreeSet<Voxel> {
    let mut outside = BTreeSet::new();
    let (min, max) = find_bounds(map);

    let mut stack = vec![max.clone()]; // this can reach for sure the outside

    let neighbours = get_neigbours();
    while let Some(coord) = stack.pop() {
        if outside.contains(&coord) || map.contains(&coord) {
            continue;
        } // ignore known outside && inside

        for delta in neighbours {
            let (x, y, z) = (coord.x + delta[0], coord.y + delta[1], coord.z + delta[2]);
            if x < min.x - 2
                || x > max.x + 2
                || y < min.y - 2
                || y > max.y + 2
                || z < min.z - 2
                || z > max.z + 2
            {
                continue; // ignore cells to far outside but leave enough space to crawl around
            }
            let n = Voxel::new(
                coord.x + delta[0] as i16,
                coord.y + delta[1] as i16,
                coord.z + delta[2] as i16,
            );

            stack.push(n);
        }
        outside.insert(coord);
    }

    // println!("Outside: {:?}", outside);
    // let coord = Voxel::new(2,2,5);
    // println!("Outside {} Inside {}", outside.contains(&coord), map.contains(&coord));

    outside
}

fn find_bounds(map: &BTreeSet<Voxel>) -> (Voxel, Voxel) {
    let mut min = map.iter().next().expect("at least one voxel").clone();
    // if there is no voxel panic
    let mut max = min.clone();
    for v in map {
        if min.x > v.x {
            min = Voxel::new(v.x, min.y, min.z);
        }
        if min.y > v.y {
            min = Voxel::new(min.x, v.y, min.z);
        }
        if min.z > v.z {
            min = Voxel::new(min.x, min.y, v.z);
        }

        if max.x < v.x {
            max = Voxel::new(v.x, max.y, max.z);
        }
        if max.y < v.y {
            max = Voxel::new(max.x, v.y, max.z);
        }
        if max.z < v.z {
            max = Voxel::new(max.x, max.y, v.z);
        }
    }

    println!("Min: {:?} Max: {:?}", min, max);
    (min, max)
}

pub fn aoc_2022_18_a(input: &str) -> usize {
    count_sides(input, 1)
}

pub fn aoc_2022_18_b(input: &str) -> usize {
    // only outside faces count.
    // raycast all 3 directions along the axis = 21³ = 9261 ray counting in and out
    // or from all voxel all 6 directions ~ 2200 * 6 = 13200 rays till first hit. ✔️ reuses first solution
    count_sides(input, 64)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_18_a_example() {
        assert_eq!(super::aoc_2022_18_a(TEST_INPUT), 64);
    }

    #[test]
    fn aoc_2022_18_a() {
        assert_eq!(super::aoc_2022_18_a(super::INPUT), 4482);
    }

    #[test]
    fn aoc_2022_18_b_example() {
        assert_eq!(super::aoc_2022_18_b(TEST_INPUT), 58);
    }

    #[test]
    fn aoc_2022_18_b() {
        assert_eq!(super::aoc_2022_18_b(super::INPUT), 2576);
    }

    #[test]
    fn max_coord() {
        assert_eq!(
            include_str!("input.txt")
                .split(",")
                .filter_map(|s| s.trim().parse::<i16>().ok())
                .max(),
            Some(21)
        );
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
