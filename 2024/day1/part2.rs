use std::collections::HashMap;

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{space1, u32},
    sequence::separated_pair,
    Finish, IResult,
};

fn line_parser(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, space1, u32)(input)
}

fn main() -> Result<()> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in std::io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let (_lin, (l, r)) = line_parser(&line)
            .finish()
            .map_err(|err| anyhow!("failed to parse a line: {err}"))?;
        left.push(l);
        right.push(r);
    }
    assert_eq!(left.len(), right.len());

    let mut occurences = HashMap::<_, u32>::new();
    for num in right {
        *occurences.entry(num).or_default() += 1;
    }

    let mut result = 0;
    for num in left {
        result += *occurences.entry(num).or_default() * num;
    }

    println!("{}", result);
    Ok(())
}
