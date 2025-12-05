mod day;
use day::*;
use nom::Finish;

fn run(input: &str) -> usize {
    let db = database(input).finish().unwrap().1;
    db.ingredients
        .into_iter()
        .filter(|ingr| db.fresh_ranges.iter().any(|range| range.contains(ingr)))
        .count()
}

fn main() {
    println!("{}", run(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use day::tests::*;

    #[test]
    fn example() {
        assert_eq!(run(EXAMPLE), 3);
    }
}
