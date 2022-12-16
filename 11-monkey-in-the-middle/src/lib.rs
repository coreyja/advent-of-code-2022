use std::ops::{Add, Div, Mul, Rem, Sub};

use num_bigint::BigUint;

#[derive(Debug, Clone, PartialEq, Default)]
struct WorryLevel(BigUint);

impl Add<WorryLevel> for WorryLevel {
    type Output = WorryLevel;

    fn add(self, rhs: WorryLevel) -> Self::Output {
        (self.0 + rhs.0).into()
    }
}

impl Sub<WorryLevel> for WorryLevel {
    type Output = WorryLevel;

    fn sub(self, rhs: WorryLevel) -> Self::Output {
        (self.0 - rhs.0).into()
    }
}

impl Mul<WorryLevel> for WorryLevel {
    type Output = WorryLevel;

    fn mul(self, rhs: WorryLevel) -> Self::Output {
        (self.0 * rhs.0).into()
    }
}

impl Div<WorryLevel> for WorryLevel {
    type Output = WorryLevel;

    fn div(self, rhs: WorryLevel) -> Self::Output {
        (self.0 / rhs.0).into()
    }
}

impl Div<u64> for WorryLevel {
    type Output = WorryLevel;

    fn div(self, rhs: u64) -> Self::Output {
        (self.0 / rhs).into()
    }
}

impl Rem<u64> for WorryLevel {
    type Output = WorryLevel;

    fn rem(self, rhs: u64) -> Self::Output {
        (self.0 % rhs).into()
    }
}

impl From<BigUint> for WorryLevel {
    fn from(val: BigUint) -> Self {
        WorryLevel(val)
    }
}

#[derive(Debug, Clone)]
enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl BinaryOperator {
    fn parse(op: &str) -> BinaryOperator {
        match op {
            "+" => BinaryOperator::Plus,
            "-" => BinaryOperator::Minus,
            "*" => BinaryOperator::Multiply,
            "/" => BinaryOperator::Divide,
            _ => panic!("Operator unknown"),
        }
    }

    fn run(&self, left: WorryLevel, right: WorryLevel) -> Item {
        let worry_level = match self {
            BinaryOperator::Plus => left + right,
            BinaryOperator::Minus => left - right,
            BinaryOperator::Multiply => left * right,
            BinaryOperator::Divide => left / right,
        };

        Item(worry_level)
    }
}

#[derive(Debug, Clone)]
enum OldOrWorryLevel {
    Old,
    Number(WorryLevel),
}

impl OldOrWorryLevel {
    fn parse(input: &str) -> OldOrWorryLevel {
        match input {
            "old" => OldOrWorryLevel::Old,
            x => OldOrWorryLevel::Number(WorryLevel(x.parse().unwrap())),
        }
    }

    fn apply(&self, old: Item) -> WorryLevel {
        match self {
            OldOrWorryLevel::Old => old.0,
            OldOrWorryLevel::Number(x) => x.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Op {
    left: OldOrWorryLevel,
    right: OldOrWorryLevel,
    operator: BinaryOperator,
}

impl Op {
    fn parse(input: &str) -> Op {
        let mut split = input.split(' ');

        let left = split.next().unwrap();
        let op = split.next().unwrap();
        let right = split.next().unwrap();

        let left = OldOrWorryLevel::parse(left);
        let right = OldOrWorryLevel::parse(right);
        let operator = BinaryOperator::parse(op);

        Self {
            left,
            right,
            operator,
        }
    }

    fn run(&self, old: Item) -> Item {
        let left = self.left.apply(old.clone());
        let right = self.right.apply(old);

        self.operator.run(left, right)
    }
}

#[derive(Debug, Clone)]
struct Item(WorryLevel);

#[derive(Debug, Clone)]
struct TestOp(u64);

impl TestOp {
    fn apply(&self, new: Item) -> bool {
        new.0 % self.0 == Default::default()
    }
}

#[derive(Debug, Clone)]
struct Test {
    test_op: TestOp,
    true_index: usize,
    false_index: usize,
}

impl Test {
    fn take_action(&self, forest: &mut Forest, new: Item) {
        let new_idx = if self.test_op.apply(new.clone()) {
            self.true_index
        } else {
            self.false_index
        };

        forest.monkies[new_idx].items.push(new);
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    index: usize,
    items: Vec<Item>,
    operation: Op,
    test: Test,
    inspection_count: usize,
}

impl Monkey {
    fn parse(input: &str) -> Self {
        let regex = regex::Regex::new(
            r"Monkey (?P<monkey_index>\d+):
  Starting items: (?P<starting_items>.*)
  Operation: new = (?P<op>.*)
  Test: divisible by (?P<div>\d+)
    If true: throw to monkey (?P<true_index>\d+)
    If false: throw to monkey (?P<false_index>\d+)",
        )
        .unwrap();

        let captures = regex.captures(input).unwrap();

        let monkey_index = captures.name("monkey_index").unwrap();
        let monkey_index = monkey_index.as_str().parse().unwrap();

        let items = captures.name("starting_items").unwrap();
        let items = items
            .as_str()
            .trim()
            .split(", ")
            .map(|x| Item(WorryLevel(x.parse().unwrap())))
            .collect();

        let operation = captures.name("op").unwrap();
        let operation = Op::parse(operation.as_str());

        let div = captures.name("div").unwrap();
        let div = div.as_str().parse().unwrap();
        let test_op = TestOp(div);

        let true_index = captures.name("true_index").unwrap();
        let true_index = true_index.as_str().parse().unwrap();

        let false_index = captures.name("false_index").unwrap();
        let false_index = false_index.as_str().parse().unwrap();

        let test = Test {
            test_op,
            true_index,
            false_index,
        };

        Self {
            index: monkey_index,
            items,
            operation,
            test,
            inspection_count: 0,
        }
    }
}

#[derive(Debug)]
struct Forest {
    monkies: Vec<Monkey>,
}

impl Forest {
    fn parse(input: &str) -> Self {
        let monkies = input.split("\n\n").map(Monkey::parse).collect();

        Self { monkies }
    }

    fn throw_items<const WORRY_DIVISOR: u64>(&mut self, midx: usize) {
        let monkey = &self.monkies[midx].clone();
        let op = &monkey.operation;
        let div = self.monkies[midx].test.test_op.0;

        let starting_items = &monkey.items;

        for item in starting_items {
            // Inspecting
            self.monkies[midx].inspection_count += 1;

            let new = item.clone();
            let new = op.run(new);

            // Worry level drops after inspection
            let new: Item = Item(new.0 / WORRY_DIVISOR);

            monkey.test.take_action(self, new);
        }

        self.monkies[midx].items.clear();
    }

    fn round<const WORRY_DIVISOR: u64>(&mut self) {
        for i in 0..self.monkies.len() {
            self.throw_items::<WORRY_DIVISOR>(i);
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let mut parsed = Forest::parse(input);

    for _ in 0..20 {
        parsed.round::<3>();
    }

    parsed.monkies.sort_by_key(|m| m.inspection_count);
    parsed.monkies.reverse();

    parsed.monkies[0..2]
        .iter()
        .map(|m| m.inspection_count)
        .reduce(|accum, item| accum * item)
        .unwrap()
}

pub fn part_2(input: &str) -> usize {
    let mut parsed = Forest::parse(input);

    for i in 0..10000 {
        if i % 100 == 0 {
            dbg!(i);
        };
        parsed.round::<1>();
    }

    parsed.monkies.sort_by_key(|m| m.inspection_count);
    parsed.monkies.reverse();

    parsed.monkies[0..2]
        .iter()
        .map(|m| m.inspection_count)
        .reduce(|accum, item| accum * item)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_input() {
        let input = include_str!("example.input");
        let ans = part_1(input);

        assert_eq!(ans, 10605);
    }

    #[test]
    fn test_my_input() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 54752);
    }

    #[test]
    #[ignore = "Would never finish on current hardware"]
    fn test_example_input_part_2() {
        let input = include_str!("example.input");
        let ans = part_2(input);

        assert_eq!(ans, 2713310158);
    }
}
