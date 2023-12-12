#[derive(Debug)]
struct DigitsIter<'a> {
    chars: std::str::Chars<'a>,
}

impl<'a> DigitsIter<'a> {
    fn new(line: &'a str) -> Self {
        Self {
            chars: line.chars(),
        }
    }
}

impl<'a> Iterator for DigitsIter<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        const ALPHANUM: [(&str, u32); 9] = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        'iter: loop {
            for alphanum in ALPHANUM {
                if self.chars.as_str().starts_with(alphanum.0) {
                    self.chars.next();
                    break 'iter Some(alphanum.1);
                }
            }

            match self.chars.next() {
                Some(c) => {
                    if let Some(dig) = c.to_digit(10) {
                        break Some(dig);
                    }
                }
                None => break None,
            }
        }
    }
}

fn read_first_last_digits(line: &str) -> (u32, u32) {
    let mut digits = DigitsIter::new(dbg!(line));
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
    dbg!(first * 10 + last)
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
