use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{space0, space1, u32},
    combinator::value,
    multi::separated_list0,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug)]
pub(crate) struct Game {
    #[allow(dead_code)]
    pub(crate) id: u32,
    sets: Vec<Set>,
}

impl Game {
    #[allow(dead_code)]
    pub(crate) fn is_constrained_by(&self, constraint: Set) -> bool {
        self.sets
            .iter()
            .all(|set| set.is_constrained_by(constraint))
    }

    #[allow(dead_code)]
    pub(crate) fn minimum_set(&self) -> Set {
        self.sets.iter().fold(Set::default(), |mut result, set| {
            result.update_max_colors(set);
            result
        })
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub(crate) struct Set {
    pub(crate) red: u32,
    pub(crate) green: u32,
    pub(crate) blue: u32,
}

impl Set {
    fn color_mut(&mut self, color: Color) -> &mut u32 {
        match color {
            Color::Red => &mut self.red,
            Color::Green => &mut self.green,
            Color::Blue => &mut self.blue,
        }
    }

    fn update_max_colors(&mut self, other: &Set) {
        self.red = std::cmp::max(self.red, other.red);
        self.green = std::cmp::max(self.green, other.green);
        self.blue = std::cmp::max(self.blue, other.blue);
    }

    fn is_constrained_by(&self, constraint: Set) -> bool {
        self.red <= constraint.red && self.green <= constraint.green && self.blue <= constraint.blue
    }

    #[allow(dead_code)]
    pub(crate) fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Color {
    Red,
    Green,
    Blue,
}

pub(crate) fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(input)?;
    let (input, id) = u32(input)?;
    let (input, _) = delimited(space0, tag(":"), space0)(input)?;
    let (input, sets) = separated_list0(delimited(space0, tag(";"), space0), set)(input)?;
    Ok((input, Game { id, sets }))
}

fn set(input: &str) -> IResult<&str, Set> {
    separated_list0(delimited(space0, tag(","), space0), color_count)
        .map(|color_counts| {
            color_counts
                .into_iter()
                .fold(Set::default(), |mut set, (count, color)| {
                    *set.color_mut(color) += count;
                    set
                })
        })
        .parse(input)
}

fn color_count(input: &str) -> IResult<&str, (u32, Color)> {
    separated_pair(u32, space1, color)(input)
}

fn color(input: &str) -> IResult<&str, Color> {
    alt((
        value(Color::Red, tag("red")),
        value(Color::Green, tag("green")),
        value(Color::Blue, tag("blue")),
    ))(input)
}
