use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, PartialEq, Default)]
struct WorryLevels(Vec<WorryLevel>);
impl WorryLevels {
    fn divide_all_by<const WORRY_DIVISOR: u64>(&mut self) {
        for x in self.0.iter_mut() {
            x.current /= WORRY_DIVISOR;
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct WorryLevel {
    current: u64,
    divisor: u64,
}
impl WorryLevel {
    fn is_divisible(&self) -> bool {
        self.current % self.divisor == 0
    }
}

impl Add<u64> for WorryLevels {
    type Output = WorryLevels;

    fn add(self, rhs: u64) -> Self::Output {
        let transformed_vec = self
            .0
            .into_iter()
            .map(|WorryLevel { current, divisor }| {
                let new = (current + rhs) % divisor;

                WorryLevel {
                    current: new,
                    divisor,
                }
            })
            .collect();

        Self(transformed_vec)
    }
}

impl Sub<u64> for WorryLevels {
    type Output = WorryLevels;

    fn sub(self, rhs: u64) -> Self::Output {
        let transformed_vec = self
            .0
            .into_iter()
            .map(|WorryLevel { current, divisor }| {
                let new = (current - rhs) % divisor;

                WorryLevel {
                    current: new,
                    divisor,
                }
            })
            .collect();

        Self(transformed_vec)
    }
}

impl Mul<u64> for WorryLevels {
    type Output = WorryLevels;

    fn mul(self, rhs: u64) -> Self::Output {
        let transformed_vec = self
            .0
            .into_iter()
            .map(|WorryLevel { current, divisor }| {
                let new = (current * rhs) % divisor;

                WorryLevel {
                    current: new,
                    divisor,
                }
            })
            .collect();

        Self(transformed_vec)
    }
}

impl Div<u64> for WorryLevels {
    type Output = WorryLevels;

    fn div(self, rhs: u64) -> Self::Output {
        let transformed_vec = self
            .0
            .into_iter()
            .map(|WorryLevel { current, divisor }| {
                let new = (current / rhs) % divisor;

                WorryLevel {
                    current: new,
                    divisor,
                }
            })
            .collect();

        Self(transformed_vec)
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

    fn run(&self, left: u64, right: u64) -> u64 {
        let worry_level = match self {
            BinaryOperator::Plus => left + right,
            BinaryOperator::Minus => left - right,
            BinaryOperator::Multiply => left * right,
            BinaryOperator::Divide => left / right,
        };

        worry_level
    }
}

#[derive(Debug, Clone)]
enum OldOrNumber {
    Old,
    Number(u64),
}

impl OldOrNumber {
    fn parse(input: &str) -> OldOrNumber {
        match input {
            "old" => OldOrNumber::Old,
            x => OldOrNumber::Number(x.parse().unwrap()),
        }
    }

    fn apply(&self, old: u64) -> u64 {
        match self {
            OldOrNumber::Old => old,
            OldOrNumber::Number(x) => x.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct Op {
    left: OldOrNumber,
    right: OldOrNumber,
    operator: BinaryOperator,
}

impl Op {
    fn parse(input: &str) -> Op {
        let mut split = input.split(' ');

        let left = split.next().unwrap();
        let op = split.next().unwrap();
        let right = split.next().unwrap();

        let left = OldOrNumber::parse(left);
        let right = OldOrNumber::parse(right);
        let operator = BinaryOperator::parse(op);

        Self {
            left,
            right,
            operator,
        }
    }

    fn run<const WORRY_DIVISOR: u64>(&self, old: WorryLevels) -> WorryLevels {
        let new: Vec<WorryLevel> = old
            .0
            .into_iter()
            .map(|wl| {
                let left = self.left.apply(wl.current);
                let right = self.right.apply(wl.current);

                let new_wl = self.operator.run(left, right);

                // I don't know why the mod optimization breaks part 1
                // Maybe something with the extra division throws off the
                // Worry levels, but since division is an operation that doesn't
                // totally make sense to me.
                // What ends up working is only doing the optimization if we have
                // a WORRY_DIVISOR of 1, and using the WORRY_DIVISOR if its would impact the
                // worry level
                let new_wl = if WORRY_DIVISOR == 1 {
                    new_wl % wl.divisor
                } else {
                    new_wl / WORRY_DIVISOR
                };

                WorryLevel {
                    current: new_wl,
                    divisor: wl.divisor,
                }
            })
            .collect();

        WorryLevels(new)
    }
}

#[derive(Debug, Clone)]
struct Item(WorryLevels);

#[derive(Debug, Clone)]
struct Test {
    true_index: usize,
    false_index: usize,
}

impl Test {
    fn take_action(&self, forest: &mut Forest, new: Item, midx: usize) {
        let new_idx = if new.0 .0[midx].is_divisible() {
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

#[derive(Debug, Clone)]
struct InitialMonkey {
    index: usize,
    items: Vec<u64>,
    operation: Op,
    test: Test,
    divisor: u64,
}

impl InitialMonkey {
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

        let div = captures.name("div").unwrap();
        let div = div.as_str().parse().unwrap();

        let items = captures.name("starting_items").unwrap();
        let items = items
            .as_str()
            .trim()
            .split(", ")
            .map(|x| x.parse().unwrap())
            .collect();

        let operation = captures.name("op").unwrap();
        let operation = Op::parse(operation.as_str());

        let true_index = captures.name("true_index").unwrap();
        let true_index = true_index.as_str().parse().unwrap();

        let false_index = captures.name("false_index").unwrap();
        let false_index = false_index.as_str().parse().unwrap();

        let test = Test {
            true_index,
            false_index,
        };

        Self {
            index: monkey_index,
            items,
            operation,
            test,
            divisor: div,
        }
    }

    fn into_monkey(self, monkies: &[InitialMonkey]) -> Monkey {
        let items = self
            .items
            .into_iter()
            .map(|item| {
                let worry_levels: Vec<WorryLevel> = monkies
                    .iter()
                    .map(|m| WorryLevel {
                        current: item,
                        divisor: m.divisor,
                    })
                    .collect();

                Item(WorryLevels(worry_levels))
            })
            .collect();

        Monkey {
            index: self.index,
            items,
            operation: self.operation,
            test: self.test,
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
        let monkies: Vec<_> = input.split("\n\n").map(InitialMonkey::parse).collect();

        let monkies = monkies
            .iter()
            .map(|im| im.clone().into_monkey(&monkies))
            .collect();

        Self { monkies }
    }

    fn throw_items<const WORRY_DIVISOR: u64>(&mut self, midx: usize) {
        let monkey = &self.monkies[midx].clone();
        let op = &monkey.operation;

        let starting_items = &monkey.items;

        for item in starting_items {
            self.monkies[midx].inspection_count += 1;

            let new = item.clone();
            let new = op.run::<WORRY_DIVISOR>(new.0);

            monkey.test.take_action(self, Item(new), monkey.index);
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
    fn test_example_input_part_2() {
        let input = include_str!("example.input");
        let ans = part_2(input);

        assert_eq!(ans, 2713310158);
    }

    #[test]
    fn test_my_input_part_2() {
        let input = include_str!("my.input");
        let ans = part_2(input);

        assert_eq!(ans, 13606755504);
    }
}
