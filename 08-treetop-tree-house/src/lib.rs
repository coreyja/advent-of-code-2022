use itertools::iproduct;

#[derive(Debug)]
struct Tree(usize);

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
}

impl Forest {
    fn parse(input: &str) -> Self {
        let trees = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_string().parse().unwrap())
                    .map(Tree)
                    .collect()
            })
            .collect();

        Self { trees }
    }

    pub(crate) fn width(&self) -> usize {
        self.trees[0].len()
    }

    pub(crate) fn height(&self) -> usize {
        self.trees.len()
    }

    fn is_visible(&self, c: &Coord) -> bool {
        self.is_visible_down(c)
            || self.is_visible_up(c)
            || self.is_visible_left(c)
            || self.is_visible_right(c)
    }

    fn is_visible_up(&self, c: &Coord) -> bool {
        let height = self.get_tree(c).0;

        let y = c.1;

        for new_y in y + 1..self.height() {
            let new_c = (c.0, new_y);
            let t = self.get_tree(&new_c);

            if t.0 >= height {
                return false;
            }
        }

        true
    }

    fn is_visible_down(&self, c: &Coord) -> bool {
        let height = self.get_tree(c).0;

        let y = c.1;

        for new_y in 0..y {
            let new_c = (c.0, new_y);
            let t = self.get_tree(&new_c);

            if t.0 >= height {
                return false;
            }
        }

        true
    }

    fn is_visible_left(&self, c: &Coord) -> bool {
        let height = self.get_tree(c).0;

        let x = c.0;

        for new_x in 0..x {
            let new_c = (new_x, c.1);
            let t = self.get_tree(&new_c);

            if t.0 >= height {
                return false;
            }
        }

        true
    }

    fn is_visible_right(&self, c: &Coord) -> bool {
        let height = self.get_tree(c).0;

        let x = c.0;

        for new_x in x + 1..self.width() {
            let new_c = (new_x, c.1);
            let t = self.get_tree(&new_c);

            if t.0 >= height {
                return false;
            }
        }

        true
    }

    fn get_tree(&self, c: &Coord) -> &Tree {
        &self.trees[c.0][c.1]
    }

    fn scenic_score(&self, c: &Coord) -> usize {
        self.scenic_score_up(c)
            * self.scenic_score_down(c)
            * self.scenic_score_left(c)
            * self.scenic_score_right(c)
    }

    fn scenic_score_down(&self, c: &Coord) -> usize {
        let height = self.get_tree(c).0;

        let y = c.0;
        let mut range = y + 1..self.height();
        let range_length = range.len();

        let distance_to_first_blocked = range.position(|new_y| {
            let new_c = (new_y, c.1);
            let t = self.get_tree(&new_c);

            t.0 >= height
        });

        if let Some(distance_to_first_blocked) = distance_to_first_blocked {
            distance_to_first_blocked + 1
        } else {
            range_length
        }
    }

    fn scenic_score_up(&self, c: &Coord) -> usize {
        let height = self.get_tree(c).0;

        let y = c.0;
        let mut range: Vec<_> = (0..y).collect();
        range.reverse();
        let range_length = range.len();

        let distance_to_first_blocked = range.into_iter().position(|new_y| {
            let new_c = (new_y, c.1);
            let t = self.get_tree(&new_c);

            t.0 >= height
        });

        if let Some(distance_to_first_blocked) = distance_to_first_blocked {
            distance_to_first_blocked + 1
        } else {
            range_length
        }
    }

    fn scenic_score_right(&self, c: &Coord) -> usize {
        let height = self.get_tree(c).0;

        let x = c.1;
        let range = x + 1..self.width();
        let range_length = range.len();

        let distance_to_first_blocked = range.into_iter().position(|new_x| {
            let new_c = (c.0, new_x);
            let t = self.get_tree(&new_c);

            t.0 >= height
        });

        if let Some(distance_to_first_blocked) = distance_to_first_blocked {
            distance_to_first_blocked + 1
        } else {
            range_length
        }
    }

    fn scenic_score_left(&self, c: &Coord) -> usize {
        let height = self.get_tree(c).0;

        let x = c.1;
        let mut range: Vec<_> = (0..x).collect();
        range.reverse();
        let range_length = range.len();

        let distance_to_first_blocked = range.into_iter().position(|new_x| {
            let new_c = (c.0, new_x);
            let t = self.get_tree(&new_c);

            t.0 >= height
        });

        if let Some(distance_to_first_blocked) = distance_to_first_blocked {
            distance_to_first_blocked + 1
        } else {
            range_length
        }
    }
}

type Coord = (usize, usize);

pub fn part_1(input: &str) -> usize {
    let forest = Forest::parse(input);

    let width = forest.width();
    let height = forest.height();

    let mut visible_coords: Vec<Coord> = vec![];

    for i in 0..width {
        for j in 0..height {
            let c = (i, j);
            if forest.is_visible(&c) {
                visible_coords.push(c);
            }
        }
    }

    visible_coords.len()
}

pub fn part_2(input: &str) -> usize {
    let forest = Forest::parse(input);

    let width = forest.width();
    let height = forest.height();

    iproduct!(0..width, 0..height)
        .map(|c| forest.scenic_score(&c))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_pasing() {
        let input = include_str!("example.input");
        let forest = Forest::parse(input);

        // (y, x)
        // Starting from top left as (0,0)
        assert_eq!(forest.get_tree(&(3, 2)).0, 5);
        assert_eq!(forest.get_tree(&(1, 2)).0, 5);
        assert_eq!(forest.get_tree(&(1, 3)).0, 1);
        assert_eq!(forest.get_tree(&(2, 2)).0, 3);
    }

    #[test]
    fn example_part_1() {
        let input = include_str!("example.input");
        let ans = part_1(input);

        assert_eq!(ans, 21);
    }

    #[test]
    fn my_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input);

        assert_eq!(ans, 1695);
    }

    #[test]
    fn test_scenic_score_up_exmaple() {
        let input = include_str!("example.input");
        let forest = Forest::parse(input);

        let score = forest.scenic_score_up(&(1, 2));
        assert_eq!(score, 1);
    }

    #[test]
    fn test_scenic_score_down_exmaple() {
        let input = include_str!("example.input");
        let forest = Forest::parse(input);

        let score = forest.scenic_score_down(&(1, 2));
        assert_eq!(score, 2);
    }

    #[test]
    fn test_scenic_score_left_exmaple() {
        let input = include_str!("example.input");
        let forest = Forest::parse(input);

        let score = forest.scenic_score_left(&(1, 2));
        assert_eq!(score, 1);
    }

    #[test]
    fn test_scenic_score_right_exmaple() {
        let input = include_str!("example.input");
        let forest = Forest::parse(input);

        let score = forest.scenic_score_right(&(1, 2));
        assert_eq!(score, 2);
    }

    #[test]
    fn test_scenic_score_example() {
        let input = include_str!("example.input");
        let forest = Forest::parse(input);

        assert_eq!(forest.scenic_score(&(1, 2)), 4);
        assert_eq!(forest.scenic_score(&(3, 2)), 8);
    }

    #[test]
    fn example_part_2() {
        let input = include_str!("example.input");
        let ans = part_2(input);

        assert_eq!(ans, 8);
    }

    #[test]
    fn my_part_2() {
        let input = include_str!("my.input");
        let ans = part_2(input);

        assert_eq!(ans, 287040);
    }
}
