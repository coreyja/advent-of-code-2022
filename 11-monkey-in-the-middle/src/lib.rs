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

    fn run(&self, left: usize, right: usize) -> Item {
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
enum OldOrIndex {
    Old,
    Idx(usize),
}
impl OldOrIndex {
    fn parse(input: &str) -> OldOrIndex {
        match input {
            "old" => OldOrIndex::Old,
            x => OldOrIndex::Idx(x.parse().unwrap()),
        }
    }

    fn apply(&self, old: Item) -> usize {
        match self {
            OldOrIndex::Old => old.0,
            OldOrIndex::Idx(x) => *x,
        }
    }
}

#[derive(Debug, Clone)]
struct Op {
    left: OldOrIndex,
    right: OldOrIndex,
    operator: BinaryOperator,
}

impl Op {
    fn parse(input: &str) -> Op {
        let mut split = input.split(' ');

        let left = split.next().unwrap();
        let op = split.next().unwrap();
        let right = split.next().unwrap();

        let left = OldOrIndex::parse(left);
        let right = OldOrIndex::parse(right);
        let operator = BinaryOperator::parse(op);

        Self {
            left,
            right,
            operator,
        }
    }

    fn run(&self, old: Item) -> Item {
        let left = self.left.apply(old);
        let right = self.right.apply(old);

        self.operator.run(left, right)
    }
}

#[derive(Debug, Clone, Copy)]
struct Item(usize);

#[derive(Debug, Clone)]
enum TestOp {
    Divisible(usize),
}
impl TestOp {
    fn apply(&self, new: Item) -> bool {
        match self {
            TestOp::Divisible(div) => new.0 % div == 0,
        }
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
        let new_idx = if self.test_op.apply(new) {
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
            .map(|x| Item(x.parse().unwrap()))
            .collect();

        let operation = captures.name("op").unwrap();
        let operation = Op::parse(operation.as_str());

        let div = captures.name("div").unwrap();
        let div = div.as_str().parse().unwrap();
        let test_op = TestOp::Divisible(div);

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

    fn throw_items(&mut self, midx: usize) {
        let monkey = &self.monkies[midx].clone();
        let op = &monkey.operation;

        let starting_items = &monkey.items;

        for item in starting_items {
            // Inspecting
            self.monkies[midx].inspection_count += 1;
            let new = op.run(*item);

            // Worry level drops after inspection
            let new: Item = Item(new.0 / 3);

            monkey.test.take_action(self, new);
        }

        self.monkies[midx].items.clear();
    }

    fn round(&mut self) {
        for i in 0..self.monkies.len() {
            self.throw_items(i);
        }
    }
}

pub fn part_1(input: &str) -> usize {
    let mut parsed = Forest::parse(input);

    for _ in 0..20 {
        parsed.round();
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

        assert_eq!(ans, 10605);
    }
}
