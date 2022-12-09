type Stack = Vec<char>;

struct CraneYard {
    stacks: Vec<Stack>,
    rearrangments: Vec<Rearrangments>,
}

impl CraneYard {
    fn parse(input: &str) -> Self {
        let mut split = input.split("\n\n");

        let stacks = split.next().unwrap();
        let rearrangments = split.next().unwrap();

        let mut to_parse: Vec<&str> = stacks.lines().collect();
        let stack_ids = to_parse.pop().unwrap();
        let stack_chars: Vec<char> = stack_ids.chars().collect();

        let stack_count = stack_chars.chunks(4).len();
        to_parse.reverse();

        let mut stacks: Vec<Stack> = vec![vec![]; stack_count];

        for s in to_parse.iter() {
            let chars: Vec<char> = s.chars().collect();

            for (i, chunk) in chars.chunks(4).enumerate() {
                if chunk[0] != '[' {
                    continue;
                }

                stacks[i].push(chunk[1])
            }
        }

        Self {
            rearrangments: rearrangments.lines().map(Rearrangments::parse).collect(),
            stacks,
        }
    }
}

#[derive(PartialEq, Debug)]
struct Rearrangments {
    from: usize,
    to: usize,
    count: usize,
}

impl Rearrangments {
    fn parse(input: &str) -> Rearrangments {
        let r = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        let captures = r
            .captures(input)
            .expect("Each rearrangement line should have this format");

        Self {
            count: captures.get(1).unwrap().as_str().parse().unwrap(),
            from: captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1,
            to: captures.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,
        }
    }
}

pub fn part_1(input: &str) -> String {
    let mut yard = CraneYard::parse(input);

    for r in yard.rearrangments {
        for _ in 0..r.count {
            let from_c = {
                let from = yard.stacks.get_mut(r.from).unwrap();
                from.pop().unwrap()
            };

            let to = yard.stacks.get_mut(r.to).unwrap();

            to.push(from_c);
        }
    }

    yard.stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

pub fn part_2(input: &str) -> String {
    let mut yard = CraneYard::parse(input);

    for r in yard.rearrangments {
        let mut holding_ground = vec![];
        for _ in 0..r.count {
            let from_c = {
                let from = yard.stacks.get_mut(r.from).unwrap();
                from.pop().unwrap()
            };

            holding_ground.push(from_c);
        }

        holding_ground.reverse();

        let to = yard.stacks.get_mut(r.to).unwrap();
        to.append(&mut holding_ground);
    }

    yard.stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1() {
        let input = include_str!("example.input");
        let ans = part_1(input);

        assert_eq!(ans, "CMZ");
    }

    #[test]
    fn test_parsing_rearrangments() {
        let input = include_str!("example.input");
        let yard = CraneYard::parse(input);

        assert_eq!(yard.rearrangments.len(), 4);
        assert_eq!(
            yard.rearrangments[0],
            Rearrangments {
                from: 1,
                to: 0,
                count: 1
            }
        );
        assert_eq!(
            yard.rearrangments[1],
            Rearrangments {
                from: 0,
                to: 2,
                count: 3
            }
        );
        assert_eq!(
            yard.rearrangments[2],
            Rearrangments {
                from: 1,
                to: 0,
                count: 2
            }
        );
        assert_eq!(
            yard.rearrangments[3],
            Rearrangments {
                from: 0,
                to: 1,
                count: 1
            }
        );
    }

    #[test]
    fn test_parsing_stacks() {
        let input = include_str!("example.input");
        let yard = CraneYard::parse(input);

        assert_eq!(yard.stacks.len(), 3);

        assert_eq!(yard.stacks[0], vec!['Z', 'N']);
        assert_eq!(yard.stacks[1], vec!['M', 'C', 'D']);
        assert_eq!(yard.stacks[2], vec!['P']);
    }

    #[test]
    fn my_input_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, "QPJPLMNNR");
    }

    #[test]
    fn example_input_part_2() {
        let input = include_str!("example.input");
        let ans = part_2(input);

        assert_eq!(ans, "MCD");
    }

    #[test]
    fn my_input_part_2() {
        let input = include_str!("my.input");
        let ans = part_2(input);

        assert_eq!(ans, "BQDNWJPVJ");
    }
}
