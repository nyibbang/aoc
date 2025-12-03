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
    const SAMPLE: &str = "";

    #[test]
    fn sample() {
        assert_eq!(run(SAMPLE), 42);
    }
}
