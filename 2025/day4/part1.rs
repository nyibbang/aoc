mod day;
use day::*;

fn run<const SIZE: usize>(input: &str) -> usize {
    let mut grid: Grid<SIZE> = input.parse().expect("bad input");
    let mut total = 0;
    loop {
        let mut removed_paper = 0;
        for pos in Grid::positions() {
            if grid.is_accessible_paper(pos) {
                grid[pos] = Cell::Empty;
                removed_paper += 1;
            }
        }
        if removed_paper == 0 {
            break total;
        }
        total += removed_paper;
    }
}

fn main() {
    println!("{}", run::<INPUT_SIZE>(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use day::tests::*;

    #[test]
    fn sample() {
        assert_eq!(run::<10>(SAMPLE), 43);
    }
}
