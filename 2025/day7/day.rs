use std::ops::{Index, IndexMut};

pub const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cell {
    #[default]
    Empty,
    BeamStart,
    Beam,
    Splitter,
}

impl From<u8> for Cell {
    fn from(value: u8) -> Self {
        match value {
            b'.' => Cell::Empty,
            b'S' => Cell::BeamStart,
            b'|' => Cell::Beam,
            b'^' => Cell::Splitter,
            _ => unreachable!("bad cell character {}", char::from(value)),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pos {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

fn incr_caped(val: usize, bound: usize) -> Option<usize> {
    if val >= bound {
        return None;
    }
    val.checked_add(1).filter(|&val| val < bound)
}

impl Pos {
    fn top(self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: self.y.checked_sub(1)?,
            width: self.width,
            height: self.height,
        })
    }
    fn left(self) -> Option<Self> {
        Some(Self {
            x: self.x.checked_sub(1)?,
            y: self.y,
            width: self.width,
            height: self.height,
        })
    }
    fn right(self) -> Option<Self> {
        Some(Self {
            x: incr_caped(self.x, self.width)?,
            y: self.y,
            width: self.width,
            height: self.height,
        })
    }
    fn bottom(self) -> Option<Self> {
        Some(Self {
            x: self.x,
            y: incr_caped(self.y, self.height)?,
            width: self.width,
            height: self.height,
        })
    }

    pub fn neighbors(self) -> impl Iterator<Item = Pos> {
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
pub struct Grid {
    cells: Box<[Cell]>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(input: &[u8]) -> Self {
        let mut cells = Vec::new();
        let mut height = 0;
        let mut width = None;
        for (idx, byte) in input.iter().enumerate() {
            if *byte == b'\n' {
                if width.is_none() {
                    width = Some(idx);
                }
                height += 1;
            } else {
                cells.push((*byte).into());
            }
        }
        eprintln!("{:?}", cells);

        Self {
            cells: cells.into_boxed_slice(),
            width: width.unwrap(),
            height,
        }
    }

    pub fn positions(&self) -> Positions {
        Positions {
            current: Pos {
                x: 0,
                y: 0,
                width: self.width,
                height: self.height,
            },
            width: self.width,
            height: self.height,
        }
    }
}

impl Index<Pos> for Grid {
    type Output = Cell;

    fn index(&self, pos: Pos) -> &Self::Output {
        &self.cells[pos.y * self.width + pos.x]
    }
}

impl IndexMut<Pos> for Grid {
    fn index_mut(&mut self, pos: Pos) -> &mut Self::Output {
        &mut self.cells[pos.y * self.width + pos.x]
    }
}
#[derive(Debug)]
pub struct Positions {
    current: Pos,
    width: usize,
    height: usize,
}

impl Iterator for Positions {
    type Item = Pos;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = self.current;
        if self.current.x == self.width || self.current.y == self.height {
            return None;
        }
        self.current.x += 1;
        if self.current.x == self.width {
            self.current.x = 0;
            self.current.y += 1;
        }
        Some(pos)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.width * self.height, Some(self.width * self.height))
    }
}

impl ExactSizeIterator for Positions {}

#[cfg(test)]
pub mod tests {
    pub const EXAMPLE: &[u8] = include_bytes!("example.txt");
}
