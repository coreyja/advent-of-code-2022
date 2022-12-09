use std::collections::HashSet;

pub fn end_of_packet_indicator<const WINDOW_SIZE: usize>(input: &str) -> usize {
    let chars: Vec<_> = input.chars().collect();
    chars
        .windows(WINDOW_SIZE)
        .enumerate()
        .find(|(_, window)| {
            let hash_set: HashSet<_> = window.iter().collect();

            hash_set.len() == window.len()
        })
        .unwrap()
        .0
        + WINDOW_SIZE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1_input_part_1() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let ans = end_of_packet_indicator::<4>(input);

        assert_eq!(ans, 7);
    }

    #[test]
    fn my_input_part_1() {
        let input = include_str!("my.input");
        let ans = end_of_packet_indicator::<4>(input);

        assert_eq!(ans, 1531);
    }

    #[test]
    fn example_1_input_part_2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let ans = end_of_packet_indicator::<14>(input);

        assert_eq!(ans, 19);
    }

    #[test]
    fn my_input_part_2() {
        let input = include_str!("my.input");
        let ans = end_of_packet_indicator::<14>(input);

        assert_eq!(ans, 2518);
    }
}
