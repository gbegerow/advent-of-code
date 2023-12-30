// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/16
    Solution idea:
    Raytracing
    Store as u8[][]
    Upper nibble ./\|-
    Lower nibble traveeling directiion 1 per bit
    count if lower = 0 on enter
    set bit of traveling direction
    push splits on stack
    end on walls and when direction bit is already set
    Make it fast, expect all mirrors to rotate in part 2

    No rotating mirrors but wandering entry.
    Possible Optimization: Remember count, direction and origin for every field.
    If you ever enter the field from the same direction, compare count.
        Lower or equal => terminate run, will only go the same way, can never beat former solution
        Higher => do not terminate run, will only go the same way, but bookkeeping is needed,
                    remember the new count on every field
                    result will be higher by Count_new - Couunt_old
        if new result is higher than max remember new max
        remember result for origin?
    Tradoff much memory for speed up
    Unoptimized and without additional algorithmic optimization: 60ms for part 2...

    Start with visualization with (features before package)
    cargo test --features viz --package aoc_2023_16 --lib  -- tests::aoc_2023_16_a::case_2 --exact --nocapture
*/

// vizualization stuff
#[cfg(feature = "viz")]
use crossterm::{
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
#[cfg(feature = "viz")]
use ratatui::{prelude::*, widgets::*};
#[cfg(feature = "viz")]
use std::io::{self, Stdout};

// TODO: rewrite to use bitflags
// Objects upper nibble is cell content
pub const OBJECT_MASK: u8 = 0b1111_0000;
pub const EMPTY: u8 = 0b0000_0000;

pub const MIRROR_NW_SE: u8 = 0b0001_0000; // \
pub const MIRROR_SW_NE: u8 = 0b0010_0000; // /
pub const SPLIT_NS: u8 = 0b0100_0000;
pub const SPLIT_WE: u8 = 0b1000_0000;

// Direction (current and stored in lowernibble)
pub const DIRECTIONS_MASK: u8 = 0b0000_1111;

pub const NORTH: u8 = 0b0000_0001; // north or upwards
pub const EAST: u8 = 0b0000_0010; // east or forward
pub const SOUTH: u8 = 0b0000_0100; // south or downwards
pub const WEST: u8 = 0b0000_1000; // west or backwards

fn parse(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|row| {
            row.trim()
                .chars()
                .map(|c| match c {
                    '.' => EMPTY.clone(),
                    '\\' => MIRROR_NW_SE.clone(),
                    '/' => MIRROR_SW_NE.clone(),
                    '|' => SPLIT_NS.clone(),
                    '-' => SPLIT_WE.clone(),
                    _ => {
                        println!("Cat on keyboard agan? '{c}'");
                        unreachable!()
                    }
                })
                .collect()
        })
        .collect()
}

fn get(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> Option<u8> {
    grid.get(row)
        .and_then(|r| r.get(col))
        .and_then(|f| Some(f.clone()))
}

fn set_bit(grid: &mut Vec<Vec<u8>>, row: usize, col: usize, value: &u8) {
    grid[row][col] |= value;
}

fn goto(
    row: usize,
    col: usize,
    direction: Option<u8>,
    width: usize,
    height: usize,
) -> Option<(usize, usize, u8)> {
    //deep nesting seems clumsy, refactor
    if let Some(dir) = direction {
        // out of bounds?
        if (row == 0 && dir == NORTH)
            || (col == 0 && dir == WEST)
            || (row >= height - 1 && dir == SOUTH)
            || (col >= width && dir == EAST)
        {
            None
        } else {
            // next coordinates
            match dir {
                NORTH => Some((row - 1, col, dir)),
                EAST => Some((row, col + 1, dir)),
                SOUTH => Some((row + 1, col, dir)),
                WEST => Some((row, col - 1, dir)),
                _ => unreachable!(),
            }
        }
    } else {
        None
    }
}

fn get_next_directions(object: u8, direction: u8) -> (Option<u8>, Option<u8>) {
    let (next_direction, split_direction) = match (object, direction) {
        //  Mirror \
        (MIRROR_NW_SE, NORTH) => (Some(WEST), None), //  ^\ => <
        (MIRROR_NW_SE, EAST) => (Some(SOUTH), None), //  \< => ^
        (MIRROR_NW_SE, SOUTH) => (Some(EAST), None), //  \v => >
        (MIRROR_NW_SE, WEST) => (Some(NORTH), None), //  >\ => v
        // Mirror /
        (MIRROR_SW_NE, NORTH) => (Some(EAST), None), //  /^ => >
        (MIRROR_SW_NE, EAST) => (Some(NORTH), None), //  /< => v
        (MIRROR_SW_NE, SOUTH) => (Some(WEST), None), //  v/ => <
        (MIRROR_SW_NE, WEST) => (Some(SOUTH), None), //  >/ => ^
        // Splitter -
        (SPLIT_WE, NORTH) => (Some(WEST), Some(EAST)), //  ^- => <>
        (SPLIT_WE, SOUTH) => (Some(WEST), Some(EAST)), //  v- => <>
        // Splitter |
        (SPLIT_NS, WEST) => (Some(NORTH), Some(SOUTH)), //  ^- => ^v
        (SPLIT_NS, EAST) => (Some(NORTH), Some(SOUTH)), //  ^- => ^v

        _ => (Some(direction), None), // go straight ahead, no obstacle
    };
    (next_direction, split_direction)
}

#[cfg(feature = "viz")]
fn is_bit_set(field: u8, value: &u8) -> bool {
    field & *value == *value
}

#[cfg(feature = "viz")]
fn count_directons(field: u8) -> u8 {
    (if is_bit_set(field, &NORTH) { 1 } else { 0 })
        + (if is_bit_set(field, &EAST) { 1 } else { 0 })
        + (if is_bit_set(field, &SOUTH) { 1 } else { 0 })
        + (if is_bit_set(field, &WEST) { 1 } else { 0 })
}

#[cfg(feature = "viz")]
fn setup_viz() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Terminal::new(backend)
}

#[cfg(feature = "viz")]
fn shutdown_viz() -> io::Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    Ok(())
}

#[cfg(feature = "viz")]
fn draw<B: Backend>(
    terminal: &mut Terminal<B>,
    grid: &Vec<Vec<u8>>,
    current_row: usize,
    current_col: usize,
    current_dir: u8,
    current: u8,
    single_step: bool,
    count: u32,
) -> io::Result<()> {
    // format grid
    let text = Text::from(
        grid.iter()
            .enumerate()
            .map(|(row_no, row)| {
                Line::from(
                    row.iter()
                        .enumerate()
                        .map(|(col_no, field)| {
                            let energy = count_directons(field.clone());
                            let style = if col_no == current_col && row_no == current_row {
                                // current is always green
                                Style::default().fg(Color::Green)
                            } else {
                                // determined by energy level
                                match energy {
                                    1 => Style::default().fg(Color::Yellow),
                                    2 => Style::default().fg(Color::Magenta),
                                    3 => Style::default().fg(Color::Red),
                                    4 => Style::default().fg(Color::White),
                                    _ => Style::default(),
                                }
                            };

                            let c = match field & 0b1111_0000 {
                                EMPTY if energy == 0 => '.',
                                EMPTY if energy > 0 => '*',
                                MIRROR_NW_SE => '\\',
                                MIRROR_SW_NE => '/',
                                SPLIT_NS => '|',
                                SPLIT_WE => '-',
                                _ => '!',
                            };
                            Span::styled(c.to_string(), style)
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>(),
    );

    let grid_widget = Paragraph::new(text)
        .block(
            Block::default()
                .title("Advent of Code")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(251, 251, 199))),
        )
        // .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    //.wrap(Wrap { trim: true });

    let current_text = vec![
        Line::from("Count: "),
        Line::from(Span::styled(
            format!("{count}"),
            Style::default().bg(Color::Green),
        )),
        Line::from("Position: "),
        Line::from(Span::styled(
            format!("Row: {current_row} Col: {current_col}"),
            Style::default().bg(Color::Green),
        )),
        Line::from("Object: "),
        Line::from(Span::styled(
            match current {
                EMPTY => ".",
                MIRROR_NW_SE => "\\ Mirror NW SE",
                MIRROR_SW_NE => "/ Mirror SW NE",
                SPLIT_NS => "| Split NS",
                SPLIT_WE => "- Split WE",
                _ => "!",
            },
            Style::default().bg(Color::Green),
        )),
        Line::from("Current Direction: "),
        Line::from(Span::styled(
            match current_dir {
                NORTH => "Upwards / North",
                EAST => "Forwards / East",
                SOUTH => "Downwards / South",
                WEST => "Backwards / West",
                _ => "Unknown",
            },
            Style::default().bg(Color::Green),
        )),
    ];

    let current_widget = Paragraph::new(current_text)
        .block(Block::default().title("Current").borders(Borders::ALL))
        .alignment(Alignment::Left);

    terminal.draw(|f| {
        // f.render_widget(p, f.size());

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
            .split(f.size());

        f.render_widget(current_widget, chunks[0]);
        f.render_widget(grid_widget, chunks[1]);
    })?;

    if single_step {
        // wait for a keypress
        loop {
            let event = read()?;
            if event == Event::Key(KeyCode::Char(' ').into()) {
                break;
            }
        }
    }

    Ok(())
}

pub fn energize(grid: &mut Vec<Vec<u8>>, start: (usize, usize, u8)) -> u32 {
    #[cfg(feature = "viz")]
    let mut terminal = setup_viz().unwrap();
    #[cfg(feature = "viz")]
    const SINGLE_STEP: bool = false;

    // parse
    let (height, width) = (grid.len(), grid[0].len());

    #[cfg(feature = "viz")]
    draw(&mut terminal, &grid, 0, 0, EAST, EMPTY, SINGLE_STEP, 0).unwrap();

    // raytrace
    let mut count = 0;
    let mut queue = vec![start];

    // march along
    while let Some((row, col, direction)) = queue.pop() {
        // field is inside boundary?
        if let Some(field) = get(&grid, row, col) {
            // on enter
            if field & DIRECTIONS_MASK == EMPTY {
                // Never seen, new field energized
                count += 1;
            }

            // cycle detection
            // did we ever reached this field from the same direction? terminate ray
            if direction == field & direction {
                continue;
            }
            // remember the direction we came into this field to stop any circle
            set_bit(grid, row, col, &direction);

            // where to go next and how many?
            // is there any object in the field to send the ray in a new direction or split it in two?
            let object = field & OBJECT_MASK; // mask seen directions
            let (next_direction, split_direction) = get_next_directions(object, direction);

            // add a new way to go?
            if let Some(s) = goto(row, col, split_direction, width, height) {
                queue.push(s);
            }

            // go ahead on this path
            if let Some(n) = goto(row, col, next_direction, width, height) {
                queue.push(n);
            }

            // update viz
            #[cfg(feature = "viz")]
            draw(
                &mut terminal,
                &grid,
                row,
                col,
                direction,
                field,
                SINGLE_STEP,
                count,
            )
            .unwrap();
        }
    }

    #[cfg(feature = "viz")]
    {
        draw(&mut terminal, &grid, 0, 0, EMPTY, EMPTY, SINGLE_STEP, count).unwrap();
        shutdown_viz().unwrap();
    }

    // clear directions for next run
    for r in 0..height {
        for c in 0..width {
            grid[r][c] &= OBJECT_MASK;
        }
    }

    count
}

pub fn aoc_2023_16_a(input: &str) -> u32 {
    let mut grid = parse(input);
    energize(&mut grid, (0, 0, EAST.clone()))
}
pub fn aoc_2023_16_b(input: &str) -> u32 {
    let mut grid = parse(input);
    let (height, width) = (grid.len(), grid[0].len());

    // either make a copy of grid or reset direction part :(

    // unroll sides
    // top down
    let mut max = (0..width)
        .map(|col| energize(&mut grid, (0, col, SOUTH.clone())))
        .max();

    // left to right
    max = max.max(
        (0..height)
            .map(|row| energize(&mut grid, (row, 0, EAST.clone())))
            .max(),
    );

    // right to left
    max = max.max(
        (0..height)
            .map(|row| energize(&mut grid, (row, width - 1, WEST.clone())))
            .max(),
    );

    // bottom up
    max = max.max(
        (0..width)
            .map(|col| energize(&mut grid, (height - 1, col, NORTH.clone())))
            .max(),
    );

    max.unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(TEST_INPUT, 46)]
    #[case(INPUT, 6883)]
    fn aoc_2023_16_a(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(super::aoc_2023_16_a(input), expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 51)]
    #[case(INPUT, 7228)]
    fn aoc_2023_16_b(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(super::aoc_2023_16_b(input), expected);
    }

    #[test]
    fn should_parse() {
        assert_eq!(
            super::parse(r#".\/-|"#),
            vec![vec![
                super::EMPTY,
                super::MIRROR_NW_SE,
                super::MIRROR_SW_NE,
                super::SPLIT_WE,
                super::SPLIT_NS
            ]]
        )
    }

    #[rstest]
    #[case(0, 0, Some(crate::EMPTY))]
    #[case(1, 0, Some(crate::SPLIT_NS))]
    #[case(9, 5, Some(crate::SPLIT_NS))]
    #[case(6, 4, Some(crate::MIRROR_SW_NE))]
    #[case(7, 4, Some(crate::MIRROR_SW_NE))]
    fn get_should(#[case] row: usize, #[case] col: usize, #[case] expected: Option<u8>) {
        let grid = super::parse(TEST_INPUT);

        assert_eq!(super::get(&grid, row, col), expected);
    }

    #[test]
    fn set_should() {
        let mut grid = super::parse(TEST_INPUT);

        super::set_bit(&mut grid, 4, 4, &crate::NORTH);

        assert_eq!(super::get(&grid, 4, 4), Some(crate::NORTH));
    }

    #[rstest]
    #[case(8, 5, Some(crate::SOUTH), None)]
    #[case(5,5, Some(crate::SOUTH), Some((6,5, crate::SOUTH)))]
    #[case(5,5, Some(crate::NORTH), Some((4,5, crate::NORTH)))]
    fn goto_should(
        #[case] row: usize,
        #[case] col: usize,
        #[case] dir: Option<u8>,
        #[case] expected: Option<(usize, usize, u8)>,
    ) {
        assert_eq!(super::goto(row, col, dir, 9, 9), expected);
    }

    #[rstest]
    #[case(crate::MIRROR_SW_NE, crate::EAST, (Some(crate::NORTH), None))]
    #[case(crate::MIRROR_SW_NE, crate::NORTH, (Some(crate::EAST), None))]
    fn next_should(
        #[case] object: u8,
        #[case] dir: u8,
        #[case] expected: (Option<u8>, Option<u8>),
    ) {
        assert_eq!(super::get_next_directions(object, dir), expected);
    }

    const INPUT: &str = include_str!("input.txt");

    const TEST_INPUT: &str = r#"
    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|...."#;
}
