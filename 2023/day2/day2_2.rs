mod common;
use common::*;

fn main() {
    let total: u32 = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (_, game) = game(&line).unwrap();
            game.minimum_set().power()
        })
        .sum();
    println!("{total}");
}
