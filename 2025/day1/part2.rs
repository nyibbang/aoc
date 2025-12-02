mod day;
use day::*;
use nom::Finish;

fn main() {
    let mut dial = START;
    let mut count = 0;
    INPUT
        .lines()
        .map(|line| {
            Rotation::parse(line)
                .finish()
                .unwrap_or_else(|_| panic!("bad input (not a rotation): {line}"))
                .1
        })
        .for_each(|rot| {
            let (new_dial, over_0) = rot.apply(dial);
            dial = new_dial;
            count += over_0;
        });
    println!("{count}");
}
