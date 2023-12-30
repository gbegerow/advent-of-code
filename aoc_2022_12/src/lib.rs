use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use std::string::ParseError;
use priority_queue::PriorityQueue;

// #[allow(dead_code)]


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}



#[derive(Debug, Clone)]
pub struct Grid {
    values: Vec<char>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

impl Grid {
    pub fn get(&self, p:&Point) -> Option<char> {
        let is_in_bounds = p.x < self.width && p.y < self.height;
        // if !is_in_bounds {println!("Not in bounds {} {}x{}", p, self.width, self.height);}
        is_in_bounds.then(|| {
            let c = self.values[(p.x + p.y * self.width) as usize].clone();
            match c {  // Start is same as a and End same as z
                'S' => 'a',
                'E' => 'z',
                _ => c,
            }
        })
    }

    pub fn set(&mut self, p:&Point, value: char) {
        let is_in_bounds = p.x < self.width && p.y < self.height;
        is_in_bounds.then(|| self.values[(p.x + p.y * self.width) as usize] = value);
    }

    // Manhattan distance
    fn distance(&self, from: &Point, to: &Point) -> usize{
        from.x.abs_diff(to.x) + from.y.abs_diff(to.y) // + self.height_distance(from, to)
    }

    // fn height_distance(&self, from: &Point, to: &Point) -> usize{
    //      match (self.get(from), self.get(to)) {
    //         (Some(f), Some(t)) =>  (f as usize).abs_diff(t as usize),
    //         _ =>  usize::MAX, // panic!("Point outside of bounds"),
    //      }
    // }

    

    pub fn get_connected_neighbours(&self, p:&Point) -> Vec<Point>{
        self.get_relative_positions(p, Self::get_adjacent_axis_positions())
            .into_iter()
            // 1 lower or higher
            // .filter(|n| self.height_distance(p, n) < 2)
            // only 1 higher not lower
            // .filter(|n| match (self.get(p), self.get(n)) {
            //     (Some(f), Some(t)) => f <= t && (f as usize).abs_diff(t as usize) < 2,
            //     _ =>  false
            //  })
            // only 1 higher or any lower 
            // "(This also means that the elevation of the destination square 
            //    can be much lower than the elevation of your current square.)""
            .filter(|n| match (self.get(p), self.get(n)) {
                (Some(f), Some(t)) => f as usize +1 >= t as usize,
                _ =>  false
             })
            .collect()
    }

    pub fn get_adjacent_axis_positions() -> Vec<(isize, isize)> {
        vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
    }

    pub fn get_relative_positions(
        &self,
        p : &Point,
        neighbours: Vec<(isize, isize)>,
    ) -> Vec<Point> {
        let mut ret = Vec::with_capacity(neighbours.len());
        for (dx, dy) in neighbours {
            if dx.is_negative() && dx.wrapping_abs() as usize > p.x
                || dy.is_negative() && dy.wrapping_abs() as usize > p.y
                || dx.is_positive() && dx.wrapping_abs() as usize + p.x > self.width
                || dy.is_positive() && dy.wrapping_abs() as usize + p.y > self.height
            {
                continue; // out of bounds
            }

            // adding usize and isize is a little bit complicated ;-)
            let px = if dx.is_negative() {
                p.x - dx.wrapping_abs() as usize
            } else {
                p.x + dx.wrapping_abs() as usize
            };

            let py = if dy.is_negative() {
                p.y - dy.wrapping_abs() as usize
            } else {
                p.y + dy.wrapping_abs() as usize
            };

            ret.push(Point {x:px, y:py});
        }

        // println!("{}, {} -> {:?}", x, y, ret);
        ret
    }

    fn reconstruct_path(came_from: HashMap<Point, Point>, start: Point) -> Vec<Point> {
        let mut total_path = vec![start.clone()];
        // println!("Came_from {:?}", came_from);
        
        let mut current = start.clone();
        while let Some(c)= came_from.get(&current){
            total_path.push(c.clone());
            current = *c;

            if total_path.len() > came_from.len(){
                panic!( "cycle in came_from at {:?}", total_path);
            }   
        }

        total_path.reverse();
        total_path
    }

    // A* with Manhattan distance as heuristic. Implementation of https://en.wikipedia.org/wiki/A*_search_algorithm
    pub fn a_star(&mut self, start:&Point) -> Option<Vec<Point>>
    {
        let capacity = self.width*self.height;
        
        // The set of discovered nodes that may need to be (re-)expanded.
        // Initially, only the start node is known.
        let mut open = PriorityQueue::new();
        open.push(start.clone(), 1);

         // For node n, cameFrom[n] is the node immediately preceding it on the cheapest path from start
        // to n currently known.
        let mut came_from = HashMap::with_capacity(capacity);
        // For node n, gScore[n] is the cost of the cheapest path from start to n currently known.
        let mut g_score:HashMap<Point, usize> = HashMap::with_capacity(capacity);
        g_score.insert(start.clone(), 0);
        // For node n, fScore[n] := gScore[n] + h(n). fScore[n] represents our current best guess as to
        // how cheap a path could be from start to finish if it goes through n.
        let mut f_score:HashMap<Point, usize> = HashMap::with_capacity(capacity);
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

    pub fn scan_for_lowest(&self) -> Vec<Point>{
        // could exclude some positions early for optimization but I don't care
        let mut positions = Vec::new();    
        for (i,c) in self.values.iter().enumerate() {
            match c {
                'S' | 'a' => positions.push( Point { x:i % self.width, y:i / self.width }),
                _ => ()
            }
        }
        positions
    }
}  

impl FromStr for Grid{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.trim().lines().next().unwrap().trim().chars().count() as usize;
        let height = s.trim().lines().count() as usize;

        let values: Vec<char> = s
            .lines()
            .flat_map(|x| x.trim().chars())
            .collect();

        let mut start = Point { x:0, y:0};     
        let mut end = Point { x:0, y:0};     
        for (i,c) in values.iter().enumerate() {
            match c {
                'S' => start = Point { x:i % width, y:i / width },
                'E' => end = Point { x:i % width, y:i / width },
                _ => ()
            }
        }     

        Ok(Grid {
            values,
            width,
            height,
            start,
            end,
        })
        
    }
}


pub fn aoc_2022_12_a(input: &str) -> usize {
    let mut grid :Grid = input.parse().expect("invalid grid");
    let start = grid.start.clone();
    if let Some(path) = grid.a_star(&start){
        // println!("Path {:?}", path);
        path.len()-1 // start does not count
    } else {
        panic!("No path found")
    }
}

pub fn aoc_2022_12_b(input: &str) -> usize {
    let mut shortest = usize::MAX;
    let mut grid :Grid = input.parse().expect("invalid grid");
    
    for p in grid.scan_for_lowest() {  
        // in this case it would have been much more optimal to use Dijstra instead of A*
        // as it would have calculated all shortest paths in one go
        // but still fast enough that I don't care about it
        if let Some(path) = grid.a_star(&p){
            if shortest > path.len() {
                shortest = path.len();
            }
        }
    }
    shortest - 1
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_12_a_example() {
        assert_eq!(super::aoc_2022_12_a(TEST_INPUT), 31);
    }

    #[test]
    fn aoc_2022_12_a() {
       assert_eq!(super::aoc_2022_12_a(include_str!("input.txt")), 534);
    }
    
    #[test]
    fn aoc_2022_12_b_example() {
        assert_eq!(super::aoc_2022_12_b(TEST_INPUT), 29);
    }

    #[test]
    fn aoc_2022_12_b() {
        assert_eq!(super::aoc_2022_12_b(include_str!("input.txt")), 525);
    }

    const TEST_INPUT: &str = "
    Sabqponm
    abcryxxl
    accszExk
    acctuvwj
    abdefghi";
}



