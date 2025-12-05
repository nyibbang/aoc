mod day;
use day::*;

fn run(input: &str) -> i64 {
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
        assert_eq!(run(EXAMPLE), 42);
    }
}
