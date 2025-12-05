use nom::{
    IResult, Parser,
    character::complete::{char, line_ending, u64},
    multi::separated_list1,
    sequence::{pair, separated_pair},
};
use std::ops::RangeInclusive;

pub const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Database {
    pub(crate) fresh_ranges: Vec<RangeInclusive<u64>>,
    pub(crate) ingredients: Vec<u64>,
}

fn database_entry(s: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(u64, char('-'), u64)
        .map(|(beg, end)| beg..=end)
        .parse(s)
}

pub fn database(s: &str) -> IResult<&str, Database> {
    separated_pair(
        separated_list1(line_ending, database_entry),
        pair(line_ending, line_ending),
        separated_list1(line_ending, u64),
    )
    .map(|(fresh_ranges, ingredients)| Database {
        fresh_ranges,
        ingredients,
    })
    .parse(s)
}

#[cfg(test)]
pub mod tests {
    use nom::Finish;

    use super::*;
    pub const EXAMPLE: &str = include_str!("example.txt");

    #[test]
    fn parse_example() {
        assert_eq!(
            database(EXAMPLE).finish().unwrap().1,
            Database {
                fresh_ranges: vec![3..=5, 10..=14, 16..=20, 12..=18],
                ingredients: vec![1, 5, 8, 11, 17, 32]
            }
        );
    }
}
