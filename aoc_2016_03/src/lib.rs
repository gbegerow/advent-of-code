// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/03
    Solution idea:

*/

pub fn aoc_2016_03_a(input: &str) -> usize {
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_whitespace()
                .flat_map(|p| p.parse::<u32>())
                .collect::<Vec<_>>()
        })
        .filter(|tri| {
            tri.len() == 3
                && tri[0] + tri[1] > tri[2]
                && tri[0] + tri[2] > tri[1]
                && tri[1] + tri[2] > tri[0]
        })
        .count()
}

pub fn aoc_2016_03_b(input: &str) -> usize {
    let tri = input
        .split_whitespace()
        .flat_map(|p| p.parse::<u32>())
        .collect::<Vec<_>>();
    let offset = tri.len() / 3;
    let col_wrap = tri.len() - 1;

    assert!(
        offset * 3 == tri.len(),
        "count of values should be dividable by 3"
    );

    (0..offset)
        .filter(|&index| {
            // 9: 3 rows à 3 elements per row
            // wrap via mod if i*9 is greater than length
            let i0 = (index * 9) % col_wrap;
            let i1 = i0 + 3;
            let i2 = i1 + 3;

            tri[i0] + tri[i1] > tri[i2]
                && tri[i0] + tri[i2] > tri[i1]
                && tri[i1] + tri[i2] > tri[i0]
        })
        .count()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("5 10 25", 0)]
    #[case("16 10 25", 1)]
    fn aoc_2016_03_a_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2016_03_a(input), exepected);
    }

    #[test]
    fn aoc_2016_03_a() {
        assert_eq!(super::aoc_2016_03_a(INPUT), 993);
    }

    #[rstest]
    #[case(
        "101 301 501
            102 302 502
            103 303 503
            201 401 601
            202 402 602
            203 403 603",
        6
    )]
    // i: 0 3 6 | 9 12 15 || 1 4 7 | 10 13 16 || 2 5 8 | 11 13 17
    // i0: 0     1*9%17      2*9%17  3*9%17     => (i*9)%(len-1)

    // #[case("101 203 501
    //         102  0 0
    //         103 0 0
    //         201 0 0
    //         202 0 0", 2)]
    fn aoc_2016_03_b_example(#[case] input: &str, #[case] exepected: usize) {
        assert_eq!(super::aoc_2016_03_b(input), exepected);
    }
    #[test]
    fn aoc_2016_03_b() {
        assert_eq!(super::aoc_2016_03_b(INPUT), 1849);
    }

    const INPUT: &str = include_str!("input.txt");
}
