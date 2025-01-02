use std::mem::swap;

// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/13
    Solution idea:

*/
use glam::I64Vec2;
use nom::{
    branch::alt,
    character::complete::{i64, line_ending, multispace0, newline, none_of},
    combinator::{eof, map},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Machine {
    buttons: (I64Vec2, I64Vec2),
    prize: I64Vec2,
    cost: I64Vec2,
}

impl Machine {
    fn solve(&self) -> Option<I64Vec2> {
        // let gcd = self.gcd_buttons();

        // // only the trivial divisor?
        // if gcd.x == 1 || gcd.y == 1 {
        //     return None;
        // }

        // if self.prize.rem_euclid(gcd) != I64Vec2::ZERO {
        //     return None;
        // }

        // Some(self.prize / gcd)

        // just solve with Cramersche Rule and ensure integer solutions
        let (r1, r2) = self.buttons;
        let p = self.prize;

        //      (r1.x r1.y)        (a c)
        //  det (r2.x r2.y)  = det (b d) = ad-bc
        let det_a = r1.x * r2.y - (r2.x * r1.y);
        if det_a == 0 {
            return None;
        }

        // (p.x r1.y)
        // (p.y r2.y)
        let det_a1 = p.x * r2.y - (p.y * r1.y);
        // (r1.x p.x)
        // (r2.x p.y)
        let det_a2 = r1.x * p.y - (r2.x * p.x);

        // println!("A: {r1} B: {r2} P: {p} det A1 {det_a1} det A2 {det_a2} det {det_a}");

        let x = det_a1 / det_a;
        let y = det_a2 / det_a;

        // is this an integer solution to our system?
        if r1.x * x + r1.y * y != p.x {
            return None;
        }
        if r2.x * x + r2.y * y != p.y {
            return None;
        }

        Some(I64Vec2::new(x, y))
    }
}

fn parse_vec2(input: &str) -> IResult<&str, I64Vec2> {
    map(
        terminated(
            preceded(
                many1(none_of("+-0123456789")),
                separated_pair(i64, many1(none_of("+-0123456789")), i64),
            ),
            alt((line_ending, eof)),
        ),
        |(a, b)| I64Vec2::new(a, b),
    )(input)
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    map(
        tuple((parse_vec2, parse_vec2, parse_vec2)),
        |(btn_a, btn_b, prize)| Machine {
            buttons: (
                // factors of x in one vector, y in the other
                I64Vec2::new(btn_a.x, btn_b.x),
                I64Vec2::new(btn_a.y, btn_b.y),
            ), //(btn_a, btn_b),
            prize,
            cost: I64Vec2::new(3, 1),
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Machine>> {
    preceded(multispace0, separated_list1(newline, parse_machine))(input)
}

/// Greatest common divisor (https://en.wikipedia.org/wiki/Binary_GCD_algorithm)
#[allow(dead_code)]
fn gcd(mut u: i64, mut v: i64) -> i64 {
    // Base cases: gcd(n, 0) = gcd(0, n) = n
    if u == 0 {
        return v;
    } else if v == 0 {
        return u;
    }

    // Using identities 2 and 3:
    // gcd(2ⁱ u, 2ʲ v) = 2ᵏ gcd(u, v) with u, v odd and k = min(i, j)
    // 2ᵏ is the greatest power of two that divides both 2ⁱ u and 2ʲ v
    let i = u.trailing_zeros();
    u >>= i;
    let j = v.trailing_zeros();
    v >>= j;
    let k = i.min(j);

    loop {
        // u and v are odd at the start of the loop
        debug_assert!(u % 2 == 1, "u = {} should be odd", u);
        debug_assert!(v % 2 == 1, "v = {} should be odd", v);

        // Swap if necessary so u ≤ v
        if u > v {
            swap(&mut u, &mut v);
        }

        // Identity 4: gcd(u, v) = gcd(u, v-u) as u ≤ v and u, v are both odd
        v -= u;
        // v is now even

        if v == 0 {
            // Identity 1: gcd(u, 0) = u
            // The shift by k is necessary to add back the 2ᵏ factor that was removed before the loop
            return u << k;
        }

        // Identity 3: gcd(u, 2ʲ v) = gcd(u, v) as u is odd
        v >>= v.trailing_zeros();
    }
}

#[tracing::instrument]
pub fn aoc_2024_13_a(input: &str) -> i64 {
    let (_, machines) = parse(input).expect("invalid input");

    machines
        .iter()
        .flat_map(|m| m.solve().map(|r| (m, r.x * m.cost.x + r.y * m.cost.y)))
        // .inspect(|(m, cost)| println!("{m:?} {cost}"))
        .fold(0, |accu, (_m, cost)| accu + cost)
}

#[tracing::instrument]
pub fn aoc_2024_13_b(input: &str) -> i64 {
    let (_, machines) = parse(input).expect("invalid input");

    machines
        .iter()
        .map(|m| Machine {
            prize: I64Vec2::new(m.prize.x + 10000000000000, m.prize.y + 10000000000000),
            ..*m
        })
        .flat_map(|m| m.solve().map(|r| (m, r.x * m.cost.x + r.y * m.cost.y)))
        // .inspect(|(m, cost)| println!("{m:?} {cost}"))
        .fold(0, |accu, (_m, cost)| accu + cost)
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 480)]
    fn aoc_2024_13_a_example(#[case] input: &str, #[case] expected: i64) {
        assert_eq!(super::aoc_2024_13_a(input), expected);
    }

    #[test]
    fn aoc_2024_13_a() {
        assert_eq!(super::aoc_2024_13_a(super::INPUT), 36250);
    }

    // #[rstest]
    // #[case(TEST_INPUT, 0)]
    // fn aoc_2024_13_b_example(#[case] input: &str, #[case] expected: i64) {
    //     assert_eq!(super::aoc_2024_13_b(input), expected);
    // }

    #[test]
    fn aoc_2024_13_b() {
        assert_eq!(super::aoc_2024_13_b(super::INPUT), 0);
    }

    #[rstest]
    #[case([1,2,4,5], [3,6], Some(I64Vec2::new(-1,2)))]
    #[case([94,22,34,67], [8400,5400], Some(I64Vec2::new(80,40)))]
    #[case([26,67,66,21], [12748,12176], None)]
    #[case([17,84,86,37], [7870, 6450], Some(I64Vec2::new(38,86)))]
    #[case([69,27,23,71], [18641, 10279], None)]
    fn solve_should(#[case] m: [i64; 4], #[case] b: [i64; 2], #[case] expected: Option<I64Vec2>) {
        let sut = Machine {
            buttons: (I64Vec2::new(m[0], m[1]), I64Vec2::new(m[2], m[3])),
            prize: I64Vec2::new(b[0], b[1]),
            cost: I64Vec2::new(3, 1),
        };

        assert_eq!(sut.solve(), expected);
    }

    const TEST_INPUT: &str = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[allow(dead_code)]
    const TEST_INPUT_2: &str = "";
}
