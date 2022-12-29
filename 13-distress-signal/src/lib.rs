use serde_json::Value;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, PartialEq)]
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

        assert_eq!(ans, 13);
    }
}
