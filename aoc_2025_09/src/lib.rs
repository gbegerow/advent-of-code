// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2025/day/09
    Solution idea:

*/
use colored::Colorize;
use glam::I64Vec2;
use std::fmt::Debug;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum TileColor {
    Red,
    Green,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Tile {
    pos: I64Vec2,
    color: TileColor,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({}, {})", self.color, self.pos.x, self.pos.y)
    }
}

#[allow(dead_code)]
impl Tile {
    fn new(x: i64, y: i64) -> Self {
        Tile {
            pos: I64Vec2::new(x, y),
            color: TileColor::Red,
        }
    }

    fn red(x: i64, y: i64) -> Self {
        Tile {
            pos: I64Vec2::new(x, y),
            color: TileColor::Red,
        }
    }

    fn green(x: i64, y: i64) -> Self {
        Tile {
            pos: I64Vec2::new(x, y),
            color: TileColor::Green,
        }
    }

    fn position(&self) -> I64Vec2 {
        self.pos
    }
}

// we should avoid changing the order of tiles, they are in polygon order

// impl Ord for Tile {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.partial_cmp(other).expect("Tile comparison failed")
//     }
// }

// impl PartialOrd for Tile {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         // color does not matter for ordering, only position
//         let s = self.position();
//         let o = other.position();

//         Some(
//             // sort by manhattan distance to origin, then by y, then by x
//             // (s.x.abs() + s.y.abs())
//             //     .cmp(&(o.x.abs() + o.y.abs()))
//             //     .then(s.y.cmp(&o.y).then(s.x.cmp(&o.x))),

//             // sort by y, then by x, we don't need manhattan distance here
//             s.y.cmp(&o.y).then(s.x.cmp(&o.x)),
//         )
//     }
// }

fn parse(input: &str) -> Vec<Tile> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().split(',').map(|s| s.parse::<i64>().unwrap()))
        .map(|mut parts| {
            let x = parts.next().unwrap();
            let y = parts.next().unwrap();
            Tile::red(x, y)
        })
        .collect::<Vec<_>>()
}

#[inline]
fn calc_area(i: Tile, j: Tile) -> i64 {
    // compute area between tiles i and j (zero based!)
    let ip = i.position();
    let jp = j.position();
    let dx = (ip.x - jp.x).abs() + 1;
    let dy = (ip.y - jp.y).abs() + 1;

    let area = dx * dy;
    // println!("Rect {:?} to  {:?} => area {}", i, j, area);
    area
}

#[allow(dead_code)]
fn get_bounds(tiles: &Vec<Tile>) -> (i64, i64, i64, i64) {
    // returns (min_x, max_x, min_y, max_y)
    tiles.iter().fold(
        (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
        |(min_x, max_x, min_y, max_y), tile| {
            let pos = tile.position();
            (
                min_x.min(pos.x),
                max_x.max(pos.x),
                min_y.min(pos.y),
                max_y.max(pos.y),
            )
        },
    )
}

/// Get the edges of the polygon defined by the red tiles
fn get_polygon_edges(tiles: &Vec<Tile>) -> Vec<(I64Vec2, I64Vec2)> {
    let vertices: Vec<I64Vec2> = tiles.iter().map(|t| t.position()).collect();
    if vertices.len() < 3 {
        return vec![];
    }

    // verticies needs to be in order y than x, assuming input tiles are given in order
    vertices
        .windows(2)
        .map(|w| (w[0], w[1]))
        // close the polygon
        .chain(std::iter::once((vertices[vertices.len() - 1], vertices[0])))
        .collect()
}

/// Ray casting algorithm: cast ray from point to infinity and count edge crossings
/// if odd number of crossings, point is inside polygon
#[allow(dead_code)]
fn point_inside_polygon(point: I64Vec2, edges: &[(I64Vec2, I64Vec2)]) -> bool {
    let mut crossings = 0;

    for &(p1, p2) in edges {
        // point is exactly on horizontal edge
        if point.y == p1.y
            && point.y == p2.y
            && point.x >= p1.x.min(p2.x)
            && point.x <= p1.x.max(p2.x)
        {
            return true; // on edge is considered inside
        } else
        // point is exactly on vertical edge
        if point.x == p1.x
            && point.x == p2.x
            && point.y >= p1.y.min(p2.y)
            && point.y <= p1.y.max(p2.y)
        {
            return true; // on edge is considered inside
        } else
        // Check if horizontal ray from point intersects edge
        if (p1.y > point.y) != (p2.y > point.y) {
            // Calculate x-coordinate of intersection
            let x_intersect = p1.x + (point.y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
            if point.x < x_intersect {
                crossings += 1;
            }
        }
    }

    // Point is inside if odd number of crossings
    crossings % 2 == 1
}

#[allow(dead_code)]
/// Check if two line segments intersect
fn segments_intersect(a1: I64Vec2, a2: I64Vec2, b1: I64Vec2, b2: I64Vec2) -> bool {
    // Check if line segment a1-a2 intersects with b1-b2
    fn ccw(a: I64Vec2, b: I64Vec2, c: I64Vec2) -> i64 {
        (c.y - a.y) * (b.x - a.x) - (b.y - a.y) * (c.x - a.x)
    }

    let ccw1 = ccw(a1, a2, b1);
    let ccw2 = ccw(a1, a2, b2);
    let ccw3 = ccw(b1, b2, a1);
    let ccw4 = ccw(b1, b2, a2);

    // General case: segments intersect if points are on opposite sides
    if ccw1.signum() != ccw2.signum() && ccw3.signum() != ccw4.signum() {
        return true;
    }

    // Special cases: check if points are collinear and overlapping
    fn on_segment(p: I64Vec2, q: I64Vec2, r: I64Vec2) -> bool {
        q.x <= p.x.max(r.x) && q.x >= p.x.min(r.x) && q.y <= p.y.max(r.y) && q.y >= p.y.min(r.y)
    }

    if ccw1 == 0 && on_segment(a1, b1, a2) {
        return true;
    }
    if ccw2 == 0 && on_segment(a1, b2, a2) {
        return true;
    }
    if ccw3 == 0 && on_segment(b1, a1, b2) {
        return true;
    }
    if ccw4 == 0 && on_segment(b1, a2, b2) {
        return true;
    }

    false
}

/// Check if two line segments cross
/// all segments are axis-aligned with at least a length of 1. we can optimize for that.
/// we can ignore collinear overlapping segments
#[inline]
#[allow(dead_code)]
fn segments_cross(a1: I64Vec2, a2: I64Vec2, b1: I64Vec2, b2: I64Vec2) -> bool {
    // case 1: a is vertical b is horizontal
    (a1.x == a2.x && b1.y == b2.y && a1.y < a2.y && a1.y < b1.y && a2.y > b1.y)
        ||
     (a1.x == a2.x && b1.y == b2.y && a2.y < a1.y && a2.y < b1.y && a1.y > b1.y)
     ||
    // case 2: a is horizontal, b is vertical
     (a1.y == a2.y && b1.x == b2.x && a1.x < a2.x && b1.x < a1.x && b2.x > a1.x)
     ||
     (a1.y == a2.y && b1.x == b2.x && a2.x < a1.x && b1.x < a2.x && b2.x > a2.x)
}

#[allow(dead_code)]
fn rect_is_inside_polygon(
    a: I64Vec2,
    b: I64Vec2,
    tiles: &Vec<Tile>,
    edges: &[(I64Vec2, I64Vec2)],
) -> bool {
    // Check that no polygon vertex is inside the rectangle
    // rectangle defined by a and b, axis-aligned, includes edges
    let (min_x, max_x, min_y, max_y) = (a.x.min(b.x), a.x.max(b.x), a.y.min(b.y), a.y.max(b.y));

    for tile in tiles {
        let pos = tile.position();
        if pos.x > min_x && pos.x < max_x && pos.y > min_y && pos.y < max_y {
            return false; // found a polygon vertex inside the rectangle
        }
    }

    // the original vertices of the polygon are for sure part of the polygon
    // the opposite corners of the rectangle might must be tested
    if point_inside_polygon(I64Vec2::new(a.x, b.y), edges) == false
        || point_inside_polygon(I64Vec2::new(b.x, a.y), edges) == false
    {
        return false;
    }

    // test middle point of rectangle to catch rectangles enclosed on 3 of the 4 edges
    let mid = I64Vec2::new((a.x + b.x) / 2, (a.y + b.y) / 2);
    if point_inside_polygon(mid, edges) == false {
        return false;
    }

    true
}

#[allow(dead_code)]
fn polygon_is_convex(tiles: &Vec<Tile>) -> bool {
    let vertices: Vec<I64Vec2> = tiles.iter().map(|t| t.position()).collect();
    if vertices.len() < 4 {
        return true; // Triangles are always convex
    }

    let mut sign = 0;
    let n = vertices.len();

    for i in 0..n {
        let dx1 = vertices[(i + 1) % n].x - vertices[i].x;
        let dy1 = vertices[(i + 1) % n].y - vertices[i].y;
        let dx2 = vertices[(i + 2) % n].x - vertices[(i + 1) % n].x;
        let dy2 = vertices[(i + 2) % n].y - vertices[(i + 1) % n].y;

        let cross_product = dx1 * dy2 - dy1 * dx2;

        if cross_product != 0 {
            let current_sign = if cross_product > 0 { 1 } else { -1 };
            if sign == 0 {
                sign = current_sign;
            } else if sign != current_sign {
                return false; // Found a change in direction, polygon is concave
            }
        }
    }

    true // No changes in direction found, polygon is convex
}

#[allow(dead_code)]
fn get_all_tiles_list(red_tiles: &Vec<Tile>) -> Vec<Tile> {
    let (min_x, max_x, min_y, max_y) = get_bounds(red_tiles);
    let edges = get_polygon_edges(red_tiles);
    let mut all_tiles: Vec<Tile> = Vec::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = I64Vec2::new(x, y);
            if red_tiles.iter().any(|t| t.position() == pos) {
                all_tiles.push(Tile::red(x, y));
            } else if point_inside_polygon(pos, &edges) {
                all_tiles.push(Tile::green(x, y));
            }
        }
    }
    all_tiles
}

#[allow(dead_code)]
fn draw(tiles: &Vec<Tile>, rect: Option<(I64Vec2, I64Vec2)>) {
    let (_, max_x, _, max_y) = get_bounds(tiles);
    let max_x = max_x + 2;

    // scale
    for x in 0..=max_x {
        print!(
            "{}",
            match x % 10 {
                0 => '|',
                4 => ':', // rest 4 or 5? we want 3 ticks in between
                _ => '\'',
            }
        );
    }
    println!();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let pos = I64Vec2::new(x, y);
            let in_rect = if let Some((a, b)) = rect {
                x >= a.x.min(b.x) && x <= a.x.max(b.x) && y >= a.y.min(b.y) && y <= a.y.max(b.y)
            } else {
                false
            };
            if let Some(tile) = tiles.iter().find(|t| t.position() == pos) {
                match (tile.color, in_rect) {
                    (TileColor::Red, false) => print!("{}", "#".red()),
                    (TileColor::Red, true) => print!("{}", "0".red()),
                    (TileColor::Green, false) => print!("{}", "X".green()),
                    (_, true) => print!("{}", "O".white()),
                }
            } else {
                if in_rect {
                    print!("{}", "O".bold().yellow().on_magenta());
                } else {
                    print!("{}", ".");
                }
            }
        }
        println!(" {:<3}", y);
    }
}

#[tracing::instrument]
pub fn aoc_2025_09_a(input: &str) -> usize {
    let red_tiles = parse(input);
    // let all_tiles = get_all_tiles_list(&red_tiles);

    // general form of cross product:  let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
    red_tiles
        .iter()
        .enumerate() // we need indices for filtering
        .flat_map(|(i_n, i)| {
            red_tiles
                .clone()
                .into_iter()
                .enumerate()
                .filter(move |(j_n, _)| *j_n > i_n) // only consider each pair once
                // .inspect( |(_, j)| {
                //     draw(&all_tiles, Some((i.position(), j.position())));
                // })
                .map(move |(_, j)| calc_area(*i, j))
        })
        .max()
        .unwrap() as usize
}

fn rect_is_inside_polygon2(
    a: I64Vec2,
    b: I64Vec2,
    edges: &[(I64Vec2, I64Vec2)],
) -> bool {
    let (min_x, max_x, min_y, max_y) = (a.x.min(b.x), a.x.max(b.x), a.y.min(b.y), a.y.max(b.y));

    // Check that all lines are outside or on the rectangle. Is this sufficient?
    edges.iter().all(|(p1, p2)| {
        // let line_on_outside = 
            (p1.x <= min_x && p2.x <= min_x) || // both points left of rectangle
            (p1.x >= max_x && p2.x >= max_x) || // both points right of rectangle
            (p1.y <= min_y && p2.y <= min_y) || // both points above rectangle
            (p1.y >= max_y && p2.y >= max_y)   // both points below rectangle
    })
}

#[tracing::instrument]
pub fn aoc_2025_09_b(input: &str) -> usize {
    // red tiles define a polygon, we must test for every rectangle between red tiles that it is fully inside the polygon
    // result is the area of the largest such rectangle.
    // Rectangles are axis-aligned. Only right angles in polygon.
    let red_tiles = parse(input);
    let edges = get_polygon_edges(&red_tiles);
    println!("Polygon edges: {:?}", edges);

    // let is_convex = polygon_is_convex(&red_tiles);
    // println!("Polygon is convex: {}", is_convex);

    //  let all_tiles = super::get_all_tiles_list(&red_tiles);
    //     draw(
    //         &all_tiles,
    //         Some((I64Vec2::new(x1, y1), I64Vec2::new(x2, y2))),
    //     );

    // calculate all areas between pairs of red tiles and store then sort descending
    // area is cheap, inside test is expensive, so only test good candidates
    let mut areas = red_tiles
        .iter()
        .enumerate()
        .flat_map(|(i_n, i)| {
            red_tiles
                .clone()
                .iter()
                .enumerate()
                .filter(move |(j_n, _)| *j_n > i_n) // only consider each pair once
                // .filter(|(_, j)| {
                //     rect_is_inside_polygon(i.position(), j.position(), &red_tiles, &edges)
                // }) // only consider rectangles fully inside polygon
                .map(move |(_, j)| (calc_area(*i, *j), (i.position(), j.position())))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    areas.sort_by_key(|(area, (_, _))| -area); // descending order

    // find the first area that is fully inside the polygon = max valid area
    areas.into_iter().find_map(|(area, (a, b))| {
        // if rect_is_inside_polygon(*a, *b, &red_tiles, &edges) {
        if rect_is_inside_polygon2(a, b, &edges) {
            Some(area as usize)
        } else {
            None
        }
    }).expect("No valid rectangle found") 
}

pub const INPUT: &str = include_str!("input.txt");

#[cfg(test)]
mod tests {
    use glam::I64Vec2;
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 50)]
    fn aoc_2025_09_a_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_09_a(input), expected);
    }

    #[test]
    fn aoc_2025_09_a() {
        assert_eq!(super::aoc_2025_09_a(super::INPUT), 4735222687);
    }

    #[rstest]
    #[case(TEST_INPUT, 24)]
    fn aoc_2025_09_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2025_09_b(input), expected);
    }

    #[test]
    fn aoc_2025_09_b() {
        // 4594510710 too high
        // 4586258880 too high
        // 1569262188
        assert_eq!(super::aoc_2025_09_b(super::INPUT), 1569262188);
    }

    #[rstest]
    #[case(TEST_INPUT, 7, 1, 11, 1, true)]
    #[case(TEST_INPUT, 11, 1, 11, 7, true)]
    #[case(TEST_INPUT, 11, 7, 9, 7, true)]
    #[case(TEST_INPUT, 9, 7, 9, 5, true)]
    #[case(TEST_INPUT, 9, 5, 2, 5, true)]
    #[case(TEST_INPUT, 2, 5, 2, 3, true)]
    #[case(TEST_INPUT, 2, 3, 7, 3, true)]
    #[case(TEST_INPUT, 7, 3, 7, 1, true)]
    #[case(TEST_INPUT, 7, 3, 11, 1, false)]
    fn edge_in_list(
        #[case] input: &str,
        #[case] x1: i64,
        #[case] y1: i64,
        #[case] x2: i64,
        #[case] y2: i64,
        #[case] expected: bool,
    ) {
        let red_tiles = super::parse(input);
        let edges = super::get_polygon_edges(&red_tiles);
        let result = edges.contains(&(I64Vec2::new(x1, y1), I64Vec2::new(x2, y2)))
            || edges.contains(&(I64Vec2::new(x2, y2), I64Vec2::new(x1, y1)));
        assert_eq!(
            result, expected,
            "Edge ({},{}) to ({},{}) in edge list",
            x1, y1, x2, y2
        );
    }

    #[rstest]
    #[case(TEST_INPUT, 7, 1, true)]
    #[case(TEST_INPUT, 7, 2, true)]
    #[case(TEST_INPUT, 8, 1, true)]
    #[case(TEST_INPUT, 8, 2, true)]
    #[case(TEST_INPUT, 10, 6, true)]
    #[case(TEST_INPUT, 10, 7, true)]
    #[case(TEST_INPUT, 6, 2, false)]
    #[case(TEST_INPUT, 11, 2, true)]
    #[case(TEST_INPUT, 11, 4, true)]
    #[case(TEST_INPUT, 11, 5, true)]
    #[case(TEST_INPUT, 11, 6, true)]
    fn test_point_is_inside_polygon(
        #[case] input: &str,
        #[case] x: i64,
        #[case] y: i64,
        #[case] expected: bool,
    ) {
        let red_tiles = super::parse(input);
        let edges = super::get_polygon_edges(&red_tiles);
        let result = super::point_inside_polygon(super::I64Vec2::new(x, y), &edges);
        assert_eq!(result, expected, "Point inside ({},{})", x, y);
    }

    #[rstest]
    #[case(TEST_INPUT, 7, 1, 11, 7, false)]
    #[case(TEST_INPUT, 9, 5, 2, 3, true)]
    #[case(TEST_INPUT, 2, 5, 11, 1, false)]
    #[case(TEST_INPUT, 2, 5, 11, 1, false)]
    #[case(TEST_INPUT, 9, 7, 11, 1, false)]
    fn test_rect_inside_polygon(
        #[case] input: &str,
        #[case] x1: i64,
        #[case] y1: i64,
        #[case] x2: i64,
        #[case] y2: i64,
        #[case] expected: bool,
    ) {
        let red_tiles = super::parse(input);
        let edges = super::get_polygon_edges(&red_tiles);
        let all_tiles = super::get_all_tiles_list(&red_tiles);
        super::draw(
            &all_tiles,
            Some((I64Vec2::new(x1, y1), I64Vec2::new(x2, y2))),
        );

        let result = super::rect_is_inside_polygon(
            I64Vec2::new(x1, y1),
            I64Vec2::new(x2, y2),
            &red_tiles,
            &edges,
        );
        assert_eq!(
            result, expected,
            "Rectangle inside ({},{}) to ({},{})",
            x1, y1, x2, y2
        );
    }

    #[test]
    fn draw_all() {
        let red_tiles = super::parse(TEST_INPUT);
        let all_tiles = super::get_all_tiles_list(&red_tiles);

        // general form of cross product:  let cross = ys.flat_map(|y| xs.clone().map(move |x| (x, y)));
        let _ = red_tiles
            .iter()
            .enumerate() // we need indices for filtering
            .flat_map(|(i_n, i)| {
                red_tiles
                    .clone()
                    .into_iter()
                    .enumerate()
                    .filter(move |(j_n, _)| *j_n > i_n) // only consider each pair once
                    .inspect(|(_, j)| {
                        println!(
                            "\nDrawing rectangle between {:?} and {:?}",
                            i.position(),
                            j.position()
                        );
                        super::draw(&all_tiles, Some((i.position(), j.position())));
                    })
            })
            .collect::<Vec<_>>();
        assert!(false, "Visual test, draw all rectangles");
    }

    #[rstest]
    #[case(TEST_INPUT)]
    #[case(crate::INPUT)]
    fn longest_edges(#[case] input: &str) {
        let red_tiles = super::parse(input);
        let edges = super::get_polygon_edges(&red_tiles);

        let mut edge_lengths = edges
            .iter()
            .map(|(a, b)| {
                let length = (a.x - b.x).abs() + (a.y - b.y).abs();
                (length, (a.clone(), b.clone()))
            })
            .collect::<Vec<_>>();
        edge_lengths.sort_by(|a, b| b.0.cmp(&a.0)); // descending order

        let top_lens = edge_lengths.iter().take(10).collect::<Vec<_>>();
        println!("Longest edges in input: {:?}", top_lens);
        assert!(false, "informational test, longest edges");
    }

    const TEST_INPUT: &str = "
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[allow(dead_code)]
    const TEST_DIAGRAM: &str = "
        01234567890123
        ..............  00
        .......#XXX#..  01
        .......X...X..  02
        ..#XXXX#...X..  03
        ..X........X..  04
        ..#XXXXXX#.X..  05
        .........X.X..  06
        .........#X#..  07
        ..............  08
  ";
}
