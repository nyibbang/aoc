use anyhow::Result;

fn main() -> Result<()> {
    let mut width = 0;
    let mut puzzle = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line?;
        if width == 0 {
            width = line.len();
        }
        puzzle.extend(line.bytes());
    }
    let puzzle = Puzzle {
        width,
        content: puzzle,
    };
    println!("{}", puzzle.count_x_mas());
    Ok(())
}

struct Puzzle {
    width: usize,
    content: Vec<u8>,
}

impl Puzzle {
    fn count_x_mas(&self) -> usize {
        self.content
            .iter()
            .enumerate()
            .filter_map(|(idx, &val)| {
                (val == b'A')
                    .then(|| self.coord(idx))
                    .filter(|coord| self.has_x_mas_at(coord))
            })
            .count()
    }

    fn has_x_mas_at(&self, coord: &Coord) -> bool {
        self.has_mas_at(coord, Diagonal::Slash) && self.has_mas_at(coord, Diagonal::Backslash)
    }

    fn has_mas_at(&self, coord: &Coord, diag: Diagonal) -> bool {
        assert_eq!(self.get(coord), b'A');
        match diag {
            Diagonal::Slash => {
                matches!(
                    (
                        coord.neighbour(Dir::TopRight),
                        coord.neighbour(Dir::BottomLeft),
                    ),
                    (Some(c1), Some(c2))
                    if is_m_and_s(self.get(&c1), self.get(&c2))
                )
            }
            Diagonal::Backslash => {
                matches!(
                    (
                        coord.neighbour(Dir::TopLeft),
                        coord.neighbour(Dir::BottomRight),
                    ),
                    (Some(c1), Some(c2))
                    if is_m_and_s(self.get(&c1), self.get(&c2))
                )
            }
        }
    }

    fn height(&self) -> usize {
        // always a perfect rectangle, so content_len = width * height
        self.content.len() / self.width
    }

    fn coord(&self, index: usize) -> Coord {
        assert!(index < self.content.len());
        let width = self.width as u32;
        let height = self.height() as u32;
        let x = (index % self.width) as u32;
        let y = (index / self.width) as u32;
        Coord {
            x,
            y,
            width,
            height,
        }
    }

    fn get(&self, coord: &Coord) -> u8 {
        let idx = coord.array_index();
        self.content[idx]
    }
}

fn is_m_and_s(a: u8, b: u8) -> bool {
    (a == b'M' && b == b'S') || (b == b'M' && a == b'S')
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    // x:0, y:0 is the top left of the puzzle.
    // y goes up as we go down, x goes up as we go right.
    // y is row index, x is column index.
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

impl Coord {
    fn neighbour(&self, dir: Dir) -> Option<Coord> {
        let (dx, dy) = dir.vec();
        let x = self.x.checked_add_signed(dx).filter(|&x| x < self.width)?;
        let y = self.y.checked_add_signed(dy).filter(|&y| y < self.height)?;
        Some(Self {
            x,
            y,
            width: self.width,
            height: self.height,
        })
    }

    fn array_index(&self) -> usize {
        (self.y * self.width + self.x) as usize
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Dir {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Dir {
    // Returns a vector of coordinates for this direction.
    fn vec(&self) -> (i32, i32) {
        match self {
            Self::TopLeft => (-1, -1),
            Self::TopRight => (1, -1),
            Self::BottomLeft => (-1, 1),
            Self::BottomRight => (1, 1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Diagonal {
    Slash,
    Backslash,
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[u8; 100] = b".M.S......\
                                        ..A..MSMS.\
                                        .M.S.MAA..\
                                        ..A.ASMSM.\
                                        .M.S.M....\
                                        ..........\
                                        S.S.S.S.S.\
                                        .A.A.A.A..\
                                        M.M.M.M.M.\
                                        ..........";

    fn example_puzzle() -> Puzzle {
        Puzzle {
            width: 10,
            content: EXAMPLE_INPUT.to_vec(),
        }
    }

    #[test]
    fn example() {
        let p = example_puzzle();
        assert_eq!(p.count_x_mas(), 9);
    }
}
