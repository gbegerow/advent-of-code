use std::collections::HashSet;



pub fn aoc_2022_09_a(input: &str) -> usize {
    let moves = parse_moves(input);
    // println!("{:?}", moves);
    
    // let visited = do_moves_a(moves);
    let visited = do_moves(moves, 2);

    visited.len()
}

// fn do_moves_a(moves: Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
//     let mut head = (0,0);
//     let mut tail = (0,0);
//     let mut visited = HashSet::from([(0,0)]);
//     // origin is visited
//     for (mx, my) in moves {
//         // move head    
//         head.0 += mx;
//         head.1 += my; 

//         // follow with tail
//         let dx = head.0 - tail.0;
//         let dy = head.1 - tail.1;
//         if dx.abs() < 2 && dy.abs() < 2 { continue;} // touching, no need to move tail

//         //println!("{dx} {dy}");
//         // move tail in dirction of head, but only one step
//         if dx != 0 { tail.0 += dx/dx.abs(); }
//         if dy != 0 { tail.1 += dy/dy.abs(); }

//         // println!("H{:?} T{:?}", head, tail);
//         visited.insert(tail);
//     }
//     visited
// }

pub fn aoc_2022_09_b(input: &str) -> usize {
    let moves = parse_moves(input);
    
    let visited = do_moves(moves, 10);

    visited.len()
}

fn do_moves(moves: Vec<(i32, i32)>, len: usize) -> HashSet<(i32, i32)> {
    let mut snake = vec![(0,0); len];

    // head is snake[0], tail is last element
    let mut visited = HashSet::from([(0,0)]); // origin is visited

    for (mx, my) in moves {
        // move head    
        snake[0].0 += mx;
        snake[0].1 += my;
        
        for i in 1..len {
            // follow with tail
            let dx = snake[i-1].0 - snake[i].0;
            let dy = snake[i-1].1 - snake[i].1;
            if dx.abs() < 2 && dy.abs() < 2 { continue;} // touching, no need to move tail

            //println!("{dx} {dy}");
            // move tail in direction of head, but only one step
            if dx != 0 { snake[i].0 += dx/dx.abs(); }
            if dy != 0 { snake[i].1 += dy/dy.abs(); }
        }

        // println!("H{:?} T{:?}", snake[0], snake[len-1]);
        visited.insert(snake[len-1]);
    }

    visited
}

fn parse_moves(input: &str) -> Vec<(i32, i32)>{
    let mut moves = Vec::new();

    for line in input.trim().lines() {
        for (dir, count) in line.trim().split_once(" "){
            // unroll count
            for _ in 0..count.parse().expect("invalid count"){
                match dir {
                    "R" => moves.push((1, 0)),
                    "L" => moves.push((-1, 0)),
                    "U" => moves.push((0, 1)),
                    "D" => moves.push((0, -1)),
                    _ => () // ignore everything else
                }
            }
        }    
    }
    moves
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_09_a_example() {
        assert_eq!(super::aoc_2022_09_a(TEST_INPUT), 13);
    }

    #[test]
    fn aoc_2022_09_a() {
       assert_eq!(super::aoc_2022_09_a(include_str!("input.txt")), 6181);
    }
    
    #[test]
    fn aoc_2022_09_b_example() {
        assert_eq!(super::aoc_2022_09_b(TEST_INPUT), 1);
    }

    #[test]
    fn aoc_2022_09_b_example2() {
        assert_eq!(super::aoc_2022_09_b(TEST_INPUT2), 36);
    }

    #[test]
    fn aoc_2022_09_b() {
        assert_eq!(super::aoc_2022_09_b(include_str!("input.txt")), 2386);
    }

    const TEST_INPUT: &str = "R 4
    U 4
    L 3
    D 1
    R 4
    D 1
    L 5
    R 2";

    const TEST_INPUT2: &str = "R 5
    U 8
    L 8
    D 3
    R 17
    D 10
    L 25
    U 20";
}



