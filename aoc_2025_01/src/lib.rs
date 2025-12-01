// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/01
    Solution idea:

*/
// use aoc_utils::grid::Grid;

#[tracing::instrument]
pub fn aoc_2025_01_a(input: &str) -> usize {
    let x = input
        .trim()
        .lines()
        .map(|l| {
            l.replace("L", "-")
                .replace("R", "")
                .parse::<i32>()
                .expect("not a number")
        })
        .fold((50, 0), |acc, n| {
            let sum = (acc.0 + n).rem_euclid(100);
            // println!("acc: {:?}, n: {}, sum: {}", acc, n, sum);
            (sum, if 0 == sum { acc.1 + 1 } else { acc.1 })
        })
        .1 as usize;

    x
}

#[tracing::instrument]
pub fn aoc_2025_01_b(input: &str) -> usize {
    let x = input
        .trim()
        .lines()
        .map(|l| {
            l.replace("L", "-")
                .replace("R", "")
                .parse::<i32>()
                .expect("not a number")
        })
        .fold((50, 0), |acc, n| {
            let unlimited_sum = acc.0 + n;
            let sum = (unlimited_sum).rem_euclid(100);
            // end at zero counts for 1 crossing
            //let is_zero = 0 == sum;
            // crossing the limit from positive to negative or over 100
            // starting from zero does not count as crossing
            // the count can cross zero multiple times in one step!
            // e.g. from 5 by L15 to -10 crosses zero once
            // 0 by L50 to 50 => 50 crosses zero zero times
            // 1 by L101 to -100 => 0 crosses zero once
            // 0 by L200 to -200 => 0 crosses zero twice
            // 99 by R3 to 2 crosses zero once
            let mut cross_limit = unlimited_sum.div_euclid(100).abs();
            // starting from zero does not count as crossing (counted on the last step)
            if cross_limit > 0 && acc.0 == 0 && unlimited_sum < 0{
                cross_limit -= 1;
                println!("adjusted crossing down from zero");
            }
            // we did not cross the limit but ended at zero
            if unlimited_sum <= 0 && sum == 0 {
                println!("ended at zero without crossing");
                cross_limit += 1;
            }

            println!(
                "acc: {:?}, n: {}, unlimited: {}, sum: {}, cross_limit: {}",
                acc, n, unlimited_sum, sum, cross_limit
            );

            (
                sum,
                acc.1 + cross_limit as usize
            )
        })
        .1 as usize;

    x
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 3)]
    fn aoc_2025_01_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_01_a(input), expected);
    }

    #[test]
    fn aoc_2025_01_a() {
        assert_eq!(super::aoc_2025_01_a(super::INPUT), 1152);
    }

    // dial starts at 50
    #[rstest]
    #[case(TEST_INPUT, 6)]
    #[case("L49", 0)]
    #[case("R49", 0)]
    #[case("L50", 1)]
    #[case("R50", 1)]  
    #[case("R1000", 10)]
    #[case("L150", 2)]
    #[case("R150", 2)]
    #[case("R333", 3)]
    #[case("L333", 3)]
    #[case("L383", 4)]
    fn aoc_2025_01_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_01_b(input), expected);
    }

    #[test]
    fn aoc_2025_01_b() {
        assert_eq!(super::aoc_2025_01_b(super::INPUT), 0);
    }

    #[test]
    fn print_password_method(){
        // password method 0x434C49434B
        let ascii :[u8; 5] = [0x43, 0x4C, 0x49, 0x43, 0x4B];
        let str = std::str::from_utf8(&ascii).unwrap();
        println!("password method {str}"); // CLICK
    }

   #[rstest]
    #[case(200)]
    #[case(100)]
    #[case(000)]
    #[case(-99)]
    #[case(-100)]
    #[case(-200)]
    #[case(-1000)]
    fn test_modulo_behavior(#[case] n: i32) {
        let res = n.div_euclid(100);
        println!("{} div_euclid 100 = {}", n, res);
    }

    const TEST_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
