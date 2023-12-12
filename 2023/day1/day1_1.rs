fn read_first_last_digits(line: &str) -> (u32, u32) {
    let mut digits = dbg!(line).chars().filter_map(|c| dbg!(c.to_digit(10)));
    let mut first = None;
    let mut last = None;
    while let Some(digit) = digits.next() {
        first.get_or_insert(digit);
        last.replace(digit);
    }

    dbg!((first.unwrap(), last.unwrap()))
}

fn read_calibration_value(line: &str) -> u32 {
    let (first, last) = read_first_last_digits(line);
    first * 10 + last
}

fn main() {
    let lines = std::io::stdin().lines();
    let total: u32 = lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| read_calibration_value(&line))
        .sum();
    println!("{total}");
}
