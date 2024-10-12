// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/02
    Keypad:
    1 2 3
    4 5 6
    7 8 9

    Solution idea:
    3x3 grid

    b
    Keypad:
    # # 1 # #
    # 2 3 4 #
    5 6 7 8 9
    # A B C #
    # # D # #
    Solution idea:
    Rotated Grid? Just a normal grid with walls #

*/

pub fn aoc_2016_02_a(input: &str) -> String {
    fn move_by(pos: (i32, i32), inc: (i32, i32)) -> (i32, i32) {
        ((pos.0 + inc.0).clamp(0, 2), (pos.1 + inc.1).clamp(0, 2))
    }

    let mut codes = "".to_string();
    // move on 3x3 grid (if possible). On linebreak add to code.
    input.chars().fold((1, 1), |pos, c| match c {
        'U' => move_by(pos, (-1, 0)),
        'D' => move_by(pos, (1, 0)),
        'L' => move_by(pos, (0, -1)),
        'R' => move_by(pos, (0, 1)),
        '\n' => {
            println!("{codes} {pos:?}");
            // calculate key from coordinates and append to codes
            let keypad_num = pos.0 * 3 + pos.1 + 1;
            codes.push(char::from_digit(keypad_num as u32, 10).unwrap());
            pos
        }
        _ => pos,
    });

    codes
}

pub fn aoc_2016_02_b(input: &str) -> String {
    let grid_def = "
        # # 1 # #
        # 2 3 4 #
        5 6 7 8 9
        # A B C #
        # # D # # 
        "
    .replace(" ", "")
    .replace("\n", "");
    let keypad_x: &[u8] = grid_def.as_bytes();
    println!("{keypad_x:?}");
    fn move_by(keypad: &[u8], pos: (i32, i32), inc: (i32, i32)) -> (i32, i32) {
        let next = ((pos.0 + inc.0).clamp(0, 4), (pos.1 + inc.1).clamp(0, 4));

        // only move if valid
        if get(keypad, next).is_some() {
            next
        } else {
            pos
        }
    }

    fn get(keypad: &[u8], pos: (i32, i32)) -> Option<char> {
        let index = (pos.0 * 5 + pos.1) as usize;
        let num = keypad[index];
        let c = num as char;
        println!("{pos:?} {index}  -> {num} {c}");
        if c.is_alphanumeric() {
            Some(c)
        } else {
            None
        }
    }

    let mut codes = "".to_string();
    // move on grid (if possible). Start is still on "5" (2,0). On linebreak add to code.
    input.chars().fold((2, 0), |pos, c| match c {
        'U' => move_by(keypad_x, pos, (-1, 0)),
        'D' => move_by(keypad_x, pos, (1, 0)),
        'L' => move_by(keypad_x, pos, (0, -1)),
        'R' => move_by(keypad_x, pos, (0, 1)),
        '\n' => {
            println!("{codes} {pos:?}");
            // calculate key from coordinates and append to codes
            codes.push(get(keypad_x, pos).expect("pos should be valid on keypad"));
            pos
        }
        _ => pos,
    });

    codes
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(
        "ULL
        RRDDD
        LURDL
        UUUUD
        ",
        "1985"
    )]
    fn aoc_2016_02_a_example(#[case] input: &str, #[case] exepected: &str) {
        assert_eq!(super::aoc_2016_02_a(input), exepected);
    }

    #[test]
    fn aoc_2016_02_a() {
        assert_eq!(super::aoc_2016_02_a(INPUT), "48584");
    }

    #[rstest]
    #[case(
        "ULL
        RRDDD
        LURDL
        UUUUD
        ",
        "5DB3"
    )]
    fn aoc_2016_02_b_example(#[case] input: &str, #[case] exepected: &str) {
        assert_eq!(super::aoc_2016_02_b(input), exepected);
    }

    #[test]
    fn aoc_2016_02_b() {
        assert_eq!(super::aoc_2016_02_b(INPUT), "");
    }

    const INPUT: &str = include_str!("input.txt");
}
