/*
    Cellular automata. Cells are hashed coordinates. Needs bounds all the time. 
    Collision detection => target buffer needs counter
    Ringbuffer of directions to test. 
    */
// #[macro_use]
// extern crate lazy_static;

use std::collections::{HashSet};
use minifb::{Window, WindowOptions, Scale,ScaleMode, Key};


const WIN_WIDTH: usize = 256;
const WIN_HEIGHT: usize = 256;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate { x: i32, y: i32 }

impl Coordinate {
    fn new(x: i32, y: i32) -> Self { Self { x, y } }
}


#[allow(dead_code)]
fn get_adjacent_positions() -> Vec<(i32, i32)>{
    vec![(1,1), (0, 1),(1,-1), (0, -1), (-1, 1), (1, 0), (-1, 0), (-1,-1)]
}

#[allow(dead_code)]
fn get_adjacent_axis_positions() -> Vec<(i32, i32)> {
    vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
}

#[derive(Debug, Clone)]
struct Map {
    coords: HashSet<Coordinate>,
    min: Coordinate,
    max: Coordinate,
}

const CELL_COLOR : u32 = 0x8f90448F;
impl Map {
    fn show(&self, window: &mut Window) {
        if !window.is_open() || window.is_key_down(Key::Escape) {return;}
        
        // translate (0, 0) to center
        let w = (self.max.x - self.min.x).abs() as i32;
        let h = (self.max.y - self.min.y).abs() as i32;
        let cx = w/2;
        let cy = h/2;
        println!("{},{} c:({},{})", w, h, cx, cy );

        let mut buffer = vec![0; WIN_WIDTH*WIN_HEIGHT];
        for c in &self.coords {
            buffer[(c.x  + cx  + (c.y + cy) * w) as usize] = CELL_COLOR;
        }

        window.update_with_buffer(&buffer, WIN_WIDTH, WIN_HEIGHT).expect("Window not available");
    }
}




pub fn aoc_2022_23_a(input: &str) -> usize {
   let map = parse(input);
    println!("{:?}", map);

   let mut window: Window = Window::new("AoC 2022 Day 23/a", WIN_WIDTH, WIN_HEIGHT, 
        WindowOptions {
            scale: Scale::X1,
            scale_mode: ScaleMode::Center,
            ..WindowOptions::default()}
    ).expect("Unable to open Window");
 // Limit to max ~60 fps update rate
 window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));


    loop {
        map.show(&mut window);
        if !window.is_open() || window.is_key_down(Key::Escape) {break;}
    }
    0
}

fn parse(input: &str) -> Map {
    let mut coords = HashSet::with_capacity(75*75);
    let mut min = Coordinate::new(i32::MAX, i32::MAX);
    let mut max = Coordinate::new(i32::MIN, i32::MIN);
    for line in input.trim().lines().enumerate()     {
        for col in line.1.trim().chars().enumerate() {
            if  col.1 == '#' {
                coords.insert(Coordinate::new(line.0 as i32, col.0 as i32));

                // check bounds     
                if min.x > col.0 as i32 { min.x = col.0 as i32 ;}
                if max.x < col.0 as i32 { max.x = col.0 as i32 ;}

                if min.y > line.0 as i32 { min.y = line.0 as i32 ;}
                if max.y < line.0 as i32 { max.y = line.0 as i32 ;}
            }
        }
    }

    Map {coords, min, max}
}

pub fn aoc_2022_23_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_23_a_example() {
        assert_eq!(super::aoc_2022_23_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_23_a() {
       assert_eq!(super::aoc_2022_23_a(include_str!("input.txt")), 0);
    }
    
    #[test]
    fn aoc_2022_23_b_example() {
        assert_eq!(super::aoc_2022_23_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_23_b() {
        assert_eq!(super::aoc_2022_23_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "
    ....#..
    ..###.#
    #...#.#
    .#...##
    #.###..
    ##.#.##
    .#..#..    
    ";
#[allow(dead_code)]
    const TEST_INPUT_SMALL: &str = "
.....
..##.
..#..
.....
..##.
.....
    ";
}



