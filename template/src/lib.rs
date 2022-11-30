pub fn part_a(input: &str) -> usize {
    for line in input.trim().lines() {
        //
    }
    0
}

pub fn part_b(input: &str) -> usize {
    for line in input.trim().lines() {
        //
    }
    0
}

fn read_input() -> &str{
    include_str!("input.txt")
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
    fn part_a() {
        assert_eq!(super::part_a(read_input()), 0);
    }

    #[test]
    fn part_b() {
        assert_eq!(super::part_b(read_input()), 0);
    }
}



