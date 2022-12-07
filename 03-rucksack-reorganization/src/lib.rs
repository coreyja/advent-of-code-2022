#[derive(Debug)]
struct Rucksack {
    pub items: Vec<char>,
}

impl Rucksack {
    fn parse(input: &str) -> Self {
        let items = input.chars().collect();

        Self { items }
    }

    fn compartments(&self) -> (Vec<char>, Vec<char>) {
        let midpoint = self.items.len() / 2;
        let first = self.items[0..midpoint].to_vec();
        let second = self.items[midpoint..].to_vec();

        (first, second)
    }

    fn matching_char(&self) -> Option<char> {
        let (first, second) = self.compartments();

        first.into_iter().find(|&c| second.contains(&c))
    }
}

pub fn char_to_score(c: char) -> u64 {
    if !c.is_ascii_alphabetic() {
        panic!("We can't handle things that aren't ascii alphabetic");
    }

    let num = c as u64;
    if c.is_ascii_uppercase() {
        let position = num - 65;

        position + 26 + 1
    } else {
        num - 97 + 1
    }
}

pub fn part_1(input: &str) -> u64 {
    let sacks: Vec<_> = input.trim().lines().map(Rucksack::parse).collect();

    let first_issue = sacks.iter().find(|s| s.matching_char().is_none());
    dbg!(first_issue);

    let matching: Vec<_> = sacks.iter().map(|s| s.matching_char()).collect();
    // dbg!(&matching);

    matching
        .iter()
        .map(|c| {
            let c = c.expect("There should always be a char in both compartments");
            char_to_score(c)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1() {
        let example_input = include_str!("example.input");
        let ans = part_1(example_input);

        assert_eq!(ans, 157)
    }

    #[test]
    fn my_input_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 8085)
    }

    #[test]
    fn char_scores() {
        assert_eq!(char_to_score('a'), 1);
        assert_eq!(char_to_score('z'), 26);
        assert_eq!(char_to_score('A'), 27);
        assert_eq!(char_to_score('Z'), 52);
    }
}
