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
}

type Coord = (usize, usize);

fn part_1(input: &str) -> usize {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
