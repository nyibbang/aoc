use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::u32, combinator::value,
};

#[derive(Clone, Debug)]
pub enum Direction {
    Left,  // Lower numbers
    Right, // Higher numbers
}

impl Direction {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        alt((value(Self::Left, tag("L")), value(Self::Right, tag("R")))).parse(s)
    }
}

#[derive(Debug)]
pub struct Rotation {
    pub direction: Direction,
    pub distance: u32,
}

impl Rotation {
    pub fn parse(s: &str) -> IResult<&str, Rotation> {
        (Direction::parse, u32)
            .map(|(direction, distance)| Self {
                direction,
                distance,
            })
            .parse(s)
    }

    pub fn apply(&self, mut dial: u32) -> (u32, u32) {
        let mut distance = self.distance;
        let mut over_0 = 0;
        match self.direction {
            Direction::Left => {
                while distance > 0 {
                    dial = dial.checked_sub(1).unwrap_or(TICKS - 1);
                    if dial == 0 {
                        over_0 += 1;
                    }
                    distance -= 1;
                }
                (dial, over_0)
            }
            Direction::Right => {
                let dial = dial + self.distance;
                (dial % TICKS, dial / TICKS)
            }
        }
    }
}

pub const START: u32 = 50;
pub const TICKS: u32 = 100;
pub const INPUT: &str = include_str!("input.txt");
