use std::collections::{HashMap, HashSet};

// x=0, y=0 is top left corner. x increases as coord goes right, y increases as coord goes down.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub(crate) struct Coord {
    y: usize, // y before x to ensure good ordering.
    x: usize,
}

impl Coord {
    fn left_up(&self) -> Option<Self> {
        Some(Self {
            x: self.x_left()?,
            y: self.y_up()?,
        })
    }

    fn up(&self) -> Option<Self> {
        Some(Self {
            x: self.x(),
            y: self.y_up()?,
        })
    }

    fn right_up(&self) -> Option<Self> {
        Some(Self {
            x: self.x_right()?,
            y: self.y_up()?,
        })
    }

    fn left(&self) -> Option<Self> {
        Some(Self {
            x: self.x_left()?,
            y: self.y(),
        })
    }

    fn right(&self) -> Option<Self> {
        Some(Self {
            x: self.x_right()?,
            y: self.y(),
        })
    }

    fn left_down(&self) -> Option<Self> {
        Some(Self {
            x: self.x_left()?,
            y: self.y_down()?,
        })
    }

    fn down(&self) -> Option<Self> {
        Some(Self {
            x: self.x(),
            y: self.y_down()?,
        })
    }

    fn right_down(&self) -> Option<Self> {
        Some(Self {
            x: self.x_right()?,
            y: self.y_down()?,
        })
    }

    fn x(&self) -> usize {
        self.x
    }

    fn x_left(&self) -> Option<usize> {
        self.x.checked_sub(1)
    }

    fn x_right(&self) -> Option<usize> {
        self.x.checked_add(1)
    }

    fn y(&self) -> usize {
        self.y
    }

    fn y_up(&self) -> Option<usize> {
        self.y.checked_sub(1)
    }

    fn y_down(&self) -> Option<usize> {
        self.y.checked_add(1)
    }

    fn lesser_neighbours(&self) -> impl Iterator<Item = Coord> + Clone {
        [self.left_up(), self.up(), self.left()].into_iter().flatten()
    }
}

#[derive(Default, Debug)]
struct EngineSchematic {
    numbers: Vec<(u32, bool)>,
    previous_number_indexes: HashMap<Coord, usize>,
    previous_symbols: HashSet<Coord>
}

impl EngineSchematic {
    fn part_numbers_sum(&self) -> u32 {
        self.numbers.iter().filter_map(|(value, marked)| marked.then_some(value)).sum()
    }

    fn add_symbol(&mut self, coord: Coord) {
        let lesser_neighbours = coord.lesser_neighbours();
        let previous_number_limit = lesser_neighbours.clone().min().unwrap_or(coord);
        // Retain only previous numbers that are further than the limit, and mark numbers that are
        // neighbours to this symbol.
        self.previous_number_indexes.retain(|number_coord, index| {
            if lesser_neighbours.clone().any(|symbol_neighbour| symbol_neighbour == *number_coord) {
                self.numbers[*index].1 = true;
            }
            number_coord > &previous_number_limit
        })
    }

    fn add_number(&mut self, num: u32, coords: Vec<Coord>) {
        todo!()
    }
}

struct Parser {
    schematic: EngineSchematic,
    state: ParserState,
}

enum ParserState {
    Pending,
    Number(u32, Vec<Coord>),
}

impl Parser {
    fn new() -> Self {
        Self {
            schematic: EngineSchematic::default(),
            state: ParserState::Pending,
        }
    }

    fn set_char(&mut self, character: char, coord: Coord) {
        todo!()
    }

    fn new_line(&mut self) {
        self.commit_number()
    }

    fn finish(mut self) -> EngineSchematic {
        self.commit_number();
        self.schematic
    }

    fn commit_number(&mut self) {
        if let ParserState::Number(num, coords) = std::mem::replace(&mut self.state, ParserState::Pending) {
            self.schematic.add_number(num, coords);
        }
    }
}

pub(crate) fn engine_part_numbers_sum<L>(lines: L) -> u32 where L: Iterator, L::Item: AsRef<str> {
    let mut parser = Parser::new();
    for (y, line) in lines.enumerate() {
        parser.new_line();
        for (x, c) in line.as_ref().chars().enumerate() {
            parser.set_char(c, Coord { x, y });
        }
    }
    let schematic = parser.finish();
    schematic.part_numbers_sum()
}
