use std::collections::HashSet;

use itertools::{peek_nth, PeekingNext};

pub fn part_1(input: &str) -> usize {
    let chars: Vec<_> = input.chars().collect();
    chars
        .windows(4)
        .enumerate()
        .find(|(_, window)| {
            let hash_set: HashSet<_> = window.iter().collect();

            hash_set.len() == window.len()
        })
        .unwrap()
        .0
        + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_input_part_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let ans = part_1(input);

        assert_eq!(ans, 7);
    }

    #[test]
    fn my_input_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 1531);
    }
}
