use std::collections::{BinaryHeap, HashMap, HashSet};

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
        // [Dir::Up, Dir::Down, Dir::Left, Dir::Right]
        [Dir::Down, Dir::Up, Dir::Right, Dir::Left]
    }

    fn from_move(from: Coord, to: Coord) -> Dir {
        let x_diff = from.0 - to.0;
        let y_diff = from.1 - to.1;

        match (x_diff, y_diff) {
            (-1, 0) => Dir::Right,
            (0, 1) => Dir::Up,
            (1, 0) => Dir::Left,
            (0, -1) => Dir::Down,
            _ => panic!("Impossible Move"),
        }
    }

    fn char(&self) -> char {
        match self {
            Dir::Up => '^',
            Dir::Down => 'v',
            Dir::Left => '<',
            Dir::Right => '>',
        }
    }
}
impl MountainSide {
    fn neighbors(&self, original: Coord) -> impl Iterator<Item = Coord> + '_ {
        let climbable_height = self.get(original).height() + 1;

        Dir::all()
            .into_iter()
            .map(move |d| original.in_direction(&d))
            .filter(|c| self.is_inbounds(*c))
            .filter(move |x| climbable_height >= self.get(*x).height())
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

    fn print_path(&self, path: &[Coord]) {
        let mut chars = HashMap::<Coord, char>::new();

        let moves: Vec<Dir> = path
            .windows(2)
            .map(|window| -> Dir { Dir::from_move(window[0], window[1]) })
            .collect();

        let mut current = self.starting_pos();
        for m in moves {
            chars.insert(current, m.char());

            current = current.in_direction(&m);
        }

        for y in 0..self.height() {
            for x in 0..self.width() {
                let coord = Coord(x, y);
                let char = chars.get(&coord).unwrap_or(&'.');
                print!("{char}")
            }
            println!()
        }
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
struct ToSearch {
    c: Coord,
    estimated_distance: usize,
    actual_cost: usize,
}

impl PartialOrd for ToSearch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other
            .estimated_distance
            .partial_cmp(&self.estimated_distance)
    }
}

impl Ord for ToSearch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimated_distance.cmp(&self.estimated_distance)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct PathFrom {
    from: Coord,
    current_distance: usize,
}

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

        to_search.push(ToSearch {
            c: starting_pos,
            estimated_distance: starting_pos.manhattan_distance(target_pos),
            actual_cost: 0,
        });

        while let Some(current_position) = to_search.pop() {
            if current_position.c == target_pos {
                break;
            }

            for n in self.neighbors(current_position.c) {
                let existing_path = paths_from.get(&n);

                let actual_cost = current_position.actual_cost + 1;
                let new_path = if let Some(existing_path) = existing_path {
                    if existing_path.current_distance <= actual_cost {
                        continue;
                    }
                    PathFrom {
                        from: current_position.c,
                        current_distance: actual_cost,
                    }
                } else {
                    PathFrom {
                        from: current_position.c,
                        current_distance: actual_cost,
                    }
                };
                paths_from.insert(n, new_path);

                let dist_estimation = new_path.current_distance + n.manhattan_distance(target_pos);
                to_search.push(ToSearch {
                    c: n,
                    estimated_distance: dist_estimation,
                    actual_cost: new_path.current_distance,
                });
            }
        }

        let mut actual_path: Vec<Coord> = vec![target_pos];
        let mut curr: Coord = target_pos;

        while curr != starting_pos {
            let prev = paths_from.get(&curr).unwrap().from;
            actual_path.push(prev);
            curr = prev;
        }

        actual_path.reverse();

        // Assert path never repeats
        let path_set: HashSet<_> = actual_path.iter().collect();
        assert_eq!(path_set.len(), actual_path.len());

        self.print_path(&actual_path);

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

        assert_eq!(ans, 425);
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

    #[test]
    fn my_input_to_search_end_example() {
        let input = include_str!("my.input");
        let ms: MountainSide = MountainSide::parse(input);

        assert_eq!(ms.starting_pos(), Coord(0, 20));
        assert_eq!(ms.target_pos(), Coord(91, 20));
    }

    #[test]
    fn my_starting_pos_has_neighbors() {
        let input = include_str!("my.input");
        let ms: MountainSide = MountainSide::parse(input);

        assert_eq!(ms.neighbors(ms.starting_pos()).count(), 3);
    }

    #[test]
    fn my_35_19_has_neighbors() {
        let input = include_str!("my.input");
        let ms: MountainSide = MountainSide::parse(input);

        assert_eq!(ms.neighbors(Coord(35, 19)).count(), 4);
    }
}
