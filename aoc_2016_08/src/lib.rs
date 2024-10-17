// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2016/day/08
    Solution idea:

*/
use regex::Regex;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
enum RotateDir {
    #[default]
    Row,
    Column,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DisplayCommand {
    Rect(u32, u32),
    Rotate(RotateDir, u32, u32),
}

#[derive(Debug)]
struct Screen {
    width: usize,
    height: usize,
    buffer: Vec<Vec<u8>>,
}
const FULL_PIXEL: u8 = b'#';
// const EMPTY_PIXEL: u8 = b'.';

impl Display for Screen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buffer = &self.buffer[0];
        let disp = buffer
            .iter()
            .enumerate()
            .map(|(i, px)| {
                let newline = i % self.width == self.width - 1;
                let px_set = *px == FULL_PIXEL;
                match (px_set, newline) {
                    (true, false) => "#",
                    (true, true) => "#\n",
                    (false, false) => ".",
                    (false, true) => ".\n",
                }
            })
            .collect::<String>();

        // is there a way to display this without allocating a new string every time?
        write!(f, "{}", disp)
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new(50, 6)
    }
}

impl Screen {
    fn new(width: usize, height: usize) -> Self {
        let buff_size = width * height;
        Self {
            width,
            height,
            buffer: vec![vec![b'.'; buff_size], vec![0; buff_size]],
        }
    }

    /// double buffer, copy active buffer and switch
    fn switch(&mut self) {
        let (first, second) = self.buffer.split_at_mut(1);
        second[0].copy_from_slice(&first[0][..]);
    }

    fn set(&mut self, row: u32, col: u32, val: u8) {
        // write to buffer 0
        let index = (row as usize % self.height) * self.width + (col as usize % self.width);
        // println!("{row} {col} -> {index} = {val}");
        self.buffer[0][index] = val;
    }

    fn get(&self, row: u32, col: u32) -> u8 {
        // read from buffer 1
        let index = (row as usize % self.height) * self.width + (col as usize % self.width);
        self.buffer[1][index]
    }

    fn execute(&mut self, cmd: DisplayCommand) {
        match cmd {
            DisplayCommand::Rect(w, h) => {
                for row in 0..h {
                    for col in 0..w {
                        self.set(row, col, FULL_PIXEL);
                    }
                }
            }

            DisplayCommand::Rotate(rotate_dir, at, by) => {
                // set values depending on axis
                // src: start position of source, wll be incremented every step
                // dst: start posiition of destination, will be incremented every step
                // inc: increment of axis on step
                // times: how many pixels must be rotated, steps taken
                let (mut src, mut dst, inc, times) = match rotate_dir {
                    RotateDir::Row => ((at, 0), (at, by), (0, 1), self.width),

                    // rotate col x=2 by 1=> (0,1) -> (1,1); 5 by 3 -> (0,4) -> (3,4)
                    RotateDir::Column => ((0, at), (by, at), (1, 0), self.height),
                };

                for _ in 0..times {
                    self.set(dst.0, dst.1, self.get(src.0, src.1));

                    src.0 += inc.0;
                    src.1 += inc.1;
                    dst.0 += inc.0;
                    dst.1 += inc.1;
                }
            }
        }
        self.switch();
    }

    fn count(&self) -> usize {
        self.buffer[0]
            .iter()
            .filter(|&&px| px == FULL_PIXEL)
            .count()
    }
}

fn parse(input: &str) -> Vec<DisplayCommand> {
    let rect_rx = Regex::new(r"\s*rect\s+(?P<w>\d+)x(?P<h>\d+)").unwrap();
    let rotate_rx =
        Regex::new(r"\s*rotate\s+(?P<dir>row|column)\s+(x|y)=(?P<at>\d+)\s+by\s+(?P<by>\d+)")
            .unwrap();

    let mut commands = Vec::new();
    for line in input.lines() {
        println!("{line}");

        if let Some(caps) = rect_rx.captures(line) {
            let w = caps["w"].parse().expect("rect width should be numeric");
            let h = caps["h"].parse().expect("rect height should be numeric");
            commands.push(DisplayCommand::Rect(w, h));
        }

        if let Some(caps) = rotate_rx.captures(line) {
            let dir = match &caps["dir"] {
                "row" => RotateDir::Row,
                "column" => RotateDir::Column,
                _ => panic!("invalid rotation"),
            };
            let at = caps["at"].parse().expect("rotate at should be numeric");
            let by = caps["by"].parse().expect("rotate by should be numeric");
            commands.push(DisplayCommand::Rotate(dir, at, by));
        }
    }
    commands
}

pub fn aoc_2016_08_a(input: &str) -> usize {
    let commands = parse(input);
    let mut screen = Screen::default();

    // println!("{commands:?}");
    for cmd in commands {
        screen.execute(cmd);
    }
    println!("{}", screen);

    screen.count()
}

pub fn aoc_2016_08_b(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("rect 1x1", 1)]
    #[case("rect 1x2", 2)]
    #[case("rect 3x2", 6)]
    #[case(
        "rect 5x2
    rotate row y=1 by 2",
        10
    )]
    #[case(
        "rect 10x1
    rotate column x=2 by 1",
        10
    )]
    #[case(
        "rect 5x1
    rotate row y=0 by 1
    rect 5x1
    ",
        6
    )]
    fn interactive(#[case] input: &str, #[case] exepected: usize) {
        let commands = parse(input);
        let mut screen = Screen::new(10, 4);

        // println!("{commands:?}");
        for cmd in commands {
            println!("{cmd:?}");
            screen.execute(cmd);
        }
        println!("┌┬┬┬┬┬┬┬┬┐");
        println!("{}", screen);

        assert_eq!(screen.count(), exepected);
        // panic!("look at output");
    }

    #[test]
    fn aoc_2016_08_a_example() {
        let commands = parse(
            "rect 3x2
            rotate column x=1 by 1
            rotate row y=0 by 4
            rotate column x=1 by 1",
        );
        let mut screen = Screen::new(7, 3);

        // println!("{commands:?}");
        for cmd in commands {
            // println!("{cmd:?}");
            screen.execute(cmd);
        }
        println!("┌┬┬┬┬┬┬┬┐");
        println!("{}", screen);

        assert_eq!(format!("{}", screen), ".#..#.#\n#.#....\n.#.....\n");
    }

    #[rstest]
    #[case("", "###..\n###..\n.....\n")]
    #[case("rotate row y=0 by 5", "###..\n###..\n.....\n")]
    #[case("rotate row y=0 by 2", "..###\n###..\n.....\n")]
    #[case("rotate column x=0 by 1", ".##..\n###..\n#....\n")]
    #[case("rotate column x=1 by 1", "#.#..\n###..\n.#...\n")]
    #[case("rotate column x=2 by 1", "##...\n###..\n..#..\n")]
    #[case("rotate column x=3 by 1", "###..\n###..\n.....\n")]
    #[case("rotate column x=4 by 1", "###..\n###..\n.....\n")]
    #[case("rotate column x=1 by 2", "###..\n#.#..\n.#...\n")]
    fn rotate_should(#[case] input: &str, #[case] exepected: &str) {
        let commands = parse(format!("rect 3x2\n{}", input).as_str());
        let mut screen = Screen::new(5, 3);

        // println!("{commands:?}");
        for cmd in commands {
            // println!("{cmd:?}");
            screen.execute(cmd);
        }
        println!("┌┬┬┬┬┬┐");
        println!("{}", screen);

        assert_eq!(format!("{}", screen), exepected);
    }

    #[test]
    fn aoc_2016_08_a() {
        assert_eq!(super::aoc_2016_08_a(INPUT), 123);
    }

    #[test]
    fn aoc_2016_08_b_example() {
        assert_eq!(super::aoc_2016_08_b(""), 0);
    }

    #[test]
    fn aoc_2016_08_b() {
        // AFBUPZBJPS
        assert_eq!(super::aoc_2016_08_b(INPUT), 0);
    }

    const INPUT: &str = include_str!("input.txt");
}
