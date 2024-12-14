use anyhow::{anyhow, Result};
use nom::{
    character::complete::{space1, u32},
    multi::separated_list1,
    Finish, IResult,
};

fn parse_report(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, u32)(input)
}

fn safe_levels(levels: &[u32]) -> bool {
    (levels.is_sorted() || levels.is_sorted_by(|a, b| b <= a)) && safe_intervals(levels)
}

fn safe_intervals(levels: &[u32]) -> bool {
    levels.windows(2).all(|w| {
        let d = w[0].abs_diff(w[1]);
        (1..=3).contains(&d)
    })
}

fn main() -> Result<()> {
    let mut safe_count: u32 = 0;
    for line in std::io::stdin().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        let (_lin, levels) = parse_report(&line)
            .finish()
            .map_err(|err| anyhow!("failed to parse a line: {err}"))?;
        if safe_levels(&levels) {
            safe_count += 1;
        }
    }
    println!("{}", safe_count);
    Ok(())
}
