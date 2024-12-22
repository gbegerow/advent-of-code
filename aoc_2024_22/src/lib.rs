// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/22
    Solution idea:

*/

const RNG_MOD: u32 = 16777216; // 2^24
const RNG_RANG: u32 = RNG_MOD - 1;
const CYCLES: u32 = 2000;

struct AocRng {
    #[allow(dead_code)]
    seed: u32,
    current: u32,
}

impl AocRng {
    fn new(seed: u32) -> Self {
        Self {
            seed,
            current: seed,
        }
    }
}
impl Iterator for AocRng {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current = (self.current ^ (self.current << 6)) & RNG_RANG; // * 64
        self.current = (self.current ^ (self.current >> 5)) & RNG_RANG; // / 32
        self.current = (self.current ^ (self.current << 11)) & RNG_RANG; // * 2048

        Some(self.current)
    }
}

fn cycle(seed: u32, cycles: u32) -> u32 {
    let mut rng = AocRng::new(seed);

    for _ in 0..cycles - 1 {
        rng.next();
    }
    rng.next().unwrap()
}

#[tracing::instrument]
pub fn aoc_2024_22_a(input: &str) -> u64 {
    let x = input
        .trim()
        .lines()
        .flat_map(|l| l.parse::<u32>())
        .map(|seed| cycle(seed, CYCLES) as u64)
        // .inspect(|c| println!(" [{}] [{:024b}] ", c, c))
        .sum();
    x
}

#[tracing::instrument]
pub fn aoc_2024_22_b(_input: &str) -> u32 {
    0
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
    fn rng_should(#[case] seed: u32, #[case] expected: u32) {
        let mut rng = super::AocRng::new(seed);
        assert_eq!(rng.next().unwrap(), expected);
    }

    #[rstest]
    #[case(1, 8685429)]
    #[case(10, 4700978)]
    #[case(100, 15273692)]
    #[case(2024, 8667524)]
    fn cycle_should(#[case] seed: u32, #[case] exepected: u32) {
        assert_eq!(super::cycle(seed, 2000), exepected);
    }

    #[rstest]
    #[case(TEST_INPUT, 37327623)]
    fn aoc_2024_22_a_example(#[case] input: &str, #[case] exepected: u64) {
        assert_eq!(super::aoc_2024_22_a(input), exepected);
    }

    #[test]
    fn aoc_2024_22_a() {
        assert_eq!(super::aoc_2024_22_a(super::INPUT), 0);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2024_22_b_example(#[case] input: &str, #[case] exepected: u32) {
        assert_eq!(super::aoc_2024_22_b(input), exepected);
    }

    #[test]
    fn aoc_2024_22_b() {
        assert_eq!(super::aoc_2024_22_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "1
10
100
2024";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
