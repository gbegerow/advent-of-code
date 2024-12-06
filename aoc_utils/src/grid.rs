use glam::IVec2;
///! IVec2 based grid
///! Origin is left upper corner
use std::{
    ops::{Index, IndexMut},
    str::FromStr,
    string::ParseError,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<T> {
    values: Vec<T>,
    width: usize,
    height: usize,
    lower_bound: IVec2,
    upper_bound: IVec2,
}

// pub struct NeihbourIterator {
// todo
// }

impl<T> Grid<T> {
    #[inline(always)]
    fn to_index(&self, index: IVec2) -> Option<usize> {
        if index.x < 0
            || index.y < 0
            || index.x >= self.width as i32
            || index.y >= self.height as i32
        {
            return None;
        }

        Some(index.y as usize * self.width + index.x as usize)
    }

    #[inline(always)]
    fn to_ivec(&self, index: usize) -> IVec2 {
        IVec2::new((index % self.width) as i32, (index / self.height) as i32)
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
        self.iter_neighbours(get_axis_positions(), pos)
    }

    /// iterate over all valid neighbours of pos along major axis and diagonals
    pub fn iter_adajacent_neighbours(&self, pos: IVec2) -> impl Iterator<Item = &T> {
        self.iter_neighbours(get_adjacent_positions(), pos)
    }

    /// iterate over all valid neighbours of pos along diagonals
    pub fn iter_diagonal_neighbours(&self, pos: IVec2) -> impl Iterator<Item = &T> {
        self.iter_neighbours(get_diagonal_positions(), pos)
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

fn get_adjacent_positions() -> Vec<IVec2> {
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

fn get_axis_positions() -> Vec<IVec2> {
    vec![
        IVec2::new(0, 1),
        IVec2::new(0, -1),
        IVec2::new(1, 0),
        IVec2::new(-1, 0),
    ]
}

fn get_diagonal_positions() -> Vec<IVec2> {
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

impl<T> FromStr for Grid<T>
where
    Vec<T>: FromIterator<char>,
{
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.trim().lines().next().unwrap().trim().chars().count() as usize;
        let height = s.trim().lines().count() as usize;

        let values: Vec<T> = s.lines().flat_map(|x| x.trim().chars()).collect();

        Ok(Grid {
            values,
            width,
            height,
            lower_bound: IVec2::ZERO,
            upper_bound: IVec2::new(width as i32 - 1, height as i32 - 1),
        })
    }
}
