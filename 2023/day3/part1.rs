#![allow(dead_code)]

mod common;
use common::*;

fn main() {
    let lines = std::io::stdin().lines().map(Result::unwrap).filter(|line| !line.is_empty());
    let total = engine_part_numbers_sum(lines);
    println!("{total}")
}
