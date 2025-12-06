mod day;
use std::collections::HashMap;

use day::*;

fn run(input: &[u8]) -> u64 {
    let mut columns = HashMap::<_, String>::new();
    let mut operations: Vec<(_, std::ops::Range<usize>)> = Vec::new();
    let mut column_idx = 0;
    for &byte in input {
        if byte == b'\n' {
            column_idx = 0;
            continue;
        }

        match byte {
            b' ' => {
                if let Some(op) = operations.last_mut() {
                    let op_r = &mut op.1;
                    *op_r = (op_r.start)..(op_r.end + 1);
                }
            }
            c if c.is_ascii_digit() => columns.entry(column_idx).or_default().push(c.into()),
            b'*' => operations.push((Op::Mul, column_idx..column_idx + 1)),
            b'+' => operations.push((Op::Add, column_idx..column_idx + 1)),
            _ => unreachable!("input={:?}", char::from(byte)),
        }

        column_idx += 1;
    }

    let mut result = 0;
    for (op, op_range) in operations {
        let numbers = columns
            .iter()
            .filter(|&(col, _)| op_range.contains(col))
            .map(|(_, value)| value.parse::<u64>().unwrap());
        result += match op {
            Op::Add => numbers.sum::<u64>(),
            Op::Mul => numbers.product(),
        };
    }
    result
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
        assert_eq!(run(EXAMPLE), 3263827);
    }
}
