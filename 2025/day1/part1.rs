mod day;
use day::*;
use nom::Finish;

fn main() {
    let mut dial = START;
    let count = INPUT
        .lines()
        .map(|line| {
            Rotation::parse(line)
                .finish()
                .unwrap_or_else(|_| panic!("bad input (not a rotation): {line}"))
                .1
        })
        .filter(|rot| {
            dial = rot.apply(dial).0;
            dial == 0
        })
        .count();
    println!("{count}");
}
