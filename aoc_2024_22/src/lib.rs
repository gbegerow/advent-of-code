// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/22
    Solution idea:

*/

use std::collections::{HashMap, HashSet, VecDeque};
// use ahash::{AHashMap, AHashSet};

const RNG_MOD: i64 = 16777216; // 2^24
const RNG_RANG: i64 = RNG_MOD - 1;
const CYCLES: i64 = 2000;

struct AocRng {
    #[allow(dead_code)]
    seed: i64,
    current: i64,
}

impl AocRng {
    fn new(seed: i64) -> Self {
        Self {
            seed,
            current: seed,
        }
    }
}
impl Iterator for AocRng {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = (self.current ^ (self.current << 6)) & RNG_RANG; // * 64
        self.current = (self.current ^ (self.current >> 5)) & RNG_RANG; // / 32
        self.current = (self.current ^ (self.current << 11)) & RNG_RANG; // * 2048

        Some(self.current)
    }
}

fn cycle(seed: i64, cycles: i64) -> i64 {
    let mut rng = AocRng::new(seed);

    for _ in 0..cycles - 1 {
        rng.next();
    }
    rng.next().unwrap()
}

#[inline(always)]
fn price(secret: i64) -> i64 {
    secret % 10
}

struct RngDiff {
    rng: AocRng,
    price: i64,
}

impl Iterator for RngDiff {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let c = price(self.rng.current);
        let n = price(self.rng.next().unwrap());
        let d = n - c;

        self.price = n;
        Some(d)
    }
}

impl RngDiff {
    fn new(seed: i64) -> Self {
        Self {
            rng: AocRng::new(seed),
            price: price(seed),
        }
    }
}

fn price_by_sequence(seed: i64, sequence: &[i64; 4]) -> i64 {
    let generator = &mut RngDiff::new(seed);

    let mut i = 4;
    let mut window = VecDeque::from_iter(generator.take(4));

    while i < CYCLES {
        i += 1;
        if window == sequence {
            return generator.price;
        }

        window.pop_front();
        window.push_back(generator.next().unwrap());
    }
    0
}

fn sequence(i: i64) -> [i64; 4] {
    // treat number as base 19
    const BASE: i64 = 19;
    const OFFSET: i64 = 9;
    [
        ((i / (BASE * BASE * BASE)) % BASE) - OFFSET,
        ((i / (BASE * BASE)) % BASE) - OFFSET,
        ((i / (BASE)) % BASE) - OFFSET,
        ((i) % BASE) - OFFSET,
    ]
}

#[tracing::instrument]
pub fn aoc_2024_22_a(input: &str) -> i64 {
    let x = input
        .trim()
        .lines()
        .flat_map(|l| l.parse::<i64>())
        .map(|seed| cycle(seed, CYCLES) as i64)
        // .inspect(|c| println!(" [{}] [{:024b}] ", c, c))
        .sum();
    x
}

pub fn brute_force_all_sequences(seeds: Vec<i64>) -> i64 {
    // alternative: brute force by generating the sequences.
    // seqences go from [-9,-9,-9,-9] to [9,9,9,9] => 0 .. 20^4 = 160_000 tries
    let mut max = i64::MIN;
    let mut max_seq: [i64; 4] = [-9, -9, -9, -9];
    let base = 19; // 19, not 20
    for i in 0..base * base * base * base {
        let sequence = sequence(i);

        let p = seeds
            .iter()
            .map(|seed| price_by_sequence(*seed, &sequence))
            .sum();

        if p > max {
            max = p;
            max_seq = sequence;
        }
    }
    println!("{max} {max_seq:?}");
    max
}

pub fn record(seeds: Vec<i64>) -> i64 {
    // record every first occurence of a sequence and sum the price per sequence in a HashMap
    let mut record = HashMap::with_capacity(seeds.len() * CYCLES as usize); // can't have more different sequences than possible sequences

    for seed in seeds {
        let mut seen = HashSet::with_capacity(CYCLES as usize);
        let generator = &mut RngDiff::new(seed);

        let mut i = 4;
        let mut window = VecDeque::from_iter(generator.take(4));

        while i < CYCLES {
            // we always stop at first encounter
            // can we break if we see a sequence the second time?
            if seen.contains(&window) {
                continue;
            }

            // would it be better to convert the sequence to a number first? (inverse of sequence)
            // probably less memory but more runtime. Benchmark it.
            seen.insert(window.clone());
            record
                .entry(window.clone())
                .and_modify(|sum| *sum *= generator.price)
                .or_insert(generator.price);

            i += 1;

            // is there a better way in stable rust without itertools for a sliding window?
            // would using a [i64;4] better? Benchmark it
            // window = [window[0..3], generator.next().unwrap()] or mem::copy
            // or just w[0]=w[1]; ... w[3]= generator.next
            // or window = [w[1], w[2], w[3], generator.next]
            window.pop_front();
            window.push_back(generator.next().unwrap());
        }
    }

    let Some((max_seq, max)) = record.iter().max_by_key(|x| x.1) else {
        unreachable!("No maximium!?")
    };
    println!("{max} {max_seq:?}");
    *max
}

fn parse(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .flat_map(|l| l.parse::<i64>())
        .collect::<Vec<_>>()
}

#[tracing::instrument]
pub fn aoc_2024_22_b(input: &str) -> i64 {
    // maximize sum of prices
    let seeds = parse(input);

    // find common sequences of 4. If a sequence is not present it will yield 0
    // every seed has a cycle length. Can we use that to optimize?

    // alternative 1: 1h 58 min
    // brute_force(seeds)

    // alternative 2: 1h 37 min
    record(seeds)
}

pub fn aoc_2024_22_b_all_sequences(input: &str) -> i64 {
    // maximize sum of prices
    let seeds = parse(input);

    // alternative 1: 1h 58 min
    brute_force_all_sequences(seeds)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(123, 15887950)]
    #[case(15887950, 16495136)]
    #[case(16495136, 527345)]
    #[case(527345, 704524)]
    #[case(704524, 1553684)]
    #[case(1553684, 12683156)]
    #[case(12683156, 11100544)]
    #[case(11100544, 12249484)]
    #[case(12249484, 7753432)]
    #[case(7753432, 5908254)]
    fn rng_should(#[case] seed: i64, #[case] expected: i64) {
        let mut rng = super::AocRng::new(seed);
        assert_eq!(rng.next().unwrap(), expected);
    }

    #[rstest]
    #[case(1, 8685429)]
    #[case(10, 4700978)]
    #[case(100, 15273692)]
    #[case(2024, 8667524)]
    fn cycle_should(#[case] seed: i64, #[case] expected: i64) {
        assert_eq!(super::cycle(seed, 2000), expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 37327623)]
    fn aoc_2024_22_a_example(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(super::aoc_2024_22_a(input), expected);
    }

    #[test]
    fn aoc_2024_22_a() {
        assert_eq!(super::aoc_2024_22_a(super::INPUT), 17960270302);
    }

    #[rstest]
    #[case(123, vec![-3,6,-1,-1,0,2,-2,0,-2], 2)]
    fn diff_should_yield(#[case] seed: i64, #[case] expected: Vec<i64>, #[case] price: i64) {
        let mut dif_rng = super::RngDiff::new(seed);
        let diffs = (0..expected.len())
            .flat_map(|_| dif_rng.next())
            .collect::<Vec<_>>();

        assert_eq!(diffs, expected);
        assert_eq!(dif_rng.price, price);
    }

    #[rstest]
    #[case(0, [-9,-9,-9,-9])]
    #[case(1, [-9,-9,-9,-8])]
    #[case(17, [-9,-9,-9,8])]
    #[case(18, [-9,-9,-9,9])]
    #[case(19, [-9,-9,-8,-9])]
    #[case(20, [-9,-9,-8,-8])]
    #[case((19*19*19*19)/2, [0,0,0,0])]
    fn sequence_should(#[case] input: i64, #[case] expected: [i64; 4]) {
        assert_eq!(super::sequence(input), expected);
    }

    #[rstest]
    #[case(1, 7)]
    #[case(2, 7)]
    #[case(3, 0)]
    #[case(2024, 9)]
    fn price_by_sequence_should(#[case] input: i64, #[case] expected: i64) {
        assert_eq!(super::price_by_sequence(input, &[-2, 1, -1, 3]), expected);
    }

    #[test]
    fn find_common_sequences() {
        /*

        1) Not counting the last possible change

        The test case

        2021
        5017
        19751

        should have Part 1: 18183557 and Part 2: 27 with sequence (3, 1, 4, 1).
        If it's lower than 27 that's probably because you're not
        checking the very last possible change.

        2) Not counting the first possible change.

        The test case

        5053
        10083
        11263

        should have Part 1: 8876699 and Part 2: 27 with sequence (-1, 0, -1, 8).
        If it's lower than 27 that's probably because you're not
        checking the first possible change.  */
    }

    #[rstest]
    #[case(TEST_INPUT_2, 23)]
    fn aoc_2024_22_b_example(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(super::aoc_2024_22_b(input), expected);
    }

    #[test]
    fn aoc_2024_22_b() {
        assert_eq!(super::aoc_2024_22_b(super::INPUT), 2042);
    }

    const TEST_INPUT: &str = "1
10
100
2024";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "1
2
3
2024";
}
