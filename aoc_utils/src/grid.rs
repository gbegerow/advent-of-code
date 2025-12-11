use glam::IVec2;
/// IVec2 based grid
/// Origin is left upper corner
/// Debug print like Grid[3x5] ['.','#','<','.','^']...
/// Display print pretty grid with scales
/// use iter_*_neighbours to iterate over adjacent gridcells
/// TODO: Add trait for parsing and displaying certain values / Positions
/// TODO: Create display implementations for bevy and Ratatui
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

#[derive(Clone, PartialEq, Eq)]
pub struct Grid<T> {
    values: Vec<T>,
    pub width: usize,
    pub height: usize,
    pub cursor: IVec2,
    pub lower_bound: IVec2,
    pub upper_bound: IVec2,
}

impl<T> Grid<T> {
    pub fn new(values: Vec<T>, width: usize, height: usize) -> Self {
        Self {
            values,
            width,
            height,
            cursor: IVec2::MIN,
            lower_bound: IVec2::ZERO,
            upper_bound: IVec2::new(width as i32 - 1, height as i32 - 1),
        }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.values.len()
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    #[inline]
    pub fn to_index(&self, index: IVec2) -> Option<usize> {
        if index.x < 0
            || index.y < 0
            || index.x >= self.width as i32
            || index.y >= self.height as i32
        {
            return None;
        }

        Some(index.y as usize * self.width + index.x as usize)
    }

    #[inline]
    pub fn to_ivec(&self, index: usize) -> IVec2 {
        // Todo: unittest with non quadratic grid
        IVec2::new((index % self.width) as i32, (index / self.width) as i32)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> + '_ {
        self.values.iter()
    }

    pub fn get(&self, pos: IVec2) -> Option<&T> {
        self.to_index(pos).map(|i| self.values.get(i))?
    }

    pub fn get_mut(&mut self, pos: IVec2) -> Option<&mut T> {
        self.to_index(pos).map(|i| self.values.get_mut(i))?
    }

    pub fn iter_with_positions(&self) -> impl Iterator<Item = (IVec2, &T)> + '_ {
        self.values
            .iter()
            .enumerate()
            .map(|(i, c)| (self.to_ivec(i), c))
    }

    /// iterate over all valid neighbours of pos given by offsets
    pub fn iter_neighbours(&self, delta: Vec<IVec2>, pos: IVec2) -> impl Iterator<Item = &T> {
        delta
            .iter()
            .flat_map(|delta| self.to_index(pos + delta))
            .map(|i| &self.values[i])
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// iterate over all valid neighbours of pos along major axis
    pub fn iter_axis_neighbours(&self, pos: IVec2) -> impl Iterator<Item = &T> {
        self.iter_neighbours(adjacent_4(), pos)
    }

    /// iterate over all valid neighbours of pos along major axis and diagonals
    pub fn iter_adajacent_neighbours(&self, pos: IVec2) -> impl Iterator<Item = &T> {
        self.iter_neighbours(adjacent_8(), pos)
    }

    /// iterate over all valid neighbours of pos along diagonals
    pub fn iter_diagonal_neighbours(&self, pos: IVec2) -> impl Iterator<Item = &T> {
        self.iter_neighbours(adjacent_diagonal(), pos)
    }

    /// iterate over all valid neighbours of pos given by offsets
    pub fn iter_neighbours_with_positions(
        &self,
        delta: Vec<IVec2>,
        pos: IVec2,
    ) -> impl Iterator<Item = (IVec2, &T)> {
        delta
            .iter()
            .flat_map(|delta| {
                self.to_index(pos + delta)
                    .map(|p| (pos + delta, &self.values[p]))
            })
            .collect::<Vec<_>>()
            .into_iter()
    }

    /// iterate over all valid neighbours of pos along major axis
    pub fn iter_axis_neighbours_with_positions(
        &self,
        pos: IVec2,
    ) -> impl Iterator<Item = (IVec2, &T)> {
        self.iter_neighbours_with_positions(adjacent_4(), pos)
    }

    /// iterate over all valid neighbours of pos along major axis and diagonals
    pub fn iter_adajacent_neighbours_with_positions(
        &self,
        pos: IVec2,
    ) -> impl Iterator<Item = (IVec2, &T)> {
        self.iter_neighbours_with_positions(adjacent_8(), pos)
    }

    /// iterate over all valid neighbours of pos along diagonals
    pub fn iter_diagonal_neighbours_with_positions(
        &self,
        pos: IVec2,
    ) -> impl Iterator<Item = (IVec2, &T)> {
        self.iter_neighbours_with_positions(adjacent_diagonal(), pos)
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find(&self, needle: T) -> Option<IVec2> {
        self.values
            .iter()
            .position(|p| *p == needle)
            .map(|index| self.to_ivec(index))
    }

    pub fn find_cursor(&mut self, cursor: T, swap_with: T) -> IVec2 {
        if let Some(cursor_pos) = self.values.iter().position(|p| *p == cursor) {
            self.values[cursor_pos] = swap_with;
            self.cursor = self.to_ivec(cursor_pos);
            // println!("Found cursor at {}", self.cursor);
        }
        // ignore not found for the moment. Maybe change to Result
        self.cursor
    }

    /// moves cursor if predicate returns true.
    /// predicate receives the new position and, if this position is inside bounds,
    /// the content of the tile of position. Otherwise None.
    /// if cursor was set, the tiles content is returnd otherwise None
    pub fn move_cursor_if(
        &mut self,
        direction: IVec2,
        predicate: fn(IVec2, Option<&T>) -> bool,
    ) -> Option<&T> {
        let new_pos = self.cursor + direction;
        let tile = self.to_index(new_pos).map(|i| &self.values[i]);
        if predicate(new_pos, tile) {
            self.cursor = new_pos;
            tile
        } else {
            None
        }
    }

    /// try to move cursor in direction, returns None if cursor would leave grid
    /// otherwise return tile under cursor
    pub fn move_cursor(&mut self, direction: IVec2) -> Option<&T> {
        // if cursor is in valid spot after move, set it
        match self
            .to_index(self.cursor + direction)
            .map(|i| &self.values[i])
        {
            Some(v) => {
                self.cursor += direction;
                Some(v)
            }
            None => None,
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn from_upper_bound(upper_bound: IVec2, empty: T) -> Self {
        let width: usize = (upper_bound.x + 1) as usize;
        let height: usize = (upper_bound.y + 1) as usize;

        Self {
            values: vec![empty; width * height],
            width,
            height,
            cursor: IVec2::MIN,
            lower_bound: IVec2::ZERO,
            upper_bound,
        }
    }
}

impl<T> Index<IVec2> for Grid<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: IVec2) -> &Self::Output {
        match self.to_index(index) {
            Some(i) => &self.values[i],
            None => panic!("{} out ouf bounds", index),
        }
    }
}

impl<T> IndexMut<IVec2> for Grid<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: IVec2) -> &mut T {
        match self.to_index(index) {
            Some(i) => &mut self.values[i],
            None => panic!("{} out ouf bounds", index),
        }
    }
}

fn adjacent_8() -> Vec<IVec2> {
    vec![
        IVec2::new(1, 1),
        IVec2::new(0, 1),
        IVec2::new(1, -1),
        IVec2::new(0, -1),
        IVec2::new(-1, 1),
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
        IVec2::new(-1, -1),
    ]
}

pub const NORTH: IVec2 = IVec2::new(0, -1);
pub const EAST: IVec2 = IVec2::new(1, 0);
pub const SOUTH: IVec2 = IVec2::new(0, 1);
pub const WEST: IVec2 = IVec2::new(-1, 0);

fn adjacent_4() -> Vec<IVec2> {
    vec![
        IVec2::new(0, 1),
        IVec2::new(0, -1),
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
    ]
}

fn adjacent_diagonal() -> Vec<IVec2> {
    vec![
        IVec2::new(1, 1),
        IVec2::new(1, -1),
        IVec2::new(-1, 1),
        IVec2::new(-1, -1),
    ]
}

#[allow(dead_code)]
fn get_knights_moves() -> Vec<IVec2> {
    // two along major axis, one along minor
    todo!("Knights moves")
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseGridError;

impl<T> FromStr for Grid<T>
where
    T: TryFrom<char>,
{
    type Err = ParseGridError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Err(ParseGridError);
        }
        let width = s
            .lines()
            .next()
            .expect("at least one line")
            .trim()
            .chars()
            .count();
        let height = s.lines().count();

        // use lines, we want to trim any line individually
        let values: Vec<T> = s
            .lines()
            .flat_map(|x| x.trim().chars())
            .flat_map(|c| c.try_into())
            .collect();

        Ok(Grid {
            values,
            width,
            height,
            cursor: IVec2::MIN,
            lower_bound: IVec2::ZERO,
            upper_bound: IVec2::new(width as i32 - 1, height as i32 - 1),
        })
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disp_width = (self.width + 2).min(40).min(self.values.len());
        let ellipses = if self.values.len() > disp_width {
            "..."
        } else {
            ""
        };
        write!(
            f,
            "Grid[{}x{} @{}] {:?}{}",
            self.width,
            self.height,
            self.cursor,
            &self.values[..disp_width],
            ellipses
        )
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pretty = true; //f.options().get_alternate(); // needs unstable feature #![feature(formatting_options)]

        if pretty {
            // scale
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match x % 10 {
                        0 => '|',
                        4 => ':', // rest 4 or 5? we want 3 ticks in between
                        _ => '\'',
                    }
                )?;
            }
            writeln!(f)?;
        }

        let width = self.width; // .saturating_sub(1);
        let cursor = if self.cursor != IVec2::MIN { self.to_index(self.cursor) } else { None };
        // grid
        for (index, c) in self.values.iter().enumerate() {
            //todo: if this is cursor pos we maybe change color
            if  Some(index) != cursor {
                write!(f, "{}", c)?;
            } else {
                println!("Cursor at index {} pos {}", index, self.cursor);
                write!(f, "@")?
            }
            if ((index + 1) % width) == 0 {
                if pretty {
                    writeln!(f, " |{:3}", (index + 1) / width)?
                } else {
                    writeln!(f)?
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(INPUT_01, GRID_01, 8, 6)]
    #[case(INPUT_02, GRID_02, 3, 3)]
    fn from_str_should(
        #[case] input: &str,
        #[case] exp_values: &str,
        #[case] exp_width: usize,
        #[case] exp_height: usize,
    ) {
        let sut: Grid<_> = input.parse().unwrap();
        let expected = Grid {
            values: exp_values.chars().collect(),
            width: exp_width,
            height: exp_height,
            cursor: IVec2::MIN,
            lower_bound: IVec2::ZERO,
            upper_bound: IVec2::new(
                (exp_width - 1).try_into().unwrap(),
                (exp_height - 1).try_into().unwrap(),
            ),
        };

        assert_eq!(sut, expected);
    }

    #[rstest]
    #[case(INPUT_01, DISPLAY_01)]
    #[case(INPUT_02, DISPLAY_02)]
    fn display_should(#[case] input: &str, #[case] expected: &str) {
        let sut: Grid<char> = input.parse().unwrap();
        let display = format!("{:#}", sut);
        assert_eq!(display, expected);
    }

    //---------------- Test inputs ----------------
    const INPUT_01: &str = "
    #.######
    #>>.<^<#
    #.<..<<#
    #>v.><>#
    #<^v^^>#
    ######.#";
    const GRID_01: &str = "#.#######>>.<^<##.<..<<##>v.><>##<^v^^>#######.#";

    const DISPLAY_01: &str = "|''':'''
#.###### |  1
#>>.<^<# |  2
#.<..<<# |  3
#>v.><># |  4
#<^v^^># |  5
######.# |  6
";

    const INPUT_02: &str = "123\n456\n789";
    const GRID_02: &str = "123456789";
    const DISPLAY_02: &str = "|''\n123 |  1\n456 |  2\n789 |  3\n";

    // const INPUT_01: &str = "";
    // const GRID_01: &Grid<char> = &Grid {
    //     values: vec![],
    //     width: 0,
    //     height: 0,
    //     lower_bound: IVec2::ZERO,
    //     upper_bound: IVec2::new(0, 0),
    // };
}
