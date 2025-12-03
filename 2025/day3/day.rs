use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub const INPUT: &str = include_str!("input.txt");

pub fn bank(input: &str) -> Vec<u64> {
    input
        .chars()
        .map(|c| c.to_digit(10).expect("not a digit").into())
        .collect()
}

pub fn banks(input: &str) -> Vec<Vec<u64>> {
    input.trim().lines().map(bank).collect()
}

fn largest_bank_joltage(bank: &[u64], digit_idx: usize) -> Option<u64> {
    for digit in (1..=9).rev() {
        if let Some(idx) = bank.iter().position(|&val| val == digit) {
            if digit_idx == 1 {
                return Some(digit);
            }
            if let Some(sub_joltage) = largest_bank_joltage(&bank[idx + 1..], digit_idx - 1) {
                return Some(digit * 10u64.pow((digit_idx - 1).try_into().unwrap()) + sub_joltage);
            }
        }
    }
    None
}

pub fn largest_joltage<const N: usize>(input: &str) -> u64 {
    banks(input)
        .par_iter()
        .map(|bank| largest_bank_joltage(bank, N).unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn largest_bank_joltage() {
        assert_eq!(
            super::largest_bank_joltage(&bank("987654321111111"), 12),
            Some(987654321111)
        );
        assert_eq!(
            super::largest_bank_joltage(&bank("234234234234278"), 12),
            Some(434234234278)
        );
        assert_eq!(
            super::largest_bank_joltage(&bank("818181911112111"), 12),
            Some(888911112111)
        );
    }
}
