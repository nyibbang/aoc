mod day;
use day::*;

fn run(input: &str) -> u64 {
    largest_joltage::<2>(input)
}

fn main() {
    println!("{}", run(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE: &str = "987654321111111\n\
        811111111111119\n\
        234234234234278\n\
        818181911112111";

    #[test]
    fn sample() {
        assert_eq!(run(SAMPLE), 357);
    }
}
