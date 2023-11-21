/* Not enought minerals :D
    Idea: either dynamic programming or DFS/A* in state space. Score is Geodes mined.
    State is array of tuple (bot, buildtime ) resources produced at time = time - buildtime
    start from most downtime bot. What is earliest possible buildtime? 
    What is earliest possible buildtime for the next one? 
    What producers uptime can be built with the resource collected till then?
    If a resource is not blocking, don't build a producer
*/

// #[allow(dead_code)]

pub fn aoc_2021_19_a(_input: &str) -> usize {

    0
}

pub fn aoc_2021_19_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2021_19_a_example() {
        assert_eq!(super::aoc_2021_19_a(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2021_19_a() {
       assert_eq!(super::aoc_2021_19_a(include_str!("input.txt")), 0);
    }
    
    #[test]
    fn aoc_2021_19_b_example() {
        assert_eq!(super::aoc_2021_19_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2021_19_b() {
        assert_eq!(super::aoc_2021_19_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = "";
}



