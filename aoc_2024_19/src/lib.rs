use std::collections::BTreeMap;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/19
    Solution idea:
        Part a
        Wording problem in Chomsky Level 1  aka Deterministic Finite Automata aka RegEx

        Part b
        Counting possible ways. Build an NFA from the patterns.
*/
// use aoc_utils::grid::Grid;
use regex::Regex;

// get regex and list of valid words
fn parse<'a>(input: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let alphabet: Vec<_> = input
        .trim()
        .lines()
        .next()
        .expect("no line")
        .split(", ")
        .collect();

    let pattern = alphabet
        .iter()
        // .intersperse("|").collect();
        .map(|w| w.to_string())
        .collect::<Vec<_>>()
        .join("|");

    let re = Regex::new(format!("^({pattern})+$").as_str()).expect("valid wordlist?");

    let words = input
        .trim()
        .lines()
        .skip(2)
        .filter(|w| re.is_match(w.trim()))
        .collect();

    (alphabet, words)
}

/// count possibilities recursivly
fn count_possibilitiies<'a>(word: &'a str, patterns: &Vec<&'a str>) -> usize {
    fn memoized_count<'a>(
        w: &'a str,
        patterns: &Vec<&'a str>,
        cache: &mut BTreeMap<&'a str, usize>,
    ) -> usize {
        if w.len() == 0 {
            return 0;
        }
        if let Some(&n) = cache.get(w) {
            return n;
        }

        patterns
            .iter()
            .map(|pattern| {
                if w.starts_with(pattern) {
                    // strip pattern
                    let rest = &w[pattern.len()..];
                    // println!("Stripped '{pattern}' from '{w}' Rest: '{rest}'");
                    if rest.len() == 0 {
                        // we are at the end of a possible construction, count this
                        1
                    } else {
                        let c = memoized_count(rest, patterns, cache);
                        cache.insert(rest, c);
                        c
                    }
                } else {
                    0
                }
            })
            .sum()
    }

    // prime cache with patterns itself. 1 is wrong because some pattern can be composited by others
    // let mut cache = patterns.iter().map(|w| (*w, 1)).collect::<BTreeMap<_, _>>();
    // let the cache warm up on its own
    let mut cache = BTreeMap::new();
    let count = memoized_count(word, patterns, &mut cache);
    // println!("{cache:?}");

    count
}

#[tracing::instrument]
pub fn aoc_2024_19_a(input: &str) -> usize {
    let (_, words) = parse(input);

    words
        .iter()
        // .inspect(|w| println!("{w}"))
        .count()
}

#[tracing::instrument]
pub fn aoc_2024_19_b(input: &str) -> usize {
    let (pattern, words) = parse(input);

    words
        .iter()
        .map(|word| count_possibilitiies(word, &pattern))
        .sum()
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 6)]
    fn aoc_2024_19_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_19_a(input), exepected);
    }

    #[test]
    fn aoc_2024_19_a() {
        assert_eq!(super::aoc_2024_19_a(super::INPUT), 353);
    }

    #[rstest]
    #[case("brwrr", 2)]
    #[case("bggr", 1)]
    #[case("gbbr", 4)]
    #[case("rrbgbr", 6)]
    #[case("bwurrg", 1)]
    #[case("brgr", 2)]
    fn count_possibilitiies_should(#[case] input: &str, #[case] exepected: usize) {
        let pattern = vec!["r", "wr", "b", "g", "bwu", "rb", "gb", "br"];

        assert_eq!(super::count_possibilitiies(input, &pattern), exepected);
    }

    #[rstest]
    #[case(TEST_INPUT, 16)]
    fn aoc_2024_19_b_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2024_19_b(input), exepected);
    }

    #[test]
    fn aoc_2024_19_b() {
        assert_eq!(super::aoc_2024_19_b(super::INPUT), 880877787214477);
    }

    const TEST_INPUT: &str = "
    r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
}
