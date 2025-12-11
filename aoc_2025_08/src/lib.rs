// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/08
    Solution idea:
    k-D tree for spatial queries
    Time to put one implementation into our aoc_utils
*/
// use aoc_utils::grid::Grid;
use aoc_utils::k_d_tree::IKdTree3d;

#[tracing::instrument]
pub fn aoc_2025_08_a(input: &str) -> usize {
    let points =input.trim().lines().map(|l| {
        let mut parts = l.split(',').map(|n| n.parse::<i32>().unwrap());
        let x = parts.next().unwrap();
        let y = parts.next().unwrap();
        let z = parts.next().unwrap();
        glam::IVec3::new(x,y,z)
    }).collect::<Vec<_>>();
    let tree = IKdTree3d::new(points, 16);

    
    0
}

#[tracing::instrument]
pub fn aoc_2025_08_b(_input: &str) -> usize {
    0
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2025_08_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_08_a(input), expected);
    }

    #[test]
    fn aoc_2025_08_a() {
        assert_eq!(super::aoc_2025_08_a(super::INPUT), 0);
    }

    #[rstest]
    #[case(TEST_INPUT, 0)]
    fn aoc_2025_08_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_08_b(input), expected);
    }

    #[test]
    fn aoc_2025_08_b() {
        assert_eq!(super::aoc_2025_08_b(super::INPUT), 0);
    }

    const TEST_INPUT: &str = "";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
