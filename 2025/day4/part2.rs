mod day;
use day::*;

fn run<const SIZE: usize>(input: &str) -> usize {
    let grid: Grid<SIZE> = input.parse().expect("bad input");
    Grid::positions()
        .filter(|&pos| grid[pos] == Cell::Paper && grid.count_paper_neighbors(pos) < 4)
        .count()
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
        assert_eq!(run::<10>(SAMPLE), 13);
    }
}
