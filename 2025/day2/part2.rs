mod day;
use day::*;

pub fn is_invalid_id(id: u64) -> bool {
    let id = id.to_string();
    let id_len = id.len();
    for occurences in 2..=id_len {
        if !id_len.is_multiple_of(occurences) {
            continue;
        }
        if id.get(0..(id_len / occurences)).unwrap().repeat(occurences) == id {
            return true;
        }
    }
    false
}

fn main() {
    println!("{}", sum_invalid_ids(INPUT, is_invalid_id));
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn example() {
        assert_eq!(sum_invalid_ids(EXAMPLE, is_invalid_id), 4174379265);
    }
}
