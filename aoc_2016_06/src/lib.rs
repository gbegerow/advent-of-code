// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/06
    Solution idea:

*/

pub fn aoc_2016_06_a(input: &str) -> String {
    let mut histogram = vec![vec![0u32; 26]; 12];
    let cols = input.trim().lines().next().unwrap().len();
    assert!(cols < histogram[0].len());

    for line in input.trim().lines() {
        for (i, c) in line.bytes().enumerate() {
            if c.is_ascii_lowercase() {
                histogram[i][c.wrapping_sub(b'a') as usize] += 1;
            }
        }
    }

    let mut pwd = String::new();
    for i in 0..cols {
        let (b, val) =
            histogram[i]
                .iter()
                .enumerate()
                .fold((0, 0u32), |(max_index, max), (index, &val)| {
                    if val > max {
                        (index, val)
                    } else {
                        (max_index, max)
                    }
                });
        let c = char::from(b as u8 + b'a');
        // println!("{c} {val}@[{b}]");
        pwd.push(c);
    }

    pwd
}

pub fn aoc_2016_06_b(input: &str) -> String {
    let mut histogram = vec![vec![0u32; 26]; 12];
    let cols = input.trim().lines().next().unwrap().len();
    assert!(cols < histogram[0].len());

    for line in input.trim().lines() {
        for (i, c) in line.bytes().enumerate() {
            if c.is_ascii_lowercase() {
                histogram[i][c.wrapping_sub(b'a') as usize] += 1;
            }
        }
    }

    let mut pwd = String::new();
    for i in 0..cols {
        let (b, val) =
            histogram[i]
                .iter()
                .enumerate()
                .fold((0, u32::MAX), |(min_index, min), (index, &val)| {
                    if val < min && val > 0 {
                        (index, val)
                    } else {
                        (min_index, min)
                    }
                });
        let c = char::from(b as u8 + b'a');
        println!("{c} {val}@[{b}]");
        pwd.push(c);
    }

    pwd
}

#[cfg(test)]
mod tests {
    // use rstest::rstest;

    #[test]
    fn aoc_2016_06_a_example() {
        assert_eq!(super::aoc_2016_06_a(TEST_INPUT), "easter");
    }

    #[test]
    fn aoc_2016_06_a() {
        assert_eq!(super::aoc_2016_06_a(INPUT), "gyvwpxaz");
    }

    #[test]
    fn aoc_2016_06_b_example() {
        assert_eq!(super::aoc_2016_06_b(TEST_INPUT), "advent");
    }

    #[test]
    fn aoc_2016_06_b() {
        assert_eq!(super::aoc_2016_06_b(INPUT), "0");
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";
}
