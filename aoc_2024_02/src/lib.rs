// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/02
    Solution idea:

*/
fn all_safe(v: &Vec<i32>) -> bool {
    let sgn_should = v[0].signum(); // sign of first difference should be continued over all diffs
    v.iter().all(|d| is_safe(sgn_should, *d))
}

fn is_safe(sgn_should: i32, d: i32) -> bool {
    //println!("Sgn: {sgn_should} d: {d}");

    sgn_should == d.signum() && d.abs() > 0 && d.abs() < 4
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    // iterator window is not stable yet so we have to collect first
    input
        .trim()
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .flat_map(|s| s.parse::<i32>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_differences(reports: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    // now we can use window on slice
    reports.iter().map(|r| get_diff(r)).collect::<Vec<_>>()
}

fn get_diff(r: &Vec<i32>) -> Vec<i32> {
    r[..]
        .windows(2)
        .map(|w| match w {
            [a, b] => a - b,
            _ => unreachable!(), // or handle the case appropriately
        })
        .collect::<Vec<_>>()
}

#[tracing::instrument]
pub fn aoc_2024_02_a(input: &str) -> usize {
    let reports = parse(input);
    let diffs = get_differences(&reports);

    // println!("{:?}", diffs);
    diffs.iter().filter(|v| all_safe(v)).count()
}

#[tracing::instrument(skip(input))]
pub fn aoc_2024_02_b(input: &str) -> usize {
    let reports = parse(input);
    let diffs = get_differences(&reports);

    // nope, we can't filter the differences, we must filter the outlieers in the source

    let mut count = 0;

    for (report, diff) in reports.iter().zip(diffs.iter()) {
        if all_safe(&diff) {
            count += 1;
            continue;
        }

        // unsafe, try to remove one at a time
        // to tired, keep it simple, just copy like there is no tomorrow
        // ignore the mess, good night
        for i in 0..report.len() {
            let mut tmp_rep = report.clone();
            tmp_rep.remove(i);

            let tmp_diffs = get_diff(&tmp_rep);
            if all_safe(&tmp_diffs) {
                count += 1;
                break;
            }
        }
    }
    count
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    /*    #[rstest]
    #[case(vec![7, 6, 4, 2, 1], true)]
    #[case(vec![1, 2, 7, 8, 9], false)]
    #[case(vec![9, 7, 6, 2, 1], false)]
    #[case(vec![1, 3, 2, 4, 5], false)]
    #[case(vec![8, 6, 4, 4, 1], false)]
    #[case(vec![1, 3, 6, 7, 9], true)] */

    #[rstest]
    #[case(vec![1, 2, 2, 1], true)]
    #[case(vec![-1, -5, -1, -1], false)]
    #[case(vec![2, 1, 4, 1], false)]
    #[case(vec![-2, 1, -2, -1], false)]
    #[case(vec![2, 2, 0, 3], false)]
    #[case(vec![-2, -3, -1, -2], true)]
    fn all_safe_should(#[case] input: Vec<i32>, #[case] expected: bool) {
        assert_eq!(super::all_safe(&input), expected);
    }

    #[test]
    fn aoc_2024_02_a_example() {
        assert_eq!(super::aoc_2024_02_a(TEST_INPUT), 2);
    }

    #[test]
    fn aoc_2024_02_a() {
        assert_eq!(super::aoc_2024_02_a(super::INPUT), 660);
    }

    #[rstest]
    #[case(vec![1, 2, 2, 1], true)]
    #[case(vec![-1, -5, -1, -1], true)]
    #[case(vec![2, 1, 4, 1], false)]
    #[case(vec![-2, 1, -2, -1], false)]
    #[case(vec![2, 2, 0, 3], false)]
    #[case(vec![-2, -3, -1, -2], true)]
    fn all_but_one_safe_should(#[case] input: Vec<i32>, #[case] expected: bool) {
        assert_eq!(super::all_safe(&input), expected);
    }

    #[test]
    fn aoc_2024_02_b_example() {
        assert_eq!(super::aoc_2024_02_b(TEST_INPUT), 4);
    }

    #[test]
    fn aoc_2024_02_b() {
        assert_eq!(super::aoc_2024_02_b(super::INPUT), 689);
    }

    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
}
