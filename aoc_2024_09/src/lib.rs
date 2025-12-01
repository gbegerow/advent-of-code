// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/09
    Solution idea:
    Never expand in AoC
*/
use std::{fmt::Write, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Run {
    // len
    Free(usize),
    // len, id
    File(usize, usize),
}

#[allow(dead_code)]
fn print_runs(runs: &[Run], trail: usize) {
    let sum: usize = runs
        .iter()
        .map(|r| match r {
            Run::Free(len) => *len,
            Run::File(len, _) => *len,
        })
        .sum();
    println!("Listed Blocks: {sum} Trailing {trail}");

    let mut s = String::with_capacity(runs.len() * 9);
    for (i, r) in runs.iter().enumerate() {
        s.clear();
        print!(
            "{}",
            match r {
                Run::Free(l) => &"............"[..*l],
                Run::File(l, id) => {
                    write!(s, "{}", id).unwrap();
                    s = s.repeat(*l);
                    &s[..*l]
                }
            }
        );
        if (0 == (i + 1) % 100) || i == runs.len() - 1 {
            println!();
        }
    }
}

fn parse(input: &str) -> Vec<Run> {
    let mut ids = 0usize..usize::MAX;
    input
        .trim()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let len = c.to_digit(10).expect("Digit!?!") as usize;
            match i % 2 {
                0 => Run::File(len, ids.next().unwrap()),
                1 => Run::Free(len),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>()
}

// end condition: no File after empty space
#[allow(dead_code)]
fn is_fragmented(runs: &[Run]) -> bool {
    let mut file_seen = false;
    let mut index = runs.len();

    while index > 0 {
        index -= 1;
        match runs[index] {
            Run::Free(_) if file_seen => {
                return true;
            }
            Run::File(_, _) => {
                file_seen = true;
            }
            _ => (),
        };
    }
    false
}

fn checksum(runs: &[Run]) -> usize {
    let c = runs.iter().fold((0, 0), |(pos, sum), r| match r {
        // add len, but leave sum unchanged
        Run::Free(len) => (pos + len - 1, sum),
        // for every block in run add pos * id
        Run::File(len, id) => (
            pos + len,
            (pos..pos + len)
                // .inspect(|p| println!("{}*{}={} [{} {}]", p, id, p * id, pos, len))
                .fold(sum, |accu, p| accu + p * id),
        ),
    });
    c.1
}

#[tracing::instrument]
pub fn aoc_2024_09_a(input: &str) -> usize {
    let mut runs = parse(input);
    // invariant: runs.sum(len) + trail(len) is constant
    // let start_sum: usize = runs
    //     .iter()
    //     .map(|r| match r {
    //         Run::Free(len) => *len,
    //         Run::File(len, _) => *len,
    //     })
    //     .sum();
    // let mut trail = 0;

    // println!("{runs:?}");
    // print_runs(&runs, trail);

    // do not restart every search
    let mut last_file = runs.len() - 1;
    let mut first_free: usize = 0;

    loop {
        while let Run::Free(_) = runs[last_file] {
            if last_file == 0 {
                panic!("No file found!");
            }
            last_file -= 1;
        }
        let Run::File(file_len, id) = runs[last_file] else {
            unreachable!("file")
        };

        while let Run::File(_, _) = runs[first_free] {
            first_free += 1;
            // no empty block, we are done moving files
            if first_free >= runs.len() {
                break;
            }
        }
        // no empty block in front, we are done moving files
        if first_free > last_file || first_free >= runs.len() {
            break;
        }
        let Run::Free(free_len) = runs[first_free] else {
            unreachable!("free")
        };

        // println!("Free: @{first_free} {free_len} File: @{last_file} {file_len} #{id}");
        if file_len == 0 {
            // remove empty files?
            // println!("rm empty file");
            runs.remove(last_file);
        } else if free_len == 0 {
            // println!("rm empty space");
            runs.remove(first_free);
        } else if file_len < free_len {
            // println!("insert file in space");
            let d = free_len - file_len;
            //   shorten empty.len by len of file, add to trail
            // trail += file_len;

            // removing at the end does not change the first free
            runs.remove(last_file);

            // update empty run
            if let Some(free) = runs.get_mut(first_free) {
                *free = Run::Free(d);
            }

            //   move file in front of empty
            runs.insert(first_free, Run::File(file_len, id));
        } else if file_len == free_len {
            // println!("replace with file");
            //   add empty len to trailing space
            // trail += free_len;

            // removing at the end does not change the first free
            runs.remove(last_file);

            //   replace empty with file
            if let Some(free) = runs.get_mut(first_free) {
                *free = Run::File(file_len, id);
            }
        } else {
            // println!("partial file");
            // file does not fit in empty space
            let d = file_len - free_len;

            //   add empty len to trailing
            // trail += free_len;

            //   replace empty with file a of empty.len and id
            if let Some(free) = runs.get_mut(first_free) {
                *free = Run::File(free_len, id);
            }

            if let Some(file) = runs.get_mut(last_file) {
                *file = Run::File(d, id)
            }
        }

        // println!("{runs:?}");
        // print_runs(&runs, trail);

        // assert invariant
        // let sum: usize = runs
        //     .iter()
        //     .map(|r| match r {
        //         Run::Free(len) => *len,
        //         Run::File(len, _) => *len,
        //     })
        //     .sum();

        // println!(
        //     "{} - {} [{}] {:?}",
        //     first_free,
        //     last_file,
        //     runs.len(),
        //     &runs[first_free - 1..first_free + 3]
        // );
        // assert_eq!(sum + trail, start_sum, "sum {} tail {}", sum, trail);
    }

    checksum(&runs)
}

#[tracing::instrument]
pub fn aoc_2024_09_b(input: &str) -> usize {
    let mut runs = parse(input);

    // do not restart every search for file
    let mut last_file = runs.len() - 1;
    let mut last_id = usize::MAX;

    loop {
        // find the next file from the right
        while let Run::Free(_) = runs[last_file] {
            if last_file == 0 {
                panic!("No file found!");
            }
            last_file -= 1;
        }
        let Run::File(file_len, id) = runs[last_file] else {
            unreachable!("file")
        };

        // try to find an empty space to fit within
        // merge multiple empty spaces adjacent to each other
        let mut first_free: usize = 0;
        let mut free_len = 0;
        loop {
            first_free += 1;

            // no empty block of sufficient length found, ignore file
            if first_free >= runs.len() {
                break;
            }

            match runs[first_free] {
                Run::File(0, _) => {
                    println!(
                        "Merge ? {:?} + 0 + {:?}",
                        runs[first_free.saturating_sub(1)],
                        runs[first_free.saturating_add(1)]
                    );
                }
                Run::File(_, _) => (),
                Run::Free(len) if len < file_len => (),
                Run::Free(len) => {
                    free_len += len;
                    break;
                }
            }
        }

        // no empty block in front, we are done moving files
        if first_free > last_file || first_free >= runs.len() {
            break;
        }
        let Run::Free(free_len) = runs[first_free] else {
            unreachable!("free")
        };

        println!("Free: @{first_free} {free_len} File: @{last_file} {file_len} #{id}");
    }

    checksum(&runs)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 1928)]
    fn aoc_2024_09_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_09_a(input), expected);
    }

    #[test]
    fn aoc_2024_09_a() {
        assert_eq!(super::aoc_2024_09_a(super::INPUT), 6432869891895);
    }

    #[rstest]
    #[case(TEST_INPUT, 2858)]
    fn aoc_2024_09_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2024_09_b(input), expected);
    }

    #[test]
    fn aoc_2024_09_b() {
        assert_eq!(super::aoc_2024_09_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "2333133121414131402";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "12345";
}
