mod day;
use day::*;
use nom::{
    AsChar, IResult, Parser,
    character::complete::{space0, space1, u64},
    multi::separated_list1,
    sequence::preceded,
};

fn run(input: &[u8]) -> u64 {
    let problems = problems(input).unwrap().1;
    let mut res = 0;
    for (col, op) in problems.operations.iter().enumerate() {
        let numbers_col = problems
            .numbers
            .iter()
            .map(|numbers_entry| numbers_entry[col]);
        res += match op {
            Op::Add => numbers_col.sum::<u64>(),
            Op::Mul => numbers_col.product(),
        };
    }
    res
}

pub fn numbers(s: &[u8]) -> IResult<&[u8], Vec<u64>> {
    preceded(space0, separated_list1(space1, u64)).parse_complete(s)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Problems {
    pub numbers: Vec<Vec<u64>>,
    pub operations: Vec<Op>,
}

pub fn problems(s: &[u8]) -> IResult<&[u8], Problems> {
    let mut numbers = Vec::new();
    for line in s.split(|c| c.is_newline()) {
        match self::numbers(line) {
            Ok((_, numbers_entry)) => numbers.push(numbers_entry),
            Err(_) => {
                let operations = operations(line).unwrap().1;
                return Ok((
                    &[],
                    Problems {
                        numbers,
                        operations,
                    },
                ));
            }
        }
    }
    panic!("no operations line");
}

fn main() {
    println!("{}", run(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    use day::tests::*;
    use nom::Finish;

    #[test]
    fn example() {
        assert_eq!(run(EXAMPLE), 4277556);
    }

    #[test]
    fn parse_example() {
        assert_eq!(
            problems(EXAMPLE).finish().unwrap().1,
            Problems {
                numbers: vec![
                    vec![123, 328, 51, 64],
                    vec![45, 64, 387, 23],
                    vec![6, 98, 215, 314],
                ],
                operations: vec![Op::Mul, Op::Add, Op::Mul, Op::Add]
            }
        );
    }
}
