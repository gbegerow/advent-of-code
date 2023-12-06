// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/05
    Solution idea:
Chain of HashMaps - works with test input explodes with input
Plan B: instead of Maps, just store ranges and do a lookup function
*/


#[derive(Debug, PartialEq, Eq)]
struct LookupRange {
    src: u64,
    dest: u64,
    len: u64,
}

impl LookupRange {
    fn new(definition: &str) -> Self {
        let d: Vec<_> = definition
            .trim()
            .split_whitespace()
            .flat_map(|s| s.parse())
            .collect();

        Self {
            src: d[1],
            dest: d[0],
            len: d[2],
        }
    }

    fn lookup(&self, k: Option<u64>) -> Option<u64> {
        if let Some(key) = k {
            if self.src <= key && key < self.src + self.len {
                return Some(self.dest + key - self.src);
            }
        }
        None
    }
}

#[derive(Debug, PartialEq, Eq)]
struct NamedMap {
    name: String,
    ranges: Vec<LookupRange>,
}

impl NamedMap {
    fn new(block: &str) -> Self {
        let mut lines = block.lines();
        let name = lines
            .next()
            .expect("Header")
            .trim()
            .trim_end_matches(':')
            .to_string();
        let ranges = lines.map(|l| LookupRange::new(l)).collect();

        Self { name, ranges }
    }

    fn lookup(&self, key: Option<u64>) -> Option<u64> {
        self.ranges
            .iter()
            .map(|r| r.lookup(key))
            .reduce(Option::or)
            .flatten()
            .or(key)
    }

    // fn insert(map: &mut HashMap<u32, u32>, s: &str) {
    //     if let Some((dest_start, src_start, len)) =
    //         s.split(" ").flat_map(|m| m.parse()).collect_tuple()
    //     {
    //         Self::insert_range(map, dest_start, src_start, len);
    //     }
    // }

    // fn insert_range(map: &mut HashMap<u32, u32>, dest_start: u32, src_start: u32, len: u32) {
    //     (src_start..src_start + len)
    //         .zip(dest_start..dest_start + len)
    //         .for_each(|(src, dest)| {
    //             map.insert(src, dest);
    //         });
    // }
}

fn parse(input: &str, expand_seeds: bool) -> (Vec<u64>, Vec<NamedMap>) {
    let mut lines = input.trim().split("\n\n");
    let mut seeds: Vec<_> = lines
        .next()
        .expect("seeds")
        .trim()
        .split(" ")
        .flat_map(|s| s.parse::<u64>())
        .collect();

    if expand_seeds {
        seeds = seeds
            .chunks_exact(2)
            .flat_map(|p| p[0]..p[0] + p[1])
            .collect();
    }

    let maps: Vec<_> = lines.map(|definition| NamedMap::new(definition)).collect();

    (seeds, maps)
}

fn lookup_locations(input: &str, expand_seeds: bool) -> u64 {
    let (seeds, maps) = parse(input, expand_seeds);

    //TODO we are only interested in minimum, do not store. maybe do reverse lookups
    let locations: Vec<_> = seeds
        .iter()
        .map(|&seed| {
            maps.iter()
                .fold(Some(seed), |lookup, nmap| nmap.lookup(lookup))
        })
        .collect();

    // println!("Locations: {:?}", locations);
    locations
        .iter()
        .flat_map(|&u| u)
        .min()
        .expect("invalid maps")
}

pub fn aoc_2023_05_a(input: &str) -> u64 {
    lookup_locations(input, false)
}

pub fn aoc_2023_05_b(input: &str) -> u64 {
    lookup_locations(input, true)
}

#[cfg(test)]
mod tests {
    use crate::{LookupRange, NamedMap};
    use rstest::rstest;

    #[test]
    fn aoc_2023_05_a_example() {
        assert_eq!(super::aoc_2023_05_a(TEST_INPUT), 35);
    }

    #[test]
    fn aoc_2023_05_a() {
        assert_eq!(super::aoc_2023_05_a(INPUT), 199602917);
    }

    #[test]
    fn aoc_2023_05_b_example() {
        assert_eq!(super::aoc_2023_05_b(TEST_INPUT),46);
    }

    #[test]
    fn aoc_2023_05_b() {
        assert_eq!(super::aoc_2023_05_b(INPUT), 2254686);
    }

    #[test]
    fn parse_should_expand(){
        let init = "seeds: 79 3 55 5

        seed-to-soil map:
        50 98 2";
        let (seeds, _) = super::parse(init, true);
        assert_eq!(seeds, vec![79, 80, 81, 55, 56, 57, 58, 59]);
    }

    #[rstest]
    #[case(Some(98), Some(50))]
    #[case(Some(99), Some(51))]
    #[case(Some(97), None)]
    #[case(Some(100), None)]
    fn lookup_range_should_lookup(#[case] src: Option<u64>, #[case] expected: Option<u64>) {
        let sut = LookupRange {
            src: 98,
            dest: 50,
            len: 2,
        };
        assert_eq!(sut.lookup(src), expected);
    }

    #[rstest]
    #[case(99, 51)]
    #[case(51, 53)]
    #[case(81, 81)]
    fn named_map_should_lookup(#[case] src: u64, #[case] expected: u64) {
        let init = "seed-to-soil map:
         50 98 2
         52 50 3";

        let sut = &NamedMap::new(init);
        assert_eq!(sut.lookup(Some(src)), Some(expected));
    }

    #[test]
    fn named_map_should_init() {
        let init = "seed-to-soil map:
        50 98 2
        52 50 3";

        let sut = &NamedMap::new(init);
        let expect = &NamedMap {
            name: "seed-to-soil map".to_string(),
            ranges: vec![
                LookupRange {
                    src: 98,
                    dest: 50,
                    len: 2,
                },
                LookupRange {
                    src: 50,
                    dest: 52,
                    len: 3,
                },
            ],
        };
        println!("sut:\t\t{:?}\nexepected\t{:?}", sut, expect);

        assert_eq!(sut, expect);
    }

    // #[test]
    // fn named_map_should_init() {
    //     let init = "seed-to-soil map:
    //     50 98 2
    //     52 50 3";

    //     let sut = &NamedMap::new(init);
    //     let expect = &NamedMap {
    //         name: "seed-to-soil map".to_string(),
    //         map: HashMap::from([(98, 50), (99, 51), (50, 52), (51, 53), (52, 54)]),
    //     };
    //     println!("sut:\t\t{:?}\nexepected\t{:?}", sut, expect);

    //     assert_eq!(sut, expect);
    //     // assert_eq!(sut.map.len(),expect.map.len());
    //     // sut.map.keys().for_each(|k| assert!(expect.map.contains_key(k), "{k} not expected"));
    //     // sut.map.keys().for_each(|k| assert_eq!(sut.map[k], expect.map[k]));
    // }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}
