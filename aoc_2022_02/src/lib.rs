const ROCK:u32 = 1;
const PAPER:u32 = 2;
const SCISSOR:u32 = 3;

const WIN:u32 = 6;
const DRAW:u32 = 3;
const LOOSE:u32 = 0;

pub fn part_a(input: &str) -> u32 {
    let mut score = 0;
    for l in input.trim().lines().map(|l| l.trim().split_once(" ")) {
            score += match l {
                Some(("A","X")) => ROCK + DRAW,// rock rock draw
                Some(("B","X")) => ROCK + LOOSE,// paper rock loose
                Some(("C","X")) => ROCK + WIN,// scissor rock win
 
                Some(("A","Y")) => PAPER + WIN,// rock paper win
                Some(("B","Y")) => PAPER + DRAW,// paper paper draw
                Some(("C","Y")) => PAPER + LOOSE,// scissor paper loose
 
                Some(("A","Z")) => SCISSOR + LOOSE,// rock scissor loose
                Some(("B","Z")) => SCISSOR + WIN,// paper scissor win
                Some(("C","Z")) => SCISSOR + DRAW,// scissor scissor draw

                _ => 0,
            }
    }
    score
}

pub fn part_b(input: &str) -> u32 {
    let mut score = 0;
    for l in input.trim().lines().map(|l| l.trim().split_once(" ")) {
            score += match l {
                Some(("A","X")) => SCISSOR + LOOSE,// rock loose: scissor 
                Some(("B","X")) => ROCK + LOOSE,// paper loose: rock
                Some(("C","X")) => PAPER + LOOSE,// scissor loose: paper 
 
                Some(("A","Y")) => ROCK + DRAW,// rock draw: rock
                Some(("B","Y")) => PAPER + DRAW,// paper draw: paper
                Some(("C","Y")) => SCISSOR + DRAW,// scissor draw: scissor
 
                Some(("A","Z")) => PAPER + WIN,// rock win: paper
                Some(("B","Z")) => SCISSOR + WIN,// paper win; scissor 
                Some(("C","Z")) => ROCK + WIN,// scissor win: rock

                _ => 0,
            }
    }
    score
}



#[cfg(test)]
mod tests {
    // #[test]
    // fn example1() {
    //     assert_eq!(super::part_a("\n"), 0);
    // }

    #[test]
fn aoc_2022_02a_example() {
        // paste test input
        assert_eq!(super::part_a(TEST_INPUT), 15);
    }


    #[test]
fn aoc_2022_02a() {

        assert_eq!(super::part_a(include_str!("input.txt")), 13809);
    }

    
    #[test]
fn aoc_2022_02b_example() {
        // paste test input
        assert_eq!(super::part_b(TEST_INPUT), 12);
    }


    #[test]
fn aoc_2022_02b() {
        assert_eq!(super::part_b(include_str!("input.txt")), 12316);
    }

    const TEST_INPUT: &str = "A Y
    B X
    C Z";
}



