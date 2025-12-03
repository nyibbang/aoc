use nom::{
    Finish, IResult, Parser,
    character::complete::{char, u64},
    multi::separated_list0,
    sequence::separated_pair,
};
use std::ops::RangeInclusive;

pub const INPUT: &str = include_str!("input.txt");

fn parse_range(s: &str) -> IResult<&str, RangeInclusive<u64>> {
    separated_pair(u64, char('-'), u64)
        .map(|(begin, end)| begin..=end)
        .parse(s)
}

pub fn parse_ranges(s: &str) -> IResult<&str, Vec<RangeInclusive<u64>>> {
    separated_list0(char(','), parse_range).parse(s)
}

pub fn sum_invalid_ids<F>(input: &str, is_invalid: F) -> u64
where
    F: Fn(u64) -> bool,
{
    parse_ranges(input.trim())
        .finish()
        .unwrap()
        .1
        .into_iter()
        .flatten()
        .filter(|id| is_invalid(*id))
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_range() {
        assert_eq!(
            super::parse_range("9191906840-9191941337"),
            Ok(("", 9191906840..=9191941337))
        );
    }

    #[test]
    fn parse_ranges() {
        assert_eq!(
            super::parse_ranges("24-47,613-1077"),
            Ok(("", vec![24..=47, 613..=1077]))
        )
    }
}
