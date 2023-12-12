mod common;
use common::*;

fn main() {
    const CONSTRAINT: Set = Set {
        red: 12,
        green: 13,
        blue: 14,
    };
    let total: u32 = std::io::stdin()
        .lines()
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let (_, game) = game(&line).unwrap();
            game.is_constrained_by(CONSTRAINT).then_some(game.id)
        })
        .sum();
    println!("{total}");
}
