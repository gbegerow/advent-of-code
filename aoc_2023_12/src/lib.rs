// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/12
    Solution idea:
    mask pattern
    Some possiibilies:
    - numbers define regex on mask -> create Deterministic Finite Automata from pattern.
        permutate all ? between . and # and count matches
        what kind of shortcuts are possible?
    - interpret as error correction code on binary numbers.
        Convert mask to binary number. Iterate over binary. .=0, #=1
        And Mask: ?=1 => skip if current & mask != current ???
        Or Mask: ?=0 => skip if mask | current != mask ???
        Permutate over all ? bits
        How to test for pattern?
    - brute force:
        permutate over all possible patterns. (maybe with memoization aka dynamic programming)
        How to test for pattern?
        no ? in pattern => just count # separated by . and compare with group count
                if match => return 1
                else => return 0
        more ? than sum of groups => return 0 (can't be satisfied)
        ? in pattern => fork
            - . => call recursively with one ? less but same groups
            - # => call recursively with one ? less and first number in groups one less
                   if first number in groups is 0 => call recursively with one ? remove first number from groups
                   test if a . follows the #. if not => return 0 (must be separated by .)
        sum up all results

    Dynamic programming maybe?
    Never found the right reduction. Hard to get my brain around it.

    With ? for multiple state it is an NFA not a DFA. Split token and let each new token follow another path. 
    Create states from groups.
        Dot  . => same state; # => next state; ? => split token, one same, one next state
        Group . => dead; # => next state; ? => split token, one dead, one next state
        LastOfGroup . => next state; # => dead; ? => split token, one dead, one next state
        TrailingDot . => same; # => dead; ? => split token, one dead, one same state

    Per group (Dot, n-1 times Group, LastOfGroup)*, TrailingDot
    1,3,1 => Dot, LastOfGroup, Dot, Group, Group, LastOfGroup, Dot, LastOfGroup, TrailingDot
    Last LastOfGroup and TrailingDot are accepting states.
    Num of possibilities are just the sum of all token in accepting states.
    So much easier than DP
*/

// use memoize::memoize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GroupPattern {
    groups: Vec<usize>,
}

impl GroupPattern {
    fn new(groups: Vec<usize>) -> Self {
        Self { groups }
    }

    fn is_valid(&self, report: &str) -> bool {
        let mut groups = Vec::from(self.groups.as_slice());
        let mut in_group = false;
        for c in report.chars() {
            match c {
                '#' => {
                    if groups.is_empty() || groups[0] == 0 {
                        // too many # in report
                        return false;
                    }
                    groups[0] -= 1;
                    in_group = true;
                }
                '.' | '?' if in_group => {
                    // did we just finish a group?
                    if !groups.is_empty() && groups[0] == 0 {
                        groups.remove(0);
                    } else {
                        return false; // not enough # in report for group
                    }
                    in_group = false;
                }

                _ => (),
            }
        }
        // all groups satisfied. Last group might be at end of report so it is not removed.
        groups.is_empty() || (groups[0] == 0 && groups.len() == 1)
    }
}

fn parse(input: &str) -> Vec<(&str, GroupPattern)> {
    input
        .trim()
        .lines()
        .flat_map(|l| {
            let (ps, gs) = l.trim().split_once(' ').expect("invalid input");
            // what data structure to use for the pattern? Array, binary number?
            let report = ps.trim();
            // let positions = ps
            //     .trim()
            //     .chars()
            //     .enumerate()
            //     .filter_map(|(i, c)| if c == '?' { Some(i) } else { None })
            //     .collect::<Vec<_>>();
            let groups = gs
                .split(",")
                .flat_map(|g| g.trim().parse::<usize>())
                .collect::<Vec<_>>();
            Some((report, GroupPattern::new(groups)))
        })
        .collect::<Vec<_>>()
}

// #[memoize] // memoize is not working with lifetimes?
//
fn possibilities<'a>(
    report: String,
    pattern: &'a GroupPattern,
) -> usize {
    // println!("report: {}, pattern: {:?}", report, pattern);

    // if let Some(&p) = cache.get(&report) {
    //     print!(".");
    //     return p;
    // }

    // all groups are satisfied? all remaining ? must be  . => return 1
    if pattern.is_valid(&report) {
        // println!("report: {}, groups: {:?}, wildcards: {}", report, pattern.groups, pattern.positions.len());
        return 1;
    }
    // not satisfied but no more ? => return 0
    if report.find('?').is_none() {
        return 0;
    }

    // don't overcomplicate, just use str::replacen to replace ? with . and # and count matches
    // replace first ? with . and # and call recursively with one ? less. We worry about allocations later
    // ignore leading .
    let report_dot = report
        .replacen('?', ".", 1)
        .trim_start_matches(".")
        .to_string();
    let report_hash = report
        .replacen('?', "#", 1)
        .trim_start_matches(".")
        .to_string();

    let count_possibilitiies =
        possibilities(report_dot, pattern) + possibilities(report_hash, pattern);
    // cache.insert(report, count_possibilitiies);
    count_possibilitiies
}

//NFA
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum State {
    Dot,
    Group,
    LastOfGroup,
    TrailingDot,
    // Dead,
}

fn create_nfa(pattern: &Vec<usize>) -> Vec<State> {
    let mut nfa = Vec::new();
    let mut groups = pattern.iter().peekable();
    while let Some(g) = groups.next() {
        nfa.push(State::Dot);
        for _ in 0..*g - 1 {
            nfa.push(State::Group);
        }
        nfa.push(State::LastOfGroup);
    }
    nfa.push(State::TrailingDot);
    nfa
}

fn count_matches(report: &str, nfa: &Vec<State>) -> usize {
    let len = nfa.len();
    let mut tokens = vec![0; len];
    tokens[0] = 1;

    // println!("report: {}, nfa: {:?}", report, nfa);
    for c in report.chars() {
        let mut new_tokens = vec![0; len];
        for (i, t) in tokens.iter_mut().enumerate() {
            // no tokens in this state
            if *t == 0 {
                continue;
            }
            // where to move tokens from here?
            match (nfa[i], c) {
                // Dot  . => same state; # => next state; ? => split token, one same, one next state
                // . all tokens stay in this state
                (State::Dot, '.') => new_tokens[i] += *t,
                (State::Dot, '#') => new_tokens[i + 1] += *t,
                (State::Dot, '?') => {
                    new_tokens[i] += *t;
                    new_tokens[i + 1] += *t;
                }
                // Group . => dead; # => next state; ? => split token, one dead, one next state
                (State::Group, '.') => (),
                (State::Group, '#') | (State::Group, '?') => new_tokens[i + 1] += *t,
                // LastOfGroup . => next state; # => dead; ? => split token, one dead, one next state
                (State::LastOfGroup, '.') => new_tokens[i + 1] += *t,
                (State::LastOfGroup, '#') => (),
                (State::LastOfGroup, '?') => new_tokens[i + 1] += *t,
                // TrailingDot . => same; # => dead; ? => split token, one dead, one same state
                // .? all tokens stay in this state
                (State::TrailingDot, '.') | (State::TrailingDot, '?') => new_tokens[i] += *t,
                (State::TrailingDot, '#') => (),
                _ => unreachable!(),
            }
        }
        // println!("char: {}, tokens: {:?} next: {:?}", c, tokens, new_tokens);

        tokens = new_tokens;
    }

    // sum of tokens in the accepting states
    &tokens[len - 2] + &tokens[len - 1]
}

pub fn aoc_2023_12_a(input: &'static str) -> usize {
    let condition_records = parse(input);
    condition_records
        .iter()
        .map(|(report, pattern)| {
            let p = possibilities(report.to_string(), pattern);

            // println!("report: {}, groups: {:?}, wildcards: {}, possibilities: {}", report, pattern.groups, report.chars().filter(|c| *c=='?').count(), p);
            p
        })
        .sum()
}

pub fn aoc_2023_12_a_nfa(input: &'static str) -> usize {
    let condition_records = parse(input);
    condition_records
        .iter()
        .map(|(report, pattern)| {
            let nfa = create_nfa(&pattern.groups);
            let p = count_matches(report, &nfa);
            p
        })
        .sum()
}

pub fn aoc_2023_12_b(input: &str) -> usize {
    let condition_records = parse(input)
        .iter()
        .map(|(report, pattern)| {
            let (report, pattern) = (
                (0..5).map(|_| *report).collect::<Vec<_>>().join("?"),
                GroupPattern::new(
                    (0..5)
                        .flat_map(|_| pattern.groups.clone())
                        .collect::<Vec<_>>(),
                ),
            );
            (report, pattern)
        })
        .collect::<Vec<_>>();

    condition_records
        .iter()
        .map(|(report, pattern)| {
            let nfa = create_nfa(&pattern.groups);
            let p = count_matches(report, &nfa);
            p
        })
        .sum()
}




#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn aoc_2023_12_a_example() {
        assert_eq!(super::aoc_2023_12_a(TEST_INPUT), 21);
    }

    #[test]
    fn aoc_2023_12_a() {
        // 7645 is too high
        assert_eq!(super::aoc_2023_12_a(INPUT), 7173);
    }

    #[test]
    fn aoc_2023_12_a_nfa() {
        assert_eq!(super::aoc_2023_12_a_nfa(INPUT), 7173);
    }

    #[test]
    fn aoc_2023_12_b_example() {
        assert_eq!(super::aoc_2023_12_b(TEST_INPUT), 525152);
    }

    #[test]
    fn aoc_2023_12_b() {
        assert_eq!(super::aoc_2023_12_b(INPUT), 29826669191291);
    }

    #[rstest]
    #[case("???.### 1,1,3", "???.###", vec![0usize,1,2], vec![1usize,1,3])]
    fn parse_example(
        #[case] input: &str,
        #[case] rep: &str,
        #[case] _positions: Vec<usize>,
        #[case] groups: Vec<usize>,
    ) {
        let (report, pattern) = &super::parse(input)[0];
        assert_eq!(report, &rep);
        // assert_eq!(pattern.positions, positions);
        assert_eq!(pattern.groups, groups);
    }

    #[rstest]
    #[case("#.#.###", vec![1usize,1,3], true)]
    #[case("#.#.###?.", vec![1usize,1,3], true)]
    #[case("#.#.###.?", vec![1usize,1,3], true)]
    #[case("#.#.##.", vec![1usize,1,3], false)]
    #[case("#.#.####.", vec![1usize,1,3], false)]
    #[case("#.#.####", vec![1usize,1,3], false)]
    #[case("##.#.###", vec![1usize,1,3], false)]
    fn should_be_valid(#[case] report: &str, #[case] groups: Vec<usize>, #[case] valid: bool) {
        let pattern = super::GroupPattern { groups };
        assert_eq!(pattern.is_valid(report), valid);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    #[case("??.????..??????##?? 1,3,6", 18)]
    #[case("??#??#???.#. 5,1", 2)]
    fn possibilities_example(#[case] input: &str, #[case] possibilities: usize) {
        let (report, pattern) = &super::parse(input)[0];
        assert_eq!(
            super::possibilities(report.to_string(), pattern),
            possibilities
        );
    }

    // Test NFA
    use super::State; 
    use super::State::*;
    #[rstest]
    #[case(vec![1,1,3], vec![Dot, LastOfGroup, Dot, LastOfGroup, Dot, Group, Group, LastOfGroup, TrailingDot])]
    #[case(vec![1,3,1], vec![Dot, LastOfGroup, Dot, Group, Group, LastOfGroup, Dot, LastOfGroup, TrailingDot])]
    fn create_nfa_should(#[case] groups: Vec<usize>, #[case] nfa: Vec<State>) {
        assert_eq!(super::create_nfa(&groups), nfa);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    #[case("??.????..??????##?? 1,3,6", 18)]
    #[case("??#??#???.#. 5,1", 2)]
    fn possibilities_nfa(#[case] input: &'static str, #[case] possibilities: usize) {
        assert_eq!(super::aoc_2023_12_a_nfa(input), possibilities);
    }

    // test against reference implementation from https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/12.rs for debugging my own solution
    // use std::collections::HashMap;
    // #[test]
    // fn reference() {
    //     let mut cache = HashMap::new();
    //     let mut reference_cache = HashMap::new();
    //     let condition_reports = super::parse(INPUT);

    //     condition_reports.iter().for_each(|(report, pattern)| {
    //         let p = super::possibilities(report.to_string(), pattern, &mut cache);
    //         reference_cache.clear();
    //         let pr = super::possible_ways(
    //             &mut reference_cache,
    //             report.as_bytes(),
    //             None,
    //             &pattern.groups,
    //         );

    //         if p != pr {
    //             println!(
    //                 "report: {}, groups: {:?}, possibilities: {}, reference: {}",
    //                 report, pattern.groups, p, pr
    //             );
    //         }
    //     })
    // }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "
    ???.### 1,1,3
    .??..??...?##. 1,1,3
    ?#?#?#?#?#?#?#? 1,3,1,6
    ????.#...#... 4,1,1
    ????.######..#####. 1,6,5
    ?###???????? 3,2,1";

    #[allow(dead_code)]
    const TEST_INPUT_VALID: &str = "
    #.#.### 1,1,3
    .#...#....###. 1,1,3
    .#.###.#.###### 1,3,1,6
    ####.#...#... 4,1,1
    #....######..#####. 1,6,5
    .###.##....# 3,2,1";
}
