use nom::{
    IResult, Parser,
    branch::alt,
    character::complete::{char, space0},
    combinator::value,
    multi::many1,
    sequence::preceded,
};

pub const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Op {
    Add,
    Mul,
}

pub fn op(s: &[u8]) -> IResult<&[u8], Op> {
    alt((value(Op::Add, char('+')), value(Op::Mul, char('*')))).parse_complete(s)
}

pub fn operations(s: &[u8]) -> IResult<&[u8], Vec<Op>> {
    many1(preceded(space0, op)).parse_complete(s)
}

#[cfg(test)]
pub mod tests {
    pub const EXAMPLE: &[u8] = include_bytes!("example.txt");
}
