use std::{
    convert::Infallible,
    ops::{Index, IndexMut},
    str::FromStr,
};

pub const INPUT_SIZE: usize = 136;
pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cell {
    #[default]
    Empty,
    Paper,
}

impl Cell {
    fn new(c: u8) -> Self {
        match c {
            b'@' => Self::Paper,
            b'.' => Self::Empty,
            _ => panic!("bad character"),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos<const SIZE: usize> {
    x: usize,
    y: usize,
}

fn incr_caped<const SIZE: usize>(val: usize) -> Option<usize> {
    if val >= SIZE {
        return None;
    }
    val.checked_add(1).filter(|&val| val < SIZE)
}

impl<const SIZE: usize> Pos<SIZE> {
    fn top(self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_sub(1)?,
        })
    }
    fn left(self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(1)?,
            y: self.y,
        })
    }
    fn right(self) -> Option<Self> {
        Some(Self {
            x: incr_caped::<SIZE>(self.x)?,
            y: self.y,
        })
    }
    fn bottom(self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: incr_caped::<SIZE>(self.y)?,
        })
    }

    pub fn neighbors(self) -> impl Iterator<Item = Pos<SIZE>> {
        [
            self.top().and_then(Pos::left),
            self.top(),
            self.top().and_then(Pos::right),
            self.left(),
            self.right(),
            self.bottom().and_then(Pos::left),
            self.bottom(),
            self.bottom().and_then(Pos::right),
        ]
        .into_iter()
        .flatten()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Grid<const SIZE: usize>(Box<[[Cell; SIZE]; SIZE]>);

impl<const SIZE: usize> Grid<SIZE> {
    pub fn positions() -> Positions<SIZE> {
        Positions {
            current: Pos { x: 0, y: 0 },
        }
    }

    pub fn count_paper_neighbors(&self, pos: Pos<SIZE>) -> usize {
        pos.neighbors()
            .filter(|&pos| self[pos] == Cell::Paper)
            .count()
    }

    #[allow(dead_code)]
    pub fn is_accessible_paper(&self, pos: Pos<SIZE>) -> bool {
        self[pos] == Cell::Paper && self.count_paper_neighbors(pos) < 4
    }
}

impl<const SIZE: usize> Default for Grid<SIZE> {
    fn default() -> Self {
        Self(Box::new([[Cell::default(); SIZE]; SIZE]))
    }
}

impl<const SIZE: usize> Index<Pos<SIZE>> for Grid<SIZE> {
    type Output = Cell;

    fn index(&self, pos: Pos<SIZE>) -> &Self::Output {
        &self.0[pos.y][pos.x]
    }
}

impl<const SIZE: usize> IndexMut<Pos<SIZE>> for Grid<SIZE> {
    fn index_mut(&mut self, pos: Pos<SIZE>) -> &mut Self::Output {
        &mut self.0[pos.y][pos.x]
    }
}

impl<const SIZE: usize> FromStr for Grid<SIZE> {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid::default();
        for (y, line) in s.lines().enumerate() {
            for (x, val) in line.bytes().enumerate() {
                grid[Pos { x, y }] = Cell::new(val);
            }
        }
        Ok(grid)
    }
}

#[derive(Debug)]
pub struct Positions<const SIZE: usize> {
    current: Pos<SIZE>,
}

impl<const SIZE: usize> Iterator for Positions<SIZE> {
    type Item = Pos<SIZE>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.current;
        if self.current.x == SIZE || self.current.y == SIZE {
            return None;
        }
        self.current.x += 1;
        if self.current.x == SIZE {
            self.current.x = 0;
            self.current.y += 1;
        }
        Some(pos)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (SIZE * SIZE, Some(SIZE * SIZE))
    }
}

impl<const SIZE: usize> ExactSizeIterator for Positions<SIZE> {}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub const EXAMPLE: &str = "..@@.@@@@.\n\
        @@@.@.@.@@\n\
        @@@@@.@.@@\n\
        @.@@@@..@.\n\
        @@.@@@@.@@\n\
        .@@@@@@@.@\n\
        .@.@.@.@@@\n\
        @.@@@.@@@@\n\
        .@@@@@@@@.\n\
        @.@.@@@.@.";

    #[test]
    fn example_count_paper() {
        let grid: Grid<10> = EXAMPLE.parse().unwrap();
        assert_eq!(grid.count_paper_neighbors(Pos { x: 5, y: 2 }), 6);
    }
}
