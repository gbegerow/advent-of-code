use std::fmt::format;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/05
    Solution idea:

*/

pub fn aoc_2016_05_a(door_id: &str) -> String {
    let mut pwd = String::new();
    let door_id = door_id.trim();

    for i in 0..u32::MAX {
        let hash_it = format!("{door_id}{i}");
        let hash = md5::compute(hash_it.as_bytes());

        // fast rough test on byte level
        if hash[0] != 0 || hash[1] != 0 {
            continue;
        }

        // exact test on hex formated value
        let digest = format!("{:x}", hash);
        if digest.starts_with("00000") {
            println!("{i}: {digest}");
            pwd.push(
                digest
                    .chars()
                    .nth(5)
                    .expect("Digest should be at least 6 characdter long"),
            );
        }

        if pwd.len() == 8 {
            break;
        }
    }

    pwd
}

pub fn aoc_2016_05_b(door_id: &str) -> String {
    let mut pwd = vec![b' '; 8];
    let door_id = door_id.trim();
    let mut to_insert = pwd.len();

    for i in 0..u32::MAX {
        let hash_it = format!("{door_id}{i}");
        let hash = md5::compute(hash_it.as_bytes());

        // fast rough test on byte level
        if hash[0] != 0 || hash[1] != 0 {
            continue;
        }

        // exact test on hex formated value
        let digest = format!("{:x}", hash);
        if digest.starts_with("00000") {
            // println!("{i}: {digest}");
            let db = digest.as_bytes();
            let pos = (db[5] - b'0') as usize;
            if pos > 7 || pwd[pos] != b' ' {
                continue;
            }

            pwd[pos] = db[6];
            to_insert -= 1;
            if to_insert == 0 {
                break;
            }
        }
    }
    String::from_utf8(pwd).expect("pwd should be a valid utf-8")
}

#[cfg(test)]
mod tests {
    // use rstest::rstest;

    #[test]
    fn aoc_2016_05_a_example() {
        assert_eq!(super::aoc_2016_05_a(TEST_INPUT), "18f47a30");
    }

    #[test]
    fn aoc_2016_05_a() {
        assert_eq!(super::aoc_2016_05_a(INPUT), "2414bc77");
    }

    #[test]
    fn aoc_2016_05_b_example() {
        assert_eq!(super::aoc_2016_05_b(TEST_INPUT), "05ace8e3");
    }

    #[test]
    fn aoc_2016_05_b() {
        assert_eq!(super::aoc_2016_05_b(INPUT), "437e60fc");
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "abc";
}
