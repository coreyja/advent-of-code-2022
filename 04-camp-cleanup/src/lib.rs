#[derive(Clone, Debug)]
struct Pair {
    left: Assignment,
    right: Assignment,
}

impl Pair {
    fn parse(input: &str) -> Self {
        let p: Vec<_> = input.split(',').map(Assignment::parse).collect();

        debug_assert_eq!(
            p.len(),
            2,
            "If we got anything but 2 segments here our parsing must be off"
        );

        Self {
            left: p[0].clone(),
            right: p[1].clone(),
        }
    }

    fn is_overlapping(&self) -> bool {
        self.left_contains_right() || self.right_contains_left()
    }

    fn left_contains_right(&self) -> bool {
        self.left.from <= self.right.from && self.left.to >= self.right.to
    }

    fn right_contains_left(&self) -> bool {
        self.right.from <= self.left.from && self.right.to >= self.left.to
    }
}

#[derive(Clone, Debug)]
struct Assignment {
    from: u64,
    to: u64,
}

impl Assignment {
    fn parse(input: &str) -> Self {
        let split: Vec<u64> = input.split('-').map(|s| s.parse().unwrap()).collect();

        debug_assert_eq!(
            split.len(),
            2,
            "If we got anything but 2 segments here our parsing must be off"
        );

        Self {
            from: split[0],
            to: split[1],
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let pairs: Vec<Pair> = input.lines().map(Pair::parse).collect();

    pairs.iter().filter(|p| p.is_overlapping()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1() {
        let input = include_str!("example.input");
        let ans = part_1(input);

        assert_eq!(ans, 2);
    }

    #[test]
    fn my_input_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 444);
    }
}
