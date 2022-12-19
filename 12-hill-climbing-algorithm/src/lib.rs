use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, Copy)]
struct Hill(u8);

impl Hill {
    fn height(&self) -> u8 {
        match self.0 {
            b'S' => b'a',
            b'E' => b'z',
            x => x,
        }
    }
}

#[derive(Debug, Clone)]
struct MountainSide {
    hills: Vec<Vec<Hill>>,
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    const fn all() -> [Dir; 4] {
        [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
    }
}
impl MountainSide {
    fn neighbors(&self, original: Coord) -> impl Iterator<Item = Coord> + '_ {
        let height = self.get(original).height();

        Dir::all()
            .into_iter()
            .map(move |d| original.in_direction(&d))
            .filter(|c| self.is_inbounds(*c))
            .filter(move |x| height + 1 >= self.get(*x).height())
    }

    fn is_inbounds(&self, c: Coord) -> bool {
        c.0 >= 0 && c.0 < self.width() && c.1 >= 0 && c.1 < self.height()
    }

    fn height(&self) -> isize {
        self.hills.len() as isize
    }

    fn width(&self) -> isize {
        self.hills[0].len() as isize
    }

    fn search_for(&self, needle: char) -> Option<Coord> {
        let needle = needle as u8;

        for y in 0..self.height() {
            for x in 0..self.width() {
                let c = Coord(x, y);

                if self.get(c).0 == needle {
                    return Some(c);
                }
            }
        }

        None
    }

    fn get(&self, Coord(x, y): Coord) -> Hill {
        self.hills[y as usize][x as usize]
    }

    fn starting_pos(&self) -> Coord {
        self.search_for('S').unwrap()
    }

    fn target_pos(&self) -> Coord {
        self.search_for('E').unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
struct Coord(isize, isize);

impl Coord {
    fn in_direction(&self, d: &Dir) -> Coord {
        match d {
            Dir::Up => Coord(self.0, self.1 - 1),
            Dir::Down => Coord(self.0, self.1 + 1),
            Dir::Left => Coord(self.0 - 1, self.1),
            Dir::Right => Coord(self.0 + 1, self.1),
        }
    }

    fn manhattan_distance(&self, other: Coord) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ToSearch(Coord, usize);

impl PartialOrd for ToSearch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for ToSearch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PathFrom(Coord, usize);

impl MountainSide {
    fn parse(input: &str) -> MountainSide {
        let hills = input
            .lines()
            .map(|l| l.chars().map(|c| Hill(c as u8)).collect())
            .collect();

        Self { hills }
    }

    fn count_steps(&self) -> usize {
        let starting_pos: Coord = self.starting_pos();
        let target_pos: Coord = self.target_pos();

        let mut paths_from = HashMap::<Coord, PathFrom>::new();

        let mut to_search = BinaryHeap::<ToSearch>::new();

        let mut current_position = ToSearch(starting_pos, 1);

        while current_position.0 != target_pos {
            for n in self.neighbors(current_position.0) {
                let existing_path = paths_from.get(&n);

                let actual_cost = current_position.1 + 1;
                let new_path = if let Some(existing_path) = existing_path {
                    if existing_path.1 <= actual_cost {
                        continue;
                    } else {
                        PathFrom(current_position.0, actual_cost)
                    }
                } else {
                    PathFrom(current_position.0, actual_cost)
                };
                paths_from.insert(n, new_path);

                let dist_estimation = new_path.1 + n.manhattan_distance(target_pos);
                to_search.push(ToSearch(n, dist_estimation));
            }

            current_position = to_search.pop().unwrap();
        }

        let mut actual_path: Vec<Coord> = vec![target_pos];
        let mut curr: Coord = target_pos;

        while curr != starting_pos {
            let prev = paths_from.get(&curr).unwrap().0;
            actual_path.push(prev);
            curr = prev;
        }

        actual_path.reverse();

        dbg!(&actual_path);

        actual_path.len() - 1
    }
}

pub fn part_1(input: &str) -> usize {
    let ms: MountainSide = MountainSide::parse(input);

    ms.count_steps()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1() {
        let input = include_str!("example.input");
        let ans = part_1(input);

        assert_eq!(ans, 31);
    }

    #[test]
    fn my_input_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 31);
    }

    #[test]
    fn to_search_end_example() {
        let input = include_str!("example.input");
        let ms: MountainSide = MountainSide::parse(input);

        assert_eq!(ms.target_pos(), Coord(5, 2));
        assert_eq!(ms.starting_pos(), Coord(0, 0));
    }

    #[test]
    fn example_starting_pos_has_neighbors() {
        let input = include_str!("example.input");
        let ms: MountainSide = MountainSide::parse(input);

        assert_eq!(ms.neighbors(ms.starting_pos()).count(), 2);
    }
}
