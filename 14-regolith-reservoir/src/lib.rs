use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Coord(u64, u64);

impl Coord {
    fn parse(input: &str) -> Self {
        let mut split = input.split(",").map(|x| x.parse().unwrap());

        let x = split.next().unwrap();
        let y = split.next().unwrap();

        Self(x, y)
    }

    fn next_move(&self, frame: &Frame) -> Option<Coord> {
        let rocks = &frame.rocks;
        let down = self.down();
        let down_left = down.left();
        let down_right = down.right();

        if let Some(floor) = frame.floor {
            if down.1 >= floor {
                return None;
            }
        }

        if !rocks.contains(&down) {
            Some(down)
        } else if !rocks.contains(&down_left) {
            Some(down_left)
        } else if !rocks.contains(&down_right) {
            Some(down_right)
        } else {
            None
        }
    }

    fn down(&self) -> Coord {
        Self(self.0, self.1 + 1)
    }

    fn left(&self) -> Coord {
        Self(self.0 - 1, self.1)
    }

    fn right(&self) -> Coord {
        Self(self.0 + 1, self.1)
    }
}

#[derive(Debug)]
struct RockStructure {
    vertices: Vec<Coord>,
}

impl RockStructure {
    fn parse(input: &str) -> Self {
        let vertices = input
            .split("->")
            .map(|x| x.trim())
            .map(Coord::parse)
            .collect();

        Self { vertices }
    }

    fn rocks(&self) -> impl IntoIterator<Item = Coord> + '_ {
        self.vertices.windows(2).flat_map(|a| {
            let mut coords = vec![];
            let start = a[0];
            let end = a[1];

            let x_diff = start.0.abs_diff(end.0);
            let y_diff = start.1.abs_diff(end.1);

            if x_diff != 0 && y_diff != 0 {
                panic!("This line isn't straight and we don't support that")
            } else if x_diff == 0 {
                // This means we have a vertical line
                let s = start.1.min(end.1);
                let e = start.1.max(end.1);

                for i in s..=e {
                    coords.push(Coord(start.0, i));
                }
            } else {
                // This is a horizontal line
                let s = start.0.min(end.0);
                let e = start.0.max(end.0);

                for i in s..=e {
                    coords.push(Coord(i, start.1));
                }
            }

            coords
        })
    }
}

#[derive(Debug)]
struct Maze {
    rocks: Vec<RockStructure>,
}

impl Maze {
    fn parse(input: &str) -> Self {
        let rocks = input.lines().map(RockStructure::parse).collect();

        Self { rocks }
    }

    /// Returns a HashSet of Coordinates that represent the starting rocks for the maze
    /// built from the lines in the vertices
    fn initial_rocks(&self) -> HashSet<Coord> {
        self.rocks.iter().flat_map(|r| r.rocks()).collect()
    }
}

pub fn part_1(input: &str) -> u64 {
    let parsed = Maze::parse(input);

    let mut current: Frame = parsed.into_frame_without_floor();
    let mut count = 0;

    while let Some(f) = current.next() {
        current = f;
        count += 1;
    }

    count
}

pub fn part_2(input: &str) -> u64 {
    let parsed = Maze::parse(input);

    let mut current: Frame = parsed.into_frame_with_floor();
    let mut count = 0;

    while let Some(f) = current.next() {
        current = f;
        count += 1;
    }

    count
}

#[derive(Debug)]
struct Frame {
    turn: usize,
    rocks: HashSet<Coord>,
    floor: Option<u64>,
}

impl Maze {
    fn into_frame_without_floor(self) -> Frame {
        let rocks = self.initial_rocks();

        Frame {
            turn: 0,
            rocks,
            floor: None,
        }
    }

    fn into_frame_with_floor(self) -> Frame {
        let rocks = self.initial_rocks();
        let max_y = rocks.iter().map(|c| c.1).max().unwrap();
        let floor = max_y + 2;

        Frame {
            rocks,
            turn: 0,
            floor: Some(floor),
        }
    }
}

const SAND_START: Coord = Coord(500, 0);

impl Frame {
    const DISTANCE_AFTER_HIGHEST_POINT_TO_CONSIDER_BEFORE_THE_INFINITE_VOID: u64 = 5;

    fn next(self) -> Option<Frame> {
        let mut sand = SAND_START;

        if self.rocks.contains(&SAND_START) {
            return None;
        }

        while let Some(s) = sand.next_move(&self) {
            sand = s;

            // There is an infinite loop here if the sand falls forever
            // But if we can say for sure the sand is below all rocks,
            // we know it is going to keep falling and can be done
            if sand.1 > self.highest_point_possible() {
                return None;
            }
        }

        let mut new_rocks = self.rocks;
        new_rocks.insert(sand);

        Some(Frame {
            turn: self.turn + 1,
            rocks: new_rocks,
            floor: self.floor,
        })
    }

    fn highest_point_possible(&self) -> u64 {
        self.rocks.iter().map(|x| x.1).max().unwrap()
            + Self::DISTANCE_AFTER_HIGHEST_POINT_TO_CONSIDER_BEFORE_THE_INFINITE_VOID
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1() {
        let input = include_str!("example.input");
        let ans = part_1(input);

        assert_eq!(ans, 24);
    }

    #[test]
    fn my_input_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 961);
    }

    #[test]
    fn test_rocks_example() {
        let input = include_str!("example.input");
        let parsed = Maze::parse(input);

        assert_eq!(parsed.initial_rocks().len(), 20);
    }

    #[test]
    fn example_input_part_2() {
        let input = include_str!("example.input");
        let ans = part_2(input);

        assert_eq!(ans, 93);
    }

    #[test]
    fn my_input_part_2() {
        let input = include_str!("my.input");
        let ans = part_2(input);

        assert_eq!(ans, 26375);
    }
}
