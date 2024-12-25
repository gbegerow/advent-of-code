// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2024/day/14
    Solution idea:
        a) vector math
        b) WTH!?!?!

    Further ideas after getting the star:
    This is object detection in images
    Use convolution kernel to find edges and a high frequency fiilter to remove noise
*/
use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn parse_pos_vel(input: &str) -> IResult<&str, (IVec2, IVec2)> {
    let (rest, ((px, py), (vx, vy))) = separated_pair(
        preceded(tag("p="), separated_pair(i32, tag(","), i32)),
        space1,
        preceded(tag("v="), separated_pair(i32, tag(","), i32)),
    )(input)?;

    Ok((rest, (IVec2::new(px, py), IVec2::new(vx, vy))))
}

fn parse(input: &str) -> IResult<&str, Vec<(IVec2, IVec2)>> {
    separated_list1(newline, parse_pos_vel)(input)
}

/// wrap a number between (0,0) and bounds (think modulo in both directions)
fn wrap(n: i32, max: i32) -> i32 {
    // % is remainder not modulus like in arithmetic which is different for negative numbers
    let rem = n % max;
    if rem < 0 {
        max + rem
    } else {
        rem
    }
}
/// wrap a vector between (0,0) and bounds (think modulo in both directions)
fn wrap_ivec2(n: IVec2, bounds: IVec2) -> IVec2 {
    IVec2::new(wrap(n.x, bounds.x), wrap(n.y, bounds.y))
}

fn qudrant(p: IVec2, bounds: IVec2) -> Option<i32> {
    let half = bounds / 2;
    // on the middle?
    if p.x == half.x || p.y == half.y {
        return None;
    }

    // 00  10  =>  0 1
    // 01  11      2 3
    let q = if p.x < half.x { 0 } else { 1 } + if p.y < half.y { 0 } else { 2 };

    // println!("p: {p} q: {q} bounds: {bounds} half: {half}");
    Some(q)
}

fn get_quadrant_count(p: &Vec<IVec2>, bounds: IVec2) -> [i32; 4] {
    let mut quadrants = [0, 0, 0, 0];
    for q in p.iter().flat_map(|p| qudrant(*p, bounds)) {
        quadrants[q as usize] += 1;
    }
    quadrants
}

#[allow(dead_code)]
fn get_bounding_box(p: &Vec<IVec2>) -> (IVec2, IVec2) {
    p.iter()
        .fold((IVec2::new(1000, 1000), IVec2::ZERO), |a, p| {
            (p.min(a.0), p.max(a.1))
        })
}
#[allow(dead_code)]
fn get_center_of_mass(p: &Vec<IVec2>) -> IVec2 {
    p.iter().fold(IVec2::ZERO, |a, p| a + p) / (p.len() as i32)
}

fn get_at_step(bots: &Vec<(IVec2, IVec2)>, step: i32, bounds: IVec2) -> Vec<IVec2> {
    bots.iter()
        // without IVec2 this would just be the same but per dimension
        .map(|(p, v)| wrap_ivec2(p + v * step, bounds))
        .collect::<Vec<_>>()
}

fn draw(p: &Vec<IVec2>) {
    let bb = get_bounding_box(p);

    for y in bb.0.y..bb.1.y {
        for x in bb.0.x..bb.1.x {
            print!(
                "{}",
                if p.contains(&IVec2::new(x, y)) {
                    '#'
                } else {
                    '.'
                }
            );
        }
        println!("");
    }
    println!("");
}

#[tracing::instrument]
pub fn aoc_2024_14_a(input: &str, bounds: IVec2) -> i32 {
    let t = 100;
    let (_, bots) = parse(input).expect("invalid input");

    let pos_after_n_steps = get_at_step(&bots, t, bounds);

    // println!(
    //     "After {t} s: {:?}",
    //     bots.iter()
    //         .zip(pos_after_n_steps.iter())
    //         .collect::<Vec<_>>()
    // );

    let quadrants = get_quadrant_count(&pos_after_n_steps, bounds);
    println!("{:?}", quadrants);

    quadrants.iter().product()
}

#[tracing::instrument]
pub fn aoc_2024_14_b(input: &str, bounds: IVec2) -> i32 {
    // just simulate and test for a chrstmas tree...
    // is the tree in the center or in one of the quadrants?
    // 500 particle in about 100x100 grid arranged is not a huge image
    // every bot/ most bots must be near a center of gravity of image
    // image probably inside bounding box with major axis 40 or less
    // and an aspact ratio between 1:1 to 1:2
    // and center of gravity near center of bounding box
    // dimensions are a little bit on the high side for termviz
    // bevy viz is better choice

    let (_, bots) = parse(input).expect("invalid input");
    // let len3 = bots.len() as i32 / 3;

    let mut safeties = Vec::new();
    for t in 0..bounds.x * bounds.y {
        let state = get_at_step(&bots, t, bounds);

        // maybe it is in one quadrant
        let quadrants = get_quadrant_count(&state, bounds);
        let safety_factor: i32 = quadrants.iter().product();

        safeties.push((t, safety_factor));

        // if there are significant more in one quadrant
        // if *quadrants.iter().max().unwrap() > len3 {

        //     println!("{t}: {quadrants:?} {safety_factor}");
        //     draw(&state);
        // }
    }

    safeties.sort_by_key(|(_, sf)| *sf);

    // show min and max state
    let state = get_at_step(&bots, safeties[0].0, bounds);
    println!("Min: {:?}", safeties[0]);
    draw(&state);

    let last = safeties.len() - 1;
    let state = get_at_step(&bots, safeties[last].0, bounds);
    println!("Max: {:?}", safeties[last]);
    draw(&state);

    // try minimum
    // safeties[0].0

    // safety is wrong metric but found visualy by inspecting all with a large concentration in one quadrant
    // s. find_unusual
    7584
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use super::*;
    use glam::IVec2;
    use rstest::rstest;
    use std::time::{Duration, Instant};

    #[rstest]
    #[case(0, 2, 4)]
    #[case(1, 4, 1)]
    #[case(2, 6, 5)]
    #[case(3, 8, 2)]
    fn bot_should_wander(#[case] t: i32, #[case] x: i32, #[case] y: i32) {
        let bounds = IVec2::new(11, 7);
        let p = IVec2::new(2, 4);
        let v = IVec2::new(2, -3);

        let np = super::wrap_ivec2(p + v * t, bounds);
        assert_eq!(np, IVec2::new(x, y))
    }

    #[rstest]
    #[case(TEST_INPUT, IVec2::new(11, 7), 12)]
    fn aoc_2024_14_a_example(#[case] input: &str, #[case] bounds: IVec2, #[case] expected: i32) {
        assert_eq!(super::aoc_2024_14_a(input, bounds), expected);
    }

    #[test]
    fn aoc_2024_14_a() {
        assert_eq!(
            super::aoc_2024_14_a(super::INPUT, IVec2::new(101, 103)),
            236628054
        );
    }

    #[test]
    fn how_many_in_small_box() {
        // probably a small image, check if everything is inside of a small bounding box
        // Result: not a single hit in i32 range, so probably wrong metric

        let bounds = IVec2::new(101, 103);
        let mut in_box = 0;
        let (_, bots) = parse(super::INPUT).expect("invalid input");

        // let x = (20..30).map(|i| IVec2::new(i, i)).collect::<Vec<_>>();
        // let b = get_bounding_box(x);
        // println!("{b:?}");

        let mut t = 10;
        let now = Instant::now();
        let duration = Duration::from_secs(300);
        while now.elapsed() < duration {
            t += 1;
            let state = get_at_step(&bots, t, bounds);
            let bb = get_bounding_box(&state);

            let w = bb.1.x - bb.0.x;
            let h = bb.1.y - bb.0.y;
            let r = h / w;

            if w <= 80 || h <= 80 {
                in_box += 1;
                println!("{t}: {w}x{h} ({r})");
            }
        }
        println!("t: {t} in_box: {in_box}");
        assert!(false); // assert_eq!(in_box, 0);
    }

    #[test]
    fn find_unusual() {
        let bounds = IVec2::new(101, 103);
        // let center = bounds / 2;
        let (_, bots) = parse(super::INPUT).expect("invalid input");
        let len = bots.len() as i32;

        let mut found = 0;
        let mut t = 10;
        let now = Instant::now();
        let duration = Duration::from_secs(30);
        while now.elapsed() < duration && found < 30 {
            t += 1;
            let state = get_at_step(&bots, t, bounds);

            // let center_of_mass = get_center_of_mass(&state);

            // let dist = center_of_mass.distance_squared(center);
            // if dist > 30 {
            //     found += 1;
            //     println!("{t}: {center_of_mass} {dist}");
            //     draw(&state);
            // }

            // maybe it is in one quadrant
            let mut quadrants = [0, 0, 0, 0];
            for q in state.iter().flat_map(|p| qudrant(*p, bounds)) {
                quadrants[q as usize] += 1;
            }
            if *quadrants.iter().max().unwrap() > (len / 3) {
                found += 1;
                let safety_factor: i32 = quadrants.iter().product();
                println!("{t}: {quadrants:?} {safety_factor}");
                draw(&state);
            }
        }
        println!("t: {t} found: {found}");
        assert!(false); // assert_eq!(in_box, 0);
    }

    #[test]
    fn aoc_2024_14_b() {
        // just write a unittest for christmas tree!?!?!?!?
        assert_eq!(
            super::aoc_2024_14_b(super::INPUT, IVec2::new(101, 103)),
            7584
        );
    }

    #[test]
    fn wrap_should() {
        let pos = (0..9).map(|p| p - 4).collect::<Vec<_>>();
        let sut = pos.iter().map(|p| super::wrap(*p, 3)).collect::<Vec<_>>();

        // println!("{pos:?}\n{sut:?}");

        assert_eq!(sut, vec![2, 0, 1, 2, 0, 1, 2, 0, 1])
    }

    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
}
