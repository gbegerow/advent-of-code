use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use std::io::{stdout, Write};

struct Cpu {
    x: i32,
    cycle: usize,
    pixel: i32,
    blips: Vec<(i32, usize)>,
    crt: Vec<u8>,
}

impl Cpu {
    const COLS: usize = 40;
    const ROWS: usize = 6;
    fn new() -> Self {
        Self {
            x: 1,
            cycle: 0,
            pixel: 0,
            blips: Vec::new(),
            crt: vec!['.' as u8; Self::COLS * Self::ROWS],
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;

        if self.cycle >= 20 && 0 == (self.cycle - 20) % 40 {
            self.blips.push((self.x, self.cycle))
        }

        // is 3px sprite at crt?
        let x = self.pixel % Self::COLS as i32;
        if x >= self.x - 1 && x <= self.x + 1 {
            self.crt[self.pixel as usize] = b'#';
        }

        self.pixel += 1;
    }

    fn execute(&mut self, input: &str) {
        for line in input.trim().lines() {
            match line.trim().split_whitespace().collect::<Vec<&str>>()[..] {
                ["addx", v] => {
                    self.tick();
                    self.tick();
                    self.x += v
                        .parse::<i32>()
                        .expect(format!("invalid argument {line}").as_str());
                }
                ["noop"] => self.tick(),
                _ => println!("{line}"),
            }
        }
    }

    // fn draw_crt(&self) -> std::io::Result<()>{
    //     let mut stdout = stdout();
    //     for row in self.crt.chunks(Self::COLS){
    //         stdout.write(row)?;
    //         stdout.write(b"\n")?;
    //     }

    //     stdout.write(b"\n")?;
    //     stdout.flush()?;

    //     Ok(())
    // }

    // print crt fancy
    fn draw_crt(&self) -> Result<()> {
        let mut stdout = stdout();

        let mut row: u16 = 0;
        let mut col: u16 = 0;

        stdout.queue(terminal::Clear(terminal::ClearType::All))?;
        // stdout.queue(cursor::MoveToNextLine(1))?;
        
        for c in &self.crt[..] {
            if *c == b'#' {
                stdout
                    .queue(cursor::MoveTo(col, row))?
                    .queue(style::PrintStyledContent("#".magenta()))?;
            }

            col += 1;
            if col as usize >= Self::COLS {
                col = 0;
                row += 1;
                if row as usize > Self::ROWS {
                    row = 0;
                }
            }
        }
        stdout.queue(cursor::MoveToNextLine(1))?;
        stdout.flush()?;
        Ok(())
    }
}

pub fn aoc_2022_10_a(input: &str) -> usize {
    let mut cpu = Cpu::new();
    cpu.execute(input);
    println!("{:?}", cpu.blips);

    cpu.blips.iter().map(|b| b.0 as usize * b.1).sum()
}

pub fn aoc_2022_10_b(input: &str) -> std::io::Result<()> {
    let mut cpu = Cpu::new();
    cpu.execute(input);

    println!("{:?}", cpu.blips);
    cpu.draw_crt()
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_10_a_example() {
        assert_eq!(super::aoc_2022_10_a(TEST_INPUT), 13140);
    }

    #[test]
    fn aoc_2022_10_a() {
        assert_eq!(super::aoc_2022_10_a(include_str!("input.txt")), 17940);
    }

    #[test]
    fn aoc_2022_10_b_example() {
        assert!(super::aoc_2022_10_b(TEST_INPUT).is_ok());
    }

    #[test]
    fn aoc_2022_10_b() {
        /* Print ZCBAJFJZ in 5x6 "Font"
        ####..##..###...##....##.####...##.####.
        ...#.#..#.#..#.#..#....#.#.......#....#.
        ..#..#....###..#..#....#.###.....#...#..
        .#...#....#..#.####....#.#.......#..#...
        #....#..#.#..#.#..#.#..#.#....#..#.#....
        ####..##..###..#..#..##..#.....##..####.
        */
        assert!(super::aoc_2022_10_b(include_str!("input.txt")).is_ok());
    }

    const TEST_INPUT: &str = "addx 15
    addx -11
    addx 6
    addx -3
    addx 5
    addx -1
    addx -8
    addx 13
    addx 4
    noop
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx 5
    addx -1
    addx -35
    addx 1
    addx 24
    addx -19
    addx 1
    addx 16
    addx -11
    noop
    noop
    addx 21
    addx -15
    noop
    noop
    addx -3
    addx 9
    addx 1
    addx -3
    addx 8
    addx 1
    addx 5
    noop
    noop
    noop
    noop
    noop
    addx -36
    noop
    addx 1
    addx 7
    noop
    noop
    noop
    addx 2
    addx 6
    noop
    noop
    noop
    noop
    noop
    addx 1
    noop
    noop
    addx 7
    addx 1
    noop
    addx -13
    addx 13
    addx 7
    noop
    addx 1
    addx -33
    noop
    noop
    noop
    addx 2
    noop
    noop
    noop
    addx 8
    noop
    addx -1
    addx 2
    addx 1
    noop
    addx 17
    addx -9
    addx 1
    addx 1
    addx -3
    addx 11
    noop
    noop
    addx 1
    noop
    addx 1
    noop
    noop
    addx -13
    addx -19
    addx 1
    addx 3
    addx 26
    addx -30
    addx 12
    addx -1
    addx 3
    addx 1
    noop
    noop
    noop
    addx -9
    addx 18
    addx 1
    addx 2
    noop
    noop
    addx 9
    noop
    noop
    noop
    addx -1
    addx 2
    addx -37
    addx 1
    addx 3
    noop
    addx 15
    addx -21
    addx 22
    addx -6
    addx 1
    noop
    addx 2
    addx 1
    noop
    addx -10
    noop
    noop
    addx 20
    addx 1
    addx 2
    addx 2
    addx -6
    addx -11
    noop
    noop
    noop";
}
