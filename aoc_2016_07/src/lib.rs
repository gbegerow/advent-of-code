// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/07
    Solution idea:

*/

fn support_tls(ip: &str) -> bool {
    let mut open_bracket = 0u32;
    let mut found = false;

    for win in ip.as_bytes().windows(4) {
        if win[0] == b'[' {
            open_bracket += 1;
        } else if win[0] == b']' {
            open_bracket = open_bracket.saturating_sub(1);
        } else if win[0].is_ascii_lowercase() {
            let is_abba = win[0] == win[3] && win[1] == win[2] && win[0] != win[1];

            // if we find 1 abba inside brackets, we can stop
            if is_abba && open_bracket > 0 {
                return false;
            }

            // if we found an abba, there might still follow 1 in brackets, invalidating it
            found = found || is_abba;
        }
    }
    found
}

fn support_ssl(ip: &str) -> bool {
    let mut open_bracket = 0u32;
    let mut outside = Vec::new();
    let mut inside = Vec::new();

    for win in ip.as_bytes().windows(3) {
        if win[0] == b'[' {
            open_bracket += 1;
        } else if win[0] == b']' {
            open_bracket = open_bracket.saturating_sub(1);
        } else if win[0].is_ascii_lowercase() {
            let is_aba = win[0] == win[2] && win[0] != win[1];

            if is_aba {
                if open_bracket > 0 {
                    inside.push(win);
                } else {
                    outside.push(win);
                }
            }
        }
    }

    //println!("Inside: {inside:?} Outside: {outside:?}");
    // is there any correspondng bab for a aba in the other list?
    for aba in inside {
        let bab = [aba[1], aba[0], aba[1]];
        if outside.contains(&&bab[..]) {
            return true;
        }
    }
    false
}

pub fn aoc_2016_07_a(input: &str) -> usize {
    input.trim().lines().filter(|&l| support_tls(l)).count()
}

pub fn aoc_2016_07_b(input: &str) -> usize {
    input.trim().lines().filter(|&l| support_ssl(l)).count()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[test]
    fn aoc_2016_07_a_example() {
        assert_eq!(
            super::aoc_2016_07_a(
                "
    abba[mnop]qrst
    abcd[bddb]xyyx
    aaaa[qwer]tyui
    ioxxoj[asdfgh]zxcvbn"
            ),
            2
        );
    }

    #[test]
    fn aoc_2016_07_a() {
        assert_eq!(super::aoc_2016_07_a(INPUT), 115);
    }

    /*
       aba[bab]xyz supports SSL (aba outside square brackets with corresponding bab within square brackets).
       xyx[xyx]xyx does not support SSL (xyx, but no corresponding yxy).
       aaa[kek]eke supports SSL (eke in supernet with corresponding kek in hypernet; the aaa sequence is not related, because the interior character must be different).
       zazbz[bzb]cdb supports SSL (zaz has no corresponding aza, but zbz has a corresponding bzb, even though zaz and zbz overlap).
    */
    #[rstest]
    #[case("aba[bab]xyz", true)]
    #[case("xyx[xyx]xyx", false)]
    #[case("aaa[kek]eke", true)]
    #[case("zazbz[bzb]cdb", true)]
    fn supports_ssl(#[case] ip: &str, #[case] expected: bool) {
        assert_eq!(super::support_ssl(ip), expected);
    }

    #[test]
    fn aoc_2016_07_b_example() {
        assert_eq!(
            super::aoc_2016_07_b(
                "
        aba[bab]xyz
        xyx[xyx]xyx
        aaa[kek]eke
        zazbz[bzb]cdb"
            ),
            3
        );
    }

    #[test]
    fn aoc_2016_07_b() {
        assert_eq!(super::aoc_2016_07_b(INPUT), 231);
    }

    const INPUT: &str = include_str!("input.txt");
}
