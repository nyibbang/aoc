mod day;
use std::ops::RangeInclusive;

use day::*;
use nom::Finish;

fn overlap(lhs: &RangeInclusive<u64>, rhs: &RangeInclusive<u64>) -> bool {
    lhs.contains(rhs.start()) || lhs.contains(rhs.end())
    || rhs.contains(lhs.start()) || rhs.contains(lhs.end())
}

fn try_merge(dst: &mut RangeInclusive<u64>, src: &RangeInclusive<u64>) -> bool {
    if !overlap(dst, src) { return false; }

    *dst = (*dst.start().min(src.start()))..=*(dst.end().max(src.end()));
    true
}

fn run(input: &str) -> u64 {
    let db = database(input).finish().unwrap().1;

    let mut ranges = db.fresh_ranges;
    let mut idx = 0;
    while idx < ranges.len() {
        let src = ranges[idx].clone();
        if ranges.iter_mut().skip(idx + 1).any(|dst| { try_merge(dst, &src) }) {
            ranges.swap_remove(idx);
            idx = 0;
        } else {
            idx += 1;
        }
    }
    ranges.into_iter().map(|r| r.end() - r.start() + 1).sum()
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
        assert_eq!(run(EXAMPLE), 14);
    }
}
