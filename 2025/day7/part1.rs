mod day;
use day::*;

fn run(input: &[u8]) -> i64 {
    let grid = Grid::new(input);
    todo!()
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
        assert_eq!(run(EXAMPLE), 21);
    }
}
