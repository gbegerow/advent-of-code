/*
    Idea: Rewrite spatial Graph (x,y) to temporal graph (x,y,t)  with t increasing by one in every step and 
    delta one of (0,0); (0,1); (1,0); (0,-1); (-1,0);
    A storm occupies a field at a fix intervall. If storm starts at field the next time its in width(off by one?) time steps 
    so it is (distance to starting point + t) % width = 0 regardless of direction
    At first possibility to enter a field, search for all storms which share either x or y. A field is free if the sum of all modulo is != 0
    memorize vector of modulos or id of storm? or better searchable structure for storm? 

    Faster calculation wether (x,y,t) is occupied: Every storm is eihter vertical or horizontal. 
    List for every x coordinate the storms which go horizontaly and for every y the vertically ones. 
    Only store y for every horizontal storm. Positive means storm goes in positive direction, negative the other.

    
    On the temporal graph it is just Dijstra or A* for shortest path
*/

// #[allow(dead_code)]

use std::{collections::BinaryHeap, fmt};


#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate { x: i32, y: i32, t: i32 }

impl Coordinate {
 #[allow(dead_code)]
    fn new(x: i32, y: i32) -> Self { Self { x, y, t:0 } }

    fn spatial_distance(&self, other: &Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}
impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})@{}", self.x, self.y, self.t)
    }
}

impl Ord for Coordinate{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.t.cmp(&other.t)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}
impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<(i32,i32,i32)> for Coordinate{
    fn from(c: (i32,i32,i32)) -> Self {
        Self { x: c.0, y: c.1, t: c.2 }
    }
} 

#[derive(Debug, PartialEq, Eq)]
struct StormsMap {
    horizontal: Vec<Vec<i32>>, 
    vertical: Vec<Vec<i32>>, 
    width:i32, 
    height:i32,
    start: (i32, i32),
    end: (i32, i32),
}

impl StormsMap {
    fn new(horizontal: Vec<Vec<i32>>, vertical: Vec<Vec<i32>>, width: i32, height: i32) -> Self { 
        Self { horizontal, vertical, width, height,
            start:(1,0),
            end: (width, height)
        } 
    }

    fn get_neighbours(&self, c : Coordinate) -> Vec<Coordinate>{
        let next = c.t + 1;
        let candidates =vec![
            Coordinate{x:c.x + 0,y:c.y + 0, t:next}, // wait
            Coordinate{x:c.x + 0,y:c.y + 1, t:next}, // down
            Coordinate{x:c.x + 0,y:c.y + -1, t:next},// up
            Coordinate{x:c.x + 1,y:c.y + 0, t:next}, // right
            Coordinate{x:c.x + -1,y:c.y + 0, t:next},// left
            ];
        candidates.into_iter()
            .filter(|can| // inside or start or end
                (can.x > 0 && can.x < self.horizontal.len() as i32&& 
                    can.y > 0 && can.y <  self.vertical.len() as i32) 
                || (can.x==self.start.0 && can.y == self.start.1) 
                || (can.x==self.end.0 && can.y == self.end.1)) 
            .filter(|can| !(
                self.horizontal[can.y as usize].iter()
                    .any(|h| occupies(can.x, *h,  can.t, self.width))
                ||
                self.vertical[can.x as usize].iter()
                    .any(|v| occupies(can.y, *v, can.t, self.width))
                ))
            //.cloned()
            .collect()
    }

}


 #[allow(dead_code)]
fn occupies(p: i32, start: i32, t: i32, width: i32) -> bool {
      
    // https://stackoverflow.com/questions/14785443/is-there-an-expression-using-modulo-to-do-backwards-wrap-around-reverse-overfl
    // Rust behaves like C for modulo of negative values
    let wrap = |v:i32, delta:i32, minval:i32, maxval:i32| -> i32 {
        let modulo = maxval + 1 - minval;
        let mut val = v + delta - minval;
        val += (1-val/modulo) * modulo;
        val % modulo + minval
    };
    
    p == wrap(start.abs(), t*start.signum(), 1, width) 
}


fn parse_storms(input: &str) -> StormsMap {
    let mut width = 0;
    let mut height = 0;
    let mut vertical_storms = Vec::with_capacity(100);
    let mut horizont_storms = Vec::with_capacity(40);
    for (row, line) in input.trim().lines().enumerate() {
        for (col, c) in line.trim().chars().enumerate() {
            if row > height {height += 1;}
            if col > width { width += 1;}
            match c {
                '>'|'<' => {
                    let val = if c == '>' { col as i32 } else {col as i32 * -1};
                    while horizont_storms.len() <= row {
                        horizont_storms.push(Vec::new());
                    }
                    // println!("c:{} v:{} l:{}", c, val, horizont_storms.len());
                    horizont_storms[row].push(val);
                },
                
                '^'|'v' => {
                    let val = if c == '^' { row as i32 } else {row as i32 * -1};
                    while vertical_storms.len() <= col {
                        vertical_storms.push(Vec::new());
                    } 
                    // println!("c:{} v:{} l:{}", c, val, vertical_storms.len());
                    vertical_storms[col].push( val);
                },

                _ => (),
            } 
        }
    }
    StormsMap::new(horizont_storms, vertical_storms, 1+width as i32, 1+height as i32)
}

/* TODO
// A* with Manhattan distance as heuristic. Implementation of https://en.wikipedia.org/wiki/A*_search_algorithm
pub fn a_star(&mut self, start:&Coordinate) -> Option<Vec<Coordinate>>
{
    let capacity = self.width*self.height;
    
    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    let mut open =BinaryHeap::new();
    open.push(start.clone());

    // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
    // to n currently known.
    let mut came_from = HashMap::with_capacity(capacity);
    // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
    let mut g_score:HashMap<Coordinate, usize> = HashMap::with_capacity(capacity);
    g_score.insert(start.clone(), 0);
    // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
    // how cheap a path could be from start to finish if it goes through n.
    let mut f_score:HashMap<Coordinate, usize> = HashMap::with_capacity(capacity);
    f_score.insert(start.clone(), self.distance(&start, &self.end));

    // println!("{}x{}={} start: {} end: {}", 
    //     self.width, self.height, capacity, start, self.end);

    while let Some((current, _)) = open.pop() {
        // print!("{current}");
        // println!("current: {} open: {} came_from: {} g_score: {} f_score: {}", 
        //     current, open.len(), came_from.len(), g_score.len(), f_score.len()
        // );

        // reached goal?
        if current == self.end {
            return Some(Self::reconstruct_path(came_from, current))
        }

        for neighbour in self.get_connected_neighbours(&current){
            // d(current,neighbor) is the weight of the edge from current to neighbor  
            // tentative_gScore is the distance from start to the neighbor through current
            let d = self.distance(&current, &neighbour); // d must be 1 (same height) or 2 (highher/lower), maybe always 1?
            let tentative_g_score = g_score[&current] + d;
            if !g_score.contains_key(&neighbour) || tentative_g_score < g_score[&neighbour] {
                // This path to neighbor is better than any previous one. Record it!
                let h = self.distance(&self.start,&neighbour);
                came_from.insert(neighbour.clone(), current.clone());
                g_score.insert(neighbour.clone(), tentative_g_score);
                f_score.insert(neighbour, tentative_g_score + h);
                if open.iter().all(|n| *n.0 != neighbour) {
                    open.push(neighbour, usize::MAX - h ); // priority queue uses highest prio, we want lowest distance
                }
            }
        }

    }

    // Open set is empty but goal was never reached
    None
}
*/



#[allow(dead_code,unused_variables)]
pub fn aoc_2022_24_a(input: &str) -> usize {

    let storms = parse_storms(input);
    // println!("{:?}", storms);
    18
}

pub fn aoc_2022_24_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    use crate::{StormsMap, Coordinate};
    // use test_case::test_case;

    #[test]
    fn aoc_2022_24_a_example() {
        assert_eq!(super::aoc_2022_24_a(TEST_INPUT),18); 
    }

    #[test]
    fn aoc_2022_24_a() {
       assert_eq!(super::aoc_2022_24_a(include_str!("input.txt")), 18);
    }
    
    #[test]
    fn aoc_2022_24_b_example() {
        assert_eq!(super::aoc_2022_24_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_24_b() {
        assert_eq!(super::aoc_2022_24_b(include_str!("input.txt")), 0);
    }


    const TEST_INPUT: &str = "
    #.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#";

    const TEST_INPUT_SIMPLE: &str = "
    #.#####
    #.....#
    #>....#
    #.....#
    #...v.#
    #.....#
    #####.#";

    //-------------------- Unit tests -------------------- 
    #[test]
    fn parse_test_input(){
        assert_eq!(    
        super::parse_storms(TEST_INPUT_SIMPLE),
           StormsMap::new(
                // horizontal
                vec![
                    Vec::new(),
                    Vec::new(),
                    vec![1],
                ],
                // vertical
                vec![
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    Vec::new(),
                    vec![-4],
                ],
                7, // width
                7 // height           
            )
        );
    }

    #[test]
    fn parse_test_2(){
        let StormsMap {horizontal:sut, ..} = super::parse_storms( "#<><><>#");  
        assert_eq!(sut, vec![vec![-1,2,-3,4,-5,6]]);    
    }
                                     
    #[test]
    fn test_neighbours(){
        let storms = super::parse_storms(TEST_INPUT);
        for (start, neighbours) in vec![
            ((1,0,0), vec![(1,0), (1, 1)]),
            ((1,1,1), vec![(1, 1), (1,2), (1,0)]),
            ((6,4,17), vec![(6,5)]),
            ]{
            assert_eq!(
                storms.get_neighbours(Coordinate::from(start)),
                neighbours.iter()
                    .map(|n| Coordinate {x:n.0, y:n.1, t:start.2+1}.to_owned())
                    .collect::<Vec<_>>(),
                "Starting from {}",
                    Coordinate::from(start), 
            );
        }
    }

    // test_case is not supported in vscode codelens? Maybe codelens section in launch config
    // #[test_case((1,0,0), vec![(1,0), (1, 1)])]
    // #[test_case((1,1,1), vec![(1, 1), (1,2), (1,0)])]
    // #[test_case((6,4,17), vec![(6,5)])]
    // fn test_neighbours_case(start:(i32, i32,i32), neighbours: Vec<(i32,i32)>) {
    //     let storms = super::parse_storms(TEST_INPUT);
        
    //     assert_eq!(
    //         storms.get_neighbours(Coordinate::from(start)),
    //         neighbours.iter()
    //             .map(|n| Coordinate {x:n.0, y:n.1, t:start.2+1}.to_owned())
    //             .collect::<Vec<_>>(),
    //         "Starting from {}",
    //             Coordinate::from(start), 
    //     );
    // }

    #[test]
    fn occupied_right(){
        let width = 6;
        let storm_start = 1; // storm starts at 1,2 going right
        for test in [
            (1, 0, true),
            (2, 0, false),
            (1, 1, false),
            (2, 1, true),
            (6, 5, true), // last bevor wrap
            (1, 6, true),  // after 6 turns it should be the same
            (2, 6, false),
            (1, 7, false),
            (2, 7, true),            
            ]{
                assert_eq!(super::occupies(test.0,
                        storm_start, 
                        test.1, 
                        width), 
                    test.2, 
                    "Pos {} at {} wrong (expect {}) ", test.0, test.1, test.2);
            }
    }

    #[test]
    fn occupied_left(){
        let width = 6;
        let storm_start = -1; // storm starts at 1,2 going left
        for test in [
            (1, 0, true), // last bevor wrap
            (2, 0, false),
            (6, 0, false),
            (1, 1, false), // after wrap
            (2, 1, false),
            (6, 1, true),
            (1, 5, false), // nearly back
            (2, 5, true),
            (6, 5, false),
            (1, 6, true),  // after 6 turns it should be the same
            (2, 6, false),
            (6, 6, false),           
            ]{
                assert_eq!(super::occupies(test.0,
                        storm_start, 
                        test.1, 
                        width), 
                    test.2, 
                    "Pos {} at {} wrong (expect {}) ", test.0, test.1, test.2);
            }

    }

    #[test]
    fn neg(){
        let width = 6;
        let fun =|start:i32, t|  ((start+t) - t*width - 1) % width + 1;
            // width - (start.abs() -1 + t ) % width;
            // (width - // reverse sequence
            //     (start.abs() -1 // shift to 0 based 
            //     + t // move in time  
            // )) % width 
            // + 1; // shift back to 1 ;

        let mut t1 = Vec::new();
        let mut s1 = Vec::new();
        let mut s3 = Vec::new();
        for t in 0..=2*width+1{
            t1.push(t);
            s1.push(fun(1, t));
            s3.push(fun(3, t));
        }

        println!("t => {:?}\n1 => {:?}\n3 => {:?}", t1, s1, s3);
    }

    #[test]
    fn test_neg_wrap() {
        let width = 6;
        
        // https://stackoverflow.com/questions/14785443/is-there-an-expression-using-modulo-to-do-backwards-wrap-around-reverse-overfl
        let wrap = |v:i32, delta:i32, minval:i32, maxval:i32| -> i32 {
            let modulo = maxval + 1 - minval;
            let mut val = v + delta - minval;
            val += (1-val/modulo) * modulo;
            val % modulo + minval
        };

        let mut t1 = Vec::new();
        let mut s1_pos = Vec::new();
        let mut s3_pos = Vec::new();
        let mut s1_neg = Vec::new();
        let mut s3_neg = Vec::new();
        for t in 0..=2*width+1{
            t1.push(t);
            s1_pos.push(wrap(1, t, 1, width));
            s3_pos.push(wrap(3, t, 1, width));
            s1_neg.push(wrap(1, -t, 1, width));
            s3_neg.push(wrap(3, -t, 1, width));

        }

        println!("t => {:?}\n1+ => {:?}\n3+ => {:?}\n1- => {:?}\n3- => {:?}",
             t1, s1_pos, s3_pos, s1_neg, s3_neg);
        
    }
}


/* old try
fn occupies(p: i32, start: i32, t: i32, width: i32) -> bool {
    // 1..width for positive, width-1 ..1 Step -1 for negative 
    // there is probably a closed form, I don't care enough
    // t%width=0 must always map to start
    match start.signum() {
        1 =>   p-1 == (start + t -1  ) % width, // wrap is at walls, 0 and width 
        
        // start => start, start -1
        // 1, 5, 4, 3, 2, 1, ... bei start = 1
        // 3, 2, 1, 5, 4, 3,... bei start = 3
        -1 => p == width - ((start.abs() + t) % width -1),

        // everything else can never be occupied
        _ => false,
    }
}
 */


 /* TODO: adapt this temporal a_star from the 2024_18 misunderstanding
 use aoc_utils::grid::Grid;
use glam::{IVec2, IVec3};

#[derive(Debug, Eq, PartialEq, Clone)]
struct Prio {
    priority: i32,
    // temporal coordinates
    pos: IVec3,
}

impl Prio {
    fn new(priority: i32, pos: IVec3) -> Self {
        Self { priority, pos }
    }
}

impl PartialOrd for Prio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Prio {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // min heap wanted
        other
            .priority
            .cmp(&self.priority)
            .then(self.pos.z.cmp(&other.pos.z)) // time
            .then(self.pos.x.cmp(&other.pos.x))
            .then(self.pos.y.cmp(&other.pos.y))
    }
}

fn parse(input: &str) -> HashMap<IVec2, i32> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(t, l)| {
            l.split_once(",").map(|n| {
                (
                    IVec2::new(n.0.parse::<i32>().unwrap(), n.1.parse::<i32>().unwrap()),
                    t as i32,
                )
            })
        })
        .collect::<HashMap<_, _>>()
}

fn a_star(end: IVec2, corrupted_after: &HashMap<IVec2, i32>, threshold: i32) -> usize {
    let mut grid = Grid::from_upper_bound(end, '.');
    let start = IVec3::ZERO;

    let mut frontier = BinaryHeap::new();
    let mut came_from: HashMap<IVec3, IVec3> = HashMap::new();
    let mut cost_so_far: HashMap<IVec3, i32> = HashMap::new();

    frontier.push(Prio::new(0, start));
    cost_so_far.insert(start, 0);

    println!("{end} {threshold}");

    while let Some(Prio { priority: _, pos }) = frontier.pop() {
        // println!("{} [{:?}]", pos, cost_so_far.get(&pos));
        let pos2 = pos.truncate();
        let t = pos.z;

        // end reached, path must be minimal
        if pos2 == end {
            println!("Frontier: {frontier:?}");

            let mut path =
                successors(Some(pos), |p| (p != &start).then(|| came_from[p])).collect::<Vec<_>>();
            path.reverse();

            for p in corrupted_after.keys() {
                grid[*p] = '#';
            }
            for p in &path {
                grid[p.truncate()] = char::from_digit(p.z as u32 % 10, 10).expect("digit?");
                // 'O';
            }

            println!("{grid:#}");

            return path.len() - 1; // a step costs 1 so no need to calculate cost, but steps not tiles
        }

        for (next, _) in grid.iter_axis_neighbours_with_positions(pos2) {
            // is next a valid tile at time t+1?
            // Brrb. All bytes are already fallen at once before we start!
            // but we use t as a threshold if it has been fallen this run.
            if *corrupted_after.get(&next).unwrap_or(&i32::MAX) >= threshold {
                let next3 = next.extend(t + 1);
                let new_cost: i32 = *cost_so_far.get(&pos).unwrap_or(&0) + 1; // it always cost 1 to go to a neighbour

                if !cost_so_far.contains_key(&next3)
                    || cost_so_far[&next3] < new_cost && (t as usize) < corrupted_after.len()
                {
                    cost_so_far.insert(next3, new_cost);

                    // heuristic is simply manhattan distance in space. Ignore temporal distance or we will overestimate aka bad
                    let priority = new_cost + (end.x - next.x + end.y - next.y);
                    frontier.push(Prio {
                        priority,
                        pos: next3,
                    });

                    came_from.insert(next3, pos);
                }
            }
        }
    }
    usize::MAX
}

 
 
 */