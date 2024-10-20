// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/09
    Solution idea:
    Never ever unroll data in aoc unless you must or second task will kill you pretty sure
    Statemachine:

    [*] --> Read
    Read --> Read: \w; count++
    Read --> Sequence: '('; skip = 0, multiplier = 0
    Sequence --> Sequence: \d; skip = skip*10+c
    Sequence --> Multiplier: 'x'
    Multiplier --> Multiplier: \d;  multiplier = multiplier *10+c
    Multiplier --> Skip(skip): ')'; c += skip * muliplier
    Skip(n) --> Skip(n-1): \S if n > 0;
    Skip(0) --> Read
    Read --> (*): EOF

    b: Count recursively instead of skip
*/
#[derive(Debug, Clone, Copy)]
enum CompressionVersion {
    V1,
    V2,
}

#[derive(Debug, Default, Clone, Copy)]
enum State {
    #[default]
    Read,
    Sequence(usize),
    Multiplier(usize, usize),
    Skip(usize),
}

/// calulate length of decompressed sequence
fn get_decompressed_count(input: &str, version: CompressionVersion) -> usize {
    let mut state = State::default();
    let mut count = 0;

    let bytes = input.as_bytes();
    let mut cursor = 0; // current position in string to allow calling on subsequence. Seems clumsy.
    while cursor < bytes.len() {
        let c = bytes[cursor];

        // calculate next state of the state machine
        (state, count) = {
            let next_state = match (state, c) {
                (_, _) if c.is_ascii_whitespace() => state, // always skip whitespace
                (State::Read, _) if c.is_ascii_alphanumeric() => {
                    // count letters
                    count += 1;
                    state
                }
                (State::Read, b'(') => State::Sequence(0), // start compressed sequence
                (State::Sequence(n), _) if c.is_ascii_digit() => {
                    // collect number of skips
                    State::Sequence(n * 10 + (c - b'0') as usize)
                }

                (State::Sequence(n), b'x') => State::Multiplier(n, 0), // start 2nd part
                (State::Multiplier(n, m), _) if c.is_ascii_digit() => {
                    State::Multiplier(n, m * 10 + (c - b'0') as usize)
                }
                (State::Multiplier(n, m), b')') => {
                    // count decompressed characters
                    match version {
                        CompressionVersion::V1 => {
                            // skip over n characters
                            count += n * m;
                            State::Skip(n - 1)
                        }
                        CompressionVersion::V2 => {
                            // count n characters recursively
                            let sub_sequence = &input[cursor + 1..(cursor + n + 1)];
                            let sub_count = get_decompressed_count(sub_sequence, version);
                            count += sub_count * m;
                            cursor += n; // do not decompress sub sequence again
                            State::Read
                        }
                    }
                }

                (State::Skip(n), _) => {
                    if n > 0 {
                        State::Skip(n.saturating_sub(1))
                    } else {
                        // we reached the end of this sequence
                        State::Read
                    }
                }
                _ => unreachable!("Invalid input or state"),
            };
            // println!("{:?}->{:?} '{}': {}", state, next_state, c as char, count);
            (next_state, count)
        };

        cursor += 1;
    }
    count
}

pub fn aoc_2016_09_a(input: &str) -> usize {
    get_decompressed_count(input, CompressionVersion::V1)
}

pub fn aoc_2016_09_b(input: &str) -> usize {
    get_decompressed_count(input, CompressionVersion::V2)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("ADVENT", 6)]
    #[case("A(1x5)BC", 7)]
    #[case("(3x3)XYZ", 9)]
    #[case("A(2x2)BCD(2x2)EFG", 11)]
    #[case("(6x1)(1x3)A", 6)]
    #[case("X(8x2)(3x3)ABCY", 18)]
    fn aoc_2016_09_a_example(#[case] input: &str, #[case] exepected: usize) {
        println!("{}", input);
        assert_eq!(super::aoc_2016_09_a(input), exepected);
    }

    #[test]
    fn aoc_2016_09_a() {
        assert_eq!(super::aoc_2016_09_a(INPUT), 112830);
    }

    #[rstest]
    #[case("ADVENT", 6)]
    #[case("A(1x5)BC", 7)]
    #[case("(3x3)XYZ", 9)]
    #[case("A(2x2)BCD(2x2)EFG", 11)]
    #[case("(6x1)(1x3)A", 3)]
    #[case("X(8x2)(3x3)ABCY", 20)]
    #[case("(27x12)(20x12)(13x14)(7x10)(1x12)A", 241920)]
    #[case("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN", 445)]
    fn aoc_2016_09_b_example(#[case] input: &str, #[case] exepected: usize) {
        println!("{}", input);
        assert_eq!(super::aoc_2016_09_b(input), exepected);
    }

    #[test]
    fn aoc_2016_09_b() {
        assert_eq!(super::aoc_2016_09_b(INPUT), 10931789799);
    }

    const INPUT: &str = include_str!("input.txt");
}
