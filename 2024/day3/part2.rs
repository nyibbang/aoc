use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::u32,
    sequence::{delimited, separated_pair},
    Finish, IResult, Parser,
};

enum Instr {
    Do,
    Dont,
    Mul(u32, u32),
}

fn parse_mul(input: &str) -> IResult<&str, Instr> {
    delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")"))
        .map(|(a, b)| Instr::Mul(a, b))
        .parse(input)
}

fn parse_do(input: &str) -> IResult<&str, Instr> {
    tag("do()").map(|_| Instr::Do).parse(input)
}

fn parse_dont(input: &str) -> IResult<&str, Instr> {
    tag("don't()").map(|_| Instr::Dont).parse(input)
}

fn parse_instr(input: &str) -> IResult<&str, Instr> {
    alt((parse_mul, parse_do, parse_dont))(input)
}

fn main() -> Result<()> {
    let mut result: u32 = 0;
    let mut enabled = true;
    for line in std::io::stdin().lines() {
        let line = line?;
        let mut line = line.as_str();
        while !line.is_empty() {
            match parse_instr(line).finish() {
                Ok((line_tail, instr)) => {
                    match instr {
                        Instr::Do => enabled = true,
                        Instr::Dont => enabled = false,
                        Instr::Mul(a, b) => {
                            if enabled {
                                result += a * b
                            }
                        }
                    }
                    line = line_tail;
                }
                Err(err) => {
                    line = &err.input[1..];
                }
            }
        }
    }
    println!("{}", result);
    Ok(())
}
