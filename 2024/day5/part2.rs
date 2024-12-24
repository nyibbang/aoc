use std::collections::HashMap;

use anyhow::Result;
use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::u32, multi::separated_list1,
    sequence::separated_pair, IResult,
};

fn main() -> Result<()> {
    println!("{}", run(std::io::stdin().lines())?);
    Ok(())
}

fn run<I, E>(lines: I) -> Result<u32>
where
    I: IntoIterator<Item = std::result::Result<String, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut rules: HashMap<_, Vec<_>> = HashMap::new();
    let mut updates = Vec::new();
    for line in lines {
        let line = line?;
        let line = line.trim();
        if let Ok((_, (first, second))) = parse_rule(line) {
            rules.entry(first).or_default().push(second);
        } else if let Ok((_, pages)) = parse_update(line) {
            updates.push(pages);
        }
    }
    let mut ordering = rules.keys().collect_vec();
    ordering.sort_by(|a, b| {});
    let mut result = 0;
    for update in updates {
        for (page_idx, page) in update.iter().enumerate() {
            for following_page in &update[(page_idx + 1)..update.len()] {
                if rules
                    .get(following_page)
                    .map(|page_rules| page_rules.contains(page))
                    .unwrap_or(false)
                {}
            }
        }
        let mid_index = update.len() / 2;
        let mid = update[mid_index];
        result += mid;
    }
    Ok(result)
}

fn parse_rule(input: &str) -> IResult<&str, (u32, u32)> {
    separated_pair(u32, tag("|"), u32)(input)
}

fn parse_update(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(tag(","), u32)(input)
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::*;

    #[test]
    fn example() {
        let input = r#"47|53
                       97|13
                       97|61
                       97|47
                       75|29
                       61|13
                       75|53
                       29|13
                       97|29
                       53|29
                       61|53
                       97|53
                       61|29
                       47|13
                       75|47
                       97|75
                       47|61
                       75|61
                       47|29
                       75|13
                       53|13
                       75,47,61,53,29
                       97,61,53,29,13
                       75,29,13
                       75,97,47,61,53
                       61,13,29
                       97,13,75,29,47"#;
        let result = run(input.lines().map(|s| Ok::<_, Infallible>(s.to_owned()))).unwrap();
        assert_eq!(result, 123)
    }
}
