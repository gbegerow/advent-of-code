enum Overlapp {
    NoOverlap,
    Overlap,
    Containd
}

pub fn aoc_2022_04_a(input: &str) -> usize {
    let mut counter = 0;

    for line in input.lines() {
        if let Some(p) = line.trim().split_once(","){
            let a = to_tuple(p.0);
            let b = to_tuple(p.1);
            
            match is_overlapping(a, b) {
                 Overlapp::Containd => counter += 1,
                 _ => ()
            }
        }
    }
    counter
}

pub fn aoc_2022_04_b(input: &str) -> u32 {
    let mut counter = 0;

    for line in input.lines() {
        if let Some(p) = line.trim().split_once(","){
            let a = to_tuple(p.0);
            let b = to_tuple(p.1);
            
            match is_overlapping(a, b) {
                 Overlapp::Containd => counter += 1,
                 Overlapp::Overlap => counter += 1,
                 _ => ()
            }
        }
    }
    counter
}

fn is_overlapping(a: (u32,u32), b: (u32,u32)) -> Overlapp {
    
    if a.1 < b.0 || a.0 > b.1 {return Overlapp::NoOverlap}  
    if a.0 >= b.0 && a.1 <= b.1 {return Overlapp::Containd}
    if b.0 >= a.0 && b.1 <= a.1 {return Overlapp::Containd}
    Overlapp::Overlap 
}

fn to_tuple(s:&str) ->(u32,u32){
    match s.split("-").filter_map(|p| p.trim().parse().ok()).take(2).collect::<Vec<_>>()[..]{
        [a, b] if b >= a => (a,b),
        [a, b] if a > b => (b,a),
        _ => (0, 0)
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_04_a_example() {
        assert_eq!(super::aoc_2022_04_a(TEST_INPUT), 2);
    }

    #[test]
    fn aoc_2022_04_a() {
       assert_eq!(super::aoc_2022_04_a(include_str!("input.txt")), 573);
    }
    
    #[test]
    fn aoc_2022_04_b_example() {
        assert_eq!(super::aoc_2022_04_b(TEST_INPUT), 4);
    }

    #[test]
    fn aoc_2022_04_b() {
        assert_eq!(super::aoc_2022_04_b(include_str!("input.txt")), 867);
    }

    const TEST_INPUT: &str = "2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8";

}



