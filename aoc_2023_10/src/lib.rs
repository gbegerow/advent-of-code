// #[allow(dead_code)]
/* Find the task under https://adventofcode.com/2023/day/10
    Solution idea:

*/
use std::collections::BTreeSet;

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
use std::{
    io::{self, Stdout},
    collections::BTreeMap
};




#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

/// in which direction leaves the pipe if we enter from last_direction
fn next_at(pipe: &char, last_direction: &Direction) -> Option<Direction> {
    match (pipe, last_direction) {
        ('-', Direction::West) => Some(Direction::West),
        ('-', Direction::East) => Some(Direction::East),
        ('|', Direction::North) => Some(Direction::North),
        ('|', Direction::South) => Some(Direction::South),
        ('L', Direction::South) => Some(Direction::East),
        ('L', Direction::West) => Some(Direction::North),
        ('F', Direction::North) => Some(Direction::East),
        ('F', Direction::West) => Some(Direction::South),
        ('J', Direction::South) => Some(Direction::West),
        ('J', Direction::East) => Some(Direction::North),
        ('7', Direction::East) => Some(Direction::South),
        ('7', Direction::North) => Some(Direction::West),

        // Special case start, just go in the given direction
        ('S', _) => Some(*last_direction),

        _ => None,
    }
}

/// walk from grid[row][col] in current direction
/// gives new coordinates and new direction
/// panics with broken pipe if the field is out of bounds
fn walk_from(
    grid: &Vec<Vec<char>>,
    row: usize,
    col: usize,
    current_dir: Direction,
) -> (usize, usize, Direction) {
    grid.get(row)
        .and_then(|rw| rw.get(col))
        .and_then(|field|  next_at(field, &current_dir))
        .and_then(|dir| {
            Some(match dir {
                Direction::North => (row - 1, col, dir),
                Direction::East => (row, col + 1, dir),
                Direction::South => (row + 1, col, dir),
                Direction::West => (row, col - 1, dir),
            })
        })
        .expect("broken pipe")
}

/// Scan around grid[row][col] for pipes and return all directions a pipe is found
fn scan(grid: &Vec<Vec<char>>, row: usize, col: usize) -> Vec<Direction> {
    // get will take care of overflow
    let mut neighbours = vec![
        (row, col + 1, Direction::East),
        (row + 1, col, Direction::South),
    ];
    if row > 0 {
        // keep it in clockwise order (just for sanity) 
        neighbours.insert(0, (row - 1, col, Direction::North));
    };
    if col > 0 {
        neighbours.push((row, col - 1, Direction::West));
    };

    neighbours
        .iter()
        .flat_map(|(r, c, dir)| {
            grid.get(*r)
                .and_then(|rw| rw.get(*c))
                .and_then(|field| next_at(field, dir))
                .and_then(|_| Some(dir.clone())) // we are not interessted where to go after scan, only which directions are valid
        })
        .collect()
}

/// find position of 'S' (start), panics if not found
fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize){
    for (row_num, row) in grid.iter().enumerate()  {
        if let Some(col_num) = row.iter().position(|&c| c == 'S') {
            return (row_num, col_num);
        }
    }
    unreachable!();
}

/// parse input to grid and start
fn parse(input: &str) -> (Vec<Vec<char>>, usize, usize) {
    assert!(input.trim().len() > 0);
    let grid = input
        .trim()
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let (start_row, start_col) = find_start(&grid);

    (grid, start_row, start_col)
}

fn track_loop(input: &str) -> (usize, BTreeSet<(usize, usize)>,  Vec<Vec<char>>, usize, usize){
    let (grid, start_row, start_col) = parse(input);
    
    let mut pipes = scan(&grid, start_row, start_col)
        .iter()
        // do step from start outside of loop to simplify end condition
        .map(|d| walk_from(&grid, start_row, start_col, *d))
        .collect::<Vec<_>>();
    let mut counter = 1;
    // track all tiles we have been to
    let mut loop_tiles = BTreeSet::from([
        (start_row, start_col),
        (pipes[0].0, pipes[0].1),
        (pipes[1].0, pipes[1].1),
        ]);
    
    #[cfg(feature = "viz")]
    let mut terminal = setup_viz().expect("setup viz");
    #[cfg(feature = "viz")]
    const SINGLE_STEP: bool = true;

    while (
        // longest path is found if both ways reach the same tile after start
        pipes[0].0 != pipes[1].0 
            || pipes[0].1 != pipes[1].1)    
        // safe guard against cycling    
        && counter < grid.len() * grid[0].len() {

        #[cfg(feature = "viz")]
        draw(&mut terminal, &grid, pipes[0].0, pipes[0].1, &loop_tiles,
            &BTreeMap::from([
                ("counter", &counter.to_string()),
                ("Path 1", &format!("{} {:?}", grid[pipes[0].0][pipes[0].1], pipes[0].2)),
                ("Path 2", &format!("{} {:?}", grid[pipes[1].0][pipes[1].1], pipes[1].2)),
            ]),
             SINGLE_STEP).expect("draw");

        // println!("pipes: {:?} {} {} count: {}", pipes, grid[pipes[0].0][pipes[0].1], grid[pipes[1].0][pipes[1].1], counter );
        pipes = pipes
            .iter()
            .map(|&(row, col, dir)| walk_from(&grid, row, col, dir))
            .collect();

        pipes.iter().for_each(|p| { loop_tiles.insert((p.0, p.1)); });
        counter += 1;
    }

    #[cfg(feature = "viz")]
    {
        draw(&mut terminal, &grid, pipes[0].0, pipes[0].1, &loop_tiles,
            &BTreeMap::from([
                ("counter", &counter.to_string()),
            ]),
             SINGLE_STEP).expect("draw");
        shutdown_viz().expect("shutdown viz");
    }
    
    (counter, loop_tiles, grid, start_row, start_col)
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
    grid: &Vec<Vec<char>>,
    current_row: usize,
    current_col: usize,
    track: &BTreeSet<(usize, usize)>,
    values: &BTreeMap<&str,&String>,
    single_step: bool,
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
                            let mut style =  if track.contains(&(row_no, col_no)) {
                                Style::default().fg(Color::Green)
                            } else {
                                Style::default()
                            };
                            if row_no == current_row && col_no == current_col {
                                style = style.bg(Color::Red)
                            } 

                            let c = field;
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

    let current_text = values.iter().flat_map(|(k, v)|{
        vec![
            Line::from(format!("{}: ", k)),
            Line::from(Span::styled(
                v.to_string(),
                Style::default().fg(Color::Yellow),
            )),
        ]
    }).collect::<Vec<_>>();

    let current_widget = Paragraph::new(current_text)
        .block(Block::default().title("Current").borders(Borders::ALL))
        .alignment(Alignment::Left);

    terminal.draw(|f| {
        // f.render_widget(p, f.size());

        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
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



pub fn aoc_2023_10_a(input: &str) -> usize {
    let (counter, _, _, _, _) = track_loop(input);
    counter    
}


#[allow(unused_variables)]
pub fn aoc_2023_10_b(input: &str) -> usize {
    let (_, track_loop, grid, start_row, start_col) = track_loop(input);

    let mut counter = 0;
   // scan every row,
    // what does it mean to be inside? 
    // Even / odd rule, but what counts as crossing the pipe?
    // runs like .F----7. or .L--J. has no inside tiles after last corner (U turn) (sqeazing around the pipe)
    // runs like .F--J. or .L--7. has inside tiles after last corner  (S turn) (crossing the pipe)
    // | alway crosses the pipe
    // could we consider S always a crossing corner? No. (s. Testinput1)
     for (row_no, row) in grid.iter().enumerate()  {
        let mut is_inside = false;
        let mut enter_at ='.';
        for (col_no, f) in row.iter().enumerate()  {
            let mut field = f.clone();
            // println!("[{} {}]: {} inside? {}", row_no, col_no, field, is_inside);
            if track_loop.contains(&(row_no, col_no)) {
                if field == 'S' {
                    field = get_tile_for_start(&grid, row_no, col_no);
                } 

                match field {
                    // we scan from left to right, so we can only enter from the left
                    'F' | 'L' => {
                        enter_at = field;
                    },
                    'J' if ['F'].contains(&enter_at) => {
                        is_inside = !is_inside;
                        enter_at='.';
                    },
                    '7' if ['L'].contains(&enter_at) => {
                        is_inside = !is_inside;
                        enter_at='.';
                    },
                    '|' => {
                        // crossing the pipe
                        is_inside = !is_inside;
                    },
                    _ => {
                        // nothing to do
                    }
                    
                }

            } else if is_inside {
                // we are inside
                println!("[{} {}]: {} inside? {}", row_no, col_no, field, is_inside);
                
                counter += 1;
            }
        }
    }

    // find inside
    counter    
}

fn get_tile_for_start(grid: &Vec<Vec<char>>, row: usize, col: usize) -> char {
    let dirs = scan(&grid, row, col);
    match &dirs[..2] {
        [Direction::East, Direction::South] => 'F',
        [Direction::North, Direction::East] => 'L',
        [Direction::South, Direction::West] => '7',
        [Direction::North, Direction::West] => 'J',
        [Direction::East, Direction::West] => '-',
        [Direction::North, Direction::South] => '|',
        _ => unreachable!(),
        
    }
}

#[cfg(test)]
mod tests {
    use crate::Direction;
    use rstest::rstest;

    #[test]
    fn aoc_2023_10_a_example() {
        assert_eq!(super::aoc_2023_10_a(TEST_INPUT), 8);
    }

    #[test]
    fn aoc_2023_10_a() {
        assert_eq!(super::aoc_2023_10_a(INPUT), 6815);
    }

    #[rstest]
    #[case(TEST_INPUT, 1)]
    #[case(TEST_INPUT2, 4)]
    #[case(TEST_INPUT3, 8)]
    fn aoc_2023_10_b_example(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::aoc_2023_10_b(input), expected);
    }

    #[test]
    fn aoc_2023_10_b() {
        assert_eq!(super::aoc_2023_10_b(INPUT), 269);
    }

    #[test]
    fn parse_should_find_start(){
        let (grid, start_row, start_col) = super::parse(TEST_INPUT);
        assert_eq!(grid[start_row][start_col], 'S');
    }

    #[rstest]
    #[case("...\n.S.\n...", vec![])]
    #[case(".|.\n.S.\n.J.", vec![Direction::North, Direction::South])]
    #[case("...\nLS-\n.-.", vec![Direction::East, Direction::West])]
    #[case(TEST_INPUT, vec![Direction::East, Direction::South])]
    fn scan_should_give_direction(#[case] input: &str, #[case] expected: Vec<Direction>) {
        let (grid, start_row, start_col) = super::parse(input);

        assert_eq!(super::scan(&grid, start_row, start_col), expected);
    }


    #[rstest]
    // upper route
    #[case(2,0,Direction::East, 2, 1, Direction::East,'S','J')]
    #[case(2,1,Direction::East, 1, 1, Direction::North,'J','F')]
    #[case(1,1,Direction::North, 1, 2, Direction::East,'F','J')]
    #[case(1,2,Direction::East, 0, 2, Direction::North,'J','F')]
    #[case(0,2,Direction::North, 0, 3, Direction::East,'F','7')]
    #[case(0,3,Direction::East, 1, 3, Direction::South,'7','|')]
    #[case(1,3,Direction::South, 2, 3, Direction::South,'|','L')]
    #[case(2,3,Direction::South, 2, 4, Direction::East,'L','7')]
    // lower route
    #[case(2,0,Direction::South, 3, 0, Direction::South,'S','|')]
    #[case(3,0,Direction::South, 4, 0, Direction::South,'|','L')]
    #[case(4,0,Direction::South, 4, 1, Direction::East,'L','J')]
    #[case(4,1,Direction::East, 3, 1, Direction::North,'J','F')]
    #[case(3,1,Direction::North, 3, 2, Direction::East,'F','-')]
    #[case(3,2,Direction::East, 3, 3, Direction::East,'-','-')]
    #[case(3,3,Direction::East, 3, 4, Direction::East,'-','J')]
    #[case(3,4,Direction::East, 2, 4, Direction::North,'J','7')]
    #[case(2,4,Direction::North, 2, 3, Direction::West,'7','L')]
    fn walk_should_give_direction(
        #[case] from_row: usize,
        #[case] from_col: usize,
        #[case] go: Direction,
        #[case] row: usize,
        #[case] col: usize,
        #[case] dir: Direction,
        #[case] from: char,
        #[case] to: char,
    ) {
        let (grid, _start_row, _start_col) = super::parse(TEST_INPUT);

        assert_eq!(grid[from_row][from_col], from);
        assert_eq!(grid[row][col], to);
        assert_eq!( 
            super::walk_from(&grid, from_row, from_col, go), 
            (row, col, dir));
    }

    #[rstest]
    #[case(TEST_INPUT, 'F')]
    #[case(TEST_INPUT2, 'F')]
    #[case(TEST_INPUT3, 'F')]
    #[case(INPUT, '7')]
    fn tile_for_start_should(#[case] input: &str, #[case] expected: char) {
        let (grid, start_row, start_col) = super::parse(input);

        assert_eq!(super::get_tile_for_start(&grid, start_row, start_col), expected);
    }

    const INPUT: &str = include_str!("input.txt");

    // Start at [2][0],
    const TEST_INPUT: &str = "
    7-F7-
    .FJ|7
    SJLL7
    |F--J
    LJ.LJ";

    // 4 tiles inside
    const TEST_INPUT2: &str = "
    ...........
    .S-------7.
    .|F-----7|.
    .||.....||.
    .||.....||.
    .|L-7.F-J|.
    .|..|.|..|.
    .L--J.L--J.
    ...........";

    // 8 Tiles inside loop
    const TEST_INPUT3 : &str ="
    .F----7F7F7F7F-7....
    .|F--7||||||||FJ....
    .||.FJ||||||||L7....
    FJL7L7LJLJ||LJ.L-7..
    L--J.L7...LJS7F-7L7.
    ....F-J..F7FJ|L7L7L7
    ....L7.F7||L7|.L7L7|
    .....|FJLJ|FJ|F7|.LJ
    ....FJL-7.||.||||...
    ....L---J.LJ.LJLJ...";
}
