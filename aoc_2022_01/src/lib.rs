pub fn part_a(input: &str) -> u32 {
    let inv = read_inventories(input);

    // let mut max= 0;
    // let mut idx = 0; 
    // for (i, s) in inv.iter().map(|v| v.iter().sum()).enumerate(){
    //     if s > max {
    //         max = s;
    //         idx = i
    //     }
    // }

    inv.iter().map(|v| v.iter().sum()).max().unwrap()
}

fn read_inventories(input: &str) -> Vec<Vec<u32>> {
    let mut inv: Vec<Vec<u32>> = Vec::new();
    let mut current = 0;
    for line in input.trim().lines().map(|s| s.trim()) {
        if line.is_empty() { current += 1; }

        if current == inv.len() {
            inv.push(Vec::new());
        }

        if let Ok(n) = line.parse() {
            inv[current].push(n);
        }
    }
    inv
}

pub fn part_b(input: &str) -> u32 {
    let inv = read_inventories(input);

    let mut sums:Vec<u32> = inv.iter().map(|v| v.iter().sum()).collect();
    sums.sort_by(|a,b| b.partial_cmp(a).unwrap());
    
    sums.iter().take(3).sum()
}



#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    const TEST_INPUT: &str = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    #[test]
    fn part_a_example() {
        // paste test input
        assert_eq!(super::part_a(TEST_INPUT), 24000);
    }


    #[test]
    fn part_a() {
        assert_eq!(super::part_a(include_str!("input.txt")), 73211);
    }

    
    #[test]
    fn part_b_example() {
        // paste test input
        assert_eq!(super::part_b(TEST_INPUT), 45000);
    }


    #[test]
    fn part_b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 213958);
    }

}



