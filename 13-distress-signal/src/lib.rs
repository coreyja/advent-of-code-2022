use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Packet {
    items: Vec<ListOrInteger>,
}

impl Packet {
    fn parse(input: &str) -> Self {
        let json: serde_json::Value = serde_json::from_str(input).unwrap();

        let items = json
            .as_array()
            .expect("The top level packet is always supposed to be an array")
            .iter()
            .map(ListOrInteger::from_json)
            .collect();

        Self { items }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ListOrInteger {
    List(Vec<ListOrInteger>),
    Integer(u64),
}

impl ListOrInteger {
    fn from_json(input: &Value) -> Self {
        match input {
            Value::Number(x) => Self::Integer(x.as_u64().expect("Should always fit in a u64")),
            Value::Array(a) => Self::List(a.iter().map(Self::from_json).collect()),
            _ => panic!("Unsupported JSON"),
        }
    }
}

impl PartialOrd for ListOrInteger {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Integer(l), Self::Integer(r)) => l.partial_cmp(r),
            (Self::List(l), Self::List(r)) => {
                let max_i = l.len().min(r.len());

                for i in 0..max_i {
                    let i_l = &l[i];
                    let i_r = &r[i];

                    let res = i_l.partial_cmp(i_r).unwrap();

                    match res {
                        std::cmp::Ordering::Equal => continue,
                        x => return Some(x),
                    }
                }

                Some(l.len().cmp(&r.len()))
            }
            (Self::Integer(l), Self::List(r)) => {
                Self::List(vec![Self::Integer(*l)]).partial_cmp(&Self::List(r.clone()))
            }
            (Self::List(l), Self::Integer(r)) => {
                Self::List(l.clone()).partial_cmp(&Self::List(vec![Self::Integer(*r)]))
            }
        }
    }
}

impl Ord for ListOrInteger {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PacketPair {
    left: Packet,
    right: Packet,
}

impl PacketPair {
    fn parse(input: &str) -> Self {
        let mut split = input.lines();

        let left = split.next().unwrap();
        let left = Packet::parse(left);

        let right = split.next().unwrap();
        let right = Packet::parse(right);

        Self { left, right }
    }
}

pub fn part_1(input: &str) -> usize {
    let packet_pairs: Vec<_> = input.split("\n\n").map(PacketPair::parse).collect();

    packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            if pair.left < pair.right {
                Some(i + 1)
            } else {
                None
            }
        })
        .sum()
}

pub fn part_2(input: &str) -> usize {
    let input = input.replace("\n\n", "\n");
    let mut packets: Vec<_> = input.trim().split('\n').map(Packet::parse).collect();

    let first_divider = Packet::parse("[[2]]");
    let second_divider = Packet::parse("[[6]]");

    packets.push(first_divider.clone());
    packets.push(second_divider.clone());

    packets.sort();

    let first_idx = packets.iter().position(|x| &first_divider == x).unwrap() + 1;
    let second_idx = packets.iter().position(|x| &second_divider == x).unwrap() + 1;

    first_idx * second_idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input() {
        let input = include_str!("example.input");
        let ans = part_1(input);

        assert_eq!(ans, 13);
    }

    #[test]
    fn part_1_my_input() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 5390);
    }

    #[test]
    fn part_2_example_input() {
        let input = include_str!("example.input");
        let ans = part_2(input);

        assert_eq!(ans, 140);
    }

    #[test]
    fn part_2_my_input() {
        let input = include_str!("my.input");
        let ans = part_2(input);

        assert_eq!(ans, 19261);
    }
}
