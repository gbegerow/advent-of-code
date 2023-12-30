use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, QueueableCommand, Result,
};
use std::io::{stdout, Write};


const LEFT_WALL:u8 =  0b01000000; 
const RIGHT_WALL:u8 = 0b00000001; 

#[allow(dead_code)]
fn draw_chimney(chimmney: &Vec<u8>, _rock: &Vec<u8>, y_pos: usize) -> Result<()> {
    let mut stdout = stdout();

    let (w, h) = terminal::size()?;
    let  col: u16 = w - 10; // at the right

    stdout.queue(terminal::Clear(terminal::ClearType::All))?;
    
    // chimney
    for row in 0..h {
        stdout.queue(cursor::MoveTo(col, row as u16))?;

        stdout.queue(style::PrintStyledContent("║".grey()))?;
        let line = chimmney[y_pos - row as usize];
        let mut column = 0b01000000;
        while column > 0 {
            if column & line  != 0{
                stdout.queue(style::PrintStyledContent("█".white()))?;
            } else {
                stdout.queue(style::PrintStyledContent(".".grey()))?;                
            }

            column >>= 1;
        } 
        stdout.queue(style::PrintStyledContent("║".grey()))?;
    }

    // TODO: Tile

    stdout.queue(cursor::MoveTo(0,0))?;
    stdout.flush()?;
    Ok(())
}

pub fn aoc_2022_17_a(input: &str) -> usize {
    // tiles are stored lowest line first
    let tile_defs :Vec<Vec<u8>>= vec![
        vec![0b00011110,],
        vec![0b00001000, 
             0b00011100,
             0b00001000,],
        vec![0b00011100,
             0b00000100,
             0b00000100,], 
        vec![0b00010000,
             0b00010000,
             0b00010000,
             0b00010000,],
        vec![0b00011000,
             0b00011000,]     
    ];
    

    let jets = input.trim().as_bytes();
    let mut current_jet = 0;
    let mut tile_index = 0;
    
    let mut chimney = vec![0u8; 4096];
    chimney[0] = 0b01111111; // Start with floor as guardian
    let mut highest = 0;

    for _rock in 0..2022 { // TODO: check for 1 off
        let mut tile :Vec<u8> = tile_defs[tile_index].clone(); 
        tile_index += 1;
        if tile_index >= tile_defs.len() {tile_index = 0;}

        // lowest line of tile is index y
        let mut y = highest + 3 +1 +1; // 3 space above + 1 start + 1 decrease at beginning  TODO: check for 1 off  ✔️

        // rock falling down
        let mut falling = true;
        while falling {
            y -= 1;

            // draw_chimney(&chimney, &tile, y).unwrap_or_default();

            // shift by jet
            if jets[current_jet] == b'>'{ // shift right
                // test for wall collisions
                if tile.iter().all(|line| line & RIGHT_WALL == 0) {
                    // move all bits right
                    for line in tile.iter_mut(){
                        *line >>= 1; 
                    }
                }
                // print!(">");   
            } else if jets[current_jet] == b'<'{ // shift left
                // test for wall collisions
                if tile.iter().all(|line| line & LEFT_WALL == 0) {
                    // move all bits right
                    for line in tile.iter_mut(){
                        *line <<= 1; 
                    }
                }
                // print!("<");   
            } 
            // println!("shift:{} \ttile: {}", jets[current_jet] as char, &tile.iter().map(|l| format!("{l:08b}")).collect::<Vec<_>>().join(", "));          
            current_jet += 1;
            if current_jet >= jets.len() {current_jet = 0;}

            // fall down
            // do we have a collison in next line? start test with lowest line
            for (i, line) in tile.iter().enumerate(){
                let index = y + i -1; // check for 1 off ✔️

                // println!("y: {} index: {} chimney: {:07b} i: {} line: {:07b}", y, index, chimney[index],  i, line);          

                if chimney[index] & line != 0 { 
                    falling = false; 
                    break;
                }
            }
            // print!("{}", if falling {"v"} else {"#\n"})
            // println!("h:{} y:{} fall:{} \tchimney: {} tile: {}", highest, y, falling, 
            //     &chimney[..5].into_iter().map(|l| format!("{l:#08b}")).collect::<Vec<_>>().join(", ")
            //     &tile.into_iter().map(|l| format!("{l:#08b}")).collect::<Vec<_>>().join(", "));
        }

        // rest
        for(i, line) in tile.iter().enumerate(){
            let index = y + i; // check for 1 off ✔️

            chimney[index] |= line; 
        }
        // correct state
        while chimney[highest+1] != 0 {
            highest += 1;
        }

                
        println!("h:{}\tchimney: {}", highest, &chimney[..20].into_iter().map(|l| format!("{l:07b}")).collect::<Vec<_>>().join(", "));

        // println!("h:{} y:{} fall:{} \tchimney: {} tile: {}", highest, y, falling, 
        //     &chimney[..10].into_iter().map(|l| format!("{l:08b}")).collect::<Vec<_>>().join(", "),
        //     &tile.iter().map(|l| format!("{l:08b}")).collect::<Vec<_>>().join(", "));
        // print!("{:04}, ", highest);
    }
    
    highest
}

pub fn aoc_2022_17_b(_input: &str) -> usize {
    0
}



#[cfg(test)]
mod tests {
    #[test]
    fn aoc_2022_17_a_example() {
        assert_eq!(super::aoc_2022_17_a(TEST_INPUT), 3068);
    }

    #[test]
    fn aoc_2022_17_a() {
       assert_eq!(super::aoc_2022_17_a(include_str!("input.txt")), 0);
    }
    
    #[test]
    fn aoc_2022_17_b_example() {
        assert_eq!(super::aoc_2022_17_b(TEST_INPUT), 0);
    }

    #[test]
    fn aoc_2022_17_b() {
        assert_eq!(super::aoc_2022_17_b(include_str!("input.txt")), 0);
    }

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
}



