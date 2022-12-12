use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(dir: &str) -> Self {
        match dir {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => panic!("Unknown direction"),
        }
    }

    pub(crate) fn all() -> [Direction; 4] {
        [Self::Up, Self::Down, Self::Right, Self::Left]
    }
}

#[derive(Debug)]
struct Movement {
    dir: Direction,
    count: isize,
}

impl Movement {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(' ');

        let dir = parts.next().unwrap();
        let dir = Direction::parse(dir);

        let count = parts.next().unwrap();
        let count = count.parse().unwrap();

        Self { dir, count }
    }
}

#[derive(Debug)]
struct PuzzleInput(Vec<Movement>);

impl PuzzleInput {
    fn parse(input: &str) -> Self {
        Self(input.lines().map(Movement::parse).collect())
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Coord {
    x: isize,
    y: isize,
}

impl Default for Coord {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl Coord {
    fn move_in_dir(&self, dir: &Direction) -> Self {
        match dir {
            Direction::Up => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn distance_to(&self, tail: &Coord) -> usize {
        let x_diff = self.x.abs_diff(tail.x);
        let y_diff = self.y.abs_diff(tail.y);

        x_diff.max(y_diff)
    }

    fn needs_to_move_towards(&self, head: &Coord, d: &Direction) -> bool {
        match d {
            Direction::Up => head.y > self.y,
            Direction::Down => head.y < self.y,
            Direction::Left => head.x < self.x,
            Direction::Right => head.x > self.x,
        }
    }
}

pub fn rope_course<const TAIL_COUNT: usize>(input: &str) -> HashSet<Coord>
where
    [Coord; TAIL_COUNT]: Default,
{
    let input = PuzzleInput::parse(input);

    let mut head: Coord = Default::default();

    assert!(TAIL_COUNT > 0, "Must have at least one tail");
    let mut tails: [Coord; TAIL_COUNT] = Default::default();

    let mut last_tail_visited: HashSet<Coord> = Default::default();
    last_tail_visited.insert(tails[0]);

    for Movement { dir, count } in input.0 {
        for _ in 0..count {
            head = head.move_in_dir(&dir);

            for tail_index in 0..TAIL_COUNT {
                let current_head = if tail_index == 0 {
                    head
                } else {
                    tails[tail_index - 1]
                };

                if current_head.distance_to(&tails[tail_index]) > 1 {
                    for d in Direction::all() {
                        if tails[tail_index].needs_to_move_towards(&current_head, &d) {
                            tails[tail_index] = tails[tail_index].move_in_dir(&d);
                        }
                    }

                    if tail_index == TAIL_COUNT - 1 {
                        last_tail_visited.insert(tails[tail_index]);
                    }
                }
            }
        }
    }

    last_tail_visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let input = include_str!("example.input");
        let ans = rope_course::<1>(input);

        let correct = [
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 1),
            (4, 2),
            (3, 2),
            (2, 2),
            (1, 2),
            (3, 3),
            (4, 3),
            (2, 4),
            (3, 4),
        ];
        assert_eq!(correct.len(), 13);

        for (x, y) in correct {
            let c = Coord { x, y };

            assert!(ans.contains(&c), "Doesn't contain {c:?}");
        }

        let correct: HashSet<_> = correct
            .iter()
            .map(|(x, y)| Coord { x: *x, y: *y })
            .collect();
        let diff: HashSet<_> = ans.difference(&correct).collect();

        dbg!(diff);

        assert_eq!(ans.len(), 13);
    }

    #[test]
    fn test_my_input_part_1() {
        let input = include_str!("my.input");
        let ans = rope_course::<1>(input);

        assert_eq!(ans.len(), 6498);
    }

    #[test]
    fn test_example_nine_tail() {
        let input = include_str!("example.input");
        let ans = rope_course::<9>(input);

        assert_eq!(ans.len(), 1);
    }

    #[test]
    fn test_larger_example_nine_tail() {
        let input = include_str!("larger_example.input");
        let ans = rope_course::<9>(input);

        assert_eq!(ans.len(), 36);
    }

    #[test]
    fn test_my_nine_tail() {
        let input = include_str!("my.input");
        let ans = rope_course::<9>(input);

        assert_eq!(ans.len(), 2531);
    }
}
