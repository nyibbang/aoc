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
    println!("{}", puzzle.count_all(b"XMAS"));
    Ok(())
}

struct Puzzle {
    width: usize,
    content: Vec<u8>,
}

impl Puzzle {
    fn count_all(&self, word: &[u8]) -> usize {
        if word.is_empty() {
            return 0;
        }
        let head = word[0];
        self.content
            .iter()
            .enumerate()
            .filter_map(|(idx, &val)| (val == head).then_some(idx))
            .map(|idx| self.count_word_occurrences_at(self.coord(idx), word))
            .sum()
    }

    fn count_word_occurrences_at(&self, coord: Coord, word: &[u8]) -> usize {
        Dir::all()
            .iter()
            .filter(|&&dir| self.has_word_at_coord_in_dir(coord, word, dir))
            .count()
    }

    fn has_word_at_coord_in_dir(&self, src_coord: Coord, word: &[u8], dir: Dir) -> bool {
        let mut coord = Some(src_coord);
        for &c in word {
            match coord {
                None => return false,
                Some(some_coord) => {
                    if self.get(&some_coord) != c {
                        return false;
                    }
                    coord = some_coord.neighbour(dir);
                }
            }
        }
        true
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
    Top,
    TopRight,
    Left,
    Right,
    BottomLeft,
    Bottom,
    BottomRight,
}

impl Dir {
    fn all() -> &'static [Self] {
        &[
            Self::TopLeft,
            Self::Top,
            Self::TopRight,
            Self::Left,
            Self::Right,
            Self::BottomLeft,
            Self::Bottom,
            Self::BottomRight,
        ]
    }

    // Returns a vector of coordinates for this direction.
    fn vec(&self) -> (i32, i32) {
        match self {
            Self::TopLeft => (-1, -1),
            Self::Top => (0, -1),
            Self::TopRight => (1, -1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
            Self::BottomLeft => (-1, 1),
            Self::Bottom => (0, 1),
            Self::BottomRight => (1, 1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &[u8; 100] = b"MMMSXXMASM\
                                        MSAMXMSMSA\
                                        AMXSXMAAMM\
                                        MSAMASMSMX\
                                        XMASAMXAMM\
                                        XXAMMXXAMA\
                                        SMSMSASXSS\
                                        SAXAMASAAA\
                                        MAMMMXMMMM\
                                        MXMXAXMASX";

    fn example_puzzle() -> Puzzle {
        Puzzle {
            width: 10,
            content: EXAMPLE_INPUT.to_vec(),
        }
    }

    #[test]
    fn example() {
        let p = example_puzzle();
        assert_eq!(p.count_all(b"XMAS"), 18);
    }

    #[test]
    fn example_at() {
        let p = example_puzzle();
        assert_eq!(
            p.count_word_occurrences_at(
                Coord {
                    x: 3,
                    y: 9,
                    width: 10,
                    height: 10
                },
                b"XMAS"
            ),
            2
        );
    }
}
