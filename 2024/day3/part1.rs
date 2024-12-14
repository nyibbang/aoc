use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::u32,
    sequence::{delimited, separated_pair},
    Finish, IResult,
};

fn parse_instr(input: &str) -> IResult<&str, (u32, u32)> {
    delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")"))(input)
}

fn main() -> Result<()> {
    let mut result: u32 = 0;
    for line in std::io::stdin().lines() {
        let line = line?;
        let mut line = line.as_str();
        while !line.is_empty() {
            match parse_instr(line).finish() {
                Ok((line_tail, (a, b))) => {
                    result += a * b;
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
