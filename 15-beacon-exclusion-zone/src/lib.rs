use std::collections::HashSet;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn part_1(input: &str, row: i64) -> usize {
    let parsed = Zone::parse(input);

    let beacons = parsed.beacons();

    // parsed
    //     .safe_locations()
    //     .iter()
    //     .filter(|c| !beacons.contains(c) && c.y == row)
    //     .count()

    let min_x = parsed.min_x();
    let max_x = parsed.max_x();

    (min_x..max_x)
        .filter(|&x| {
            let c = Coord { x, y: row };

            parsed.is_safe(c) && !beacons.contains(&c)
        })
        .count()
}

#[derive(Debug)]
struct Zone {
    sensors: Vec<Sensor>,
}
impl Zone {
    fn parse(input: &str) -> Self {
        let sensors = input.lines().map(|l| Sensor::parse(l)).collect();

        Self { sensors }
    }

    fn safe_locations(&self) -> HashSet<Coord> {
        self.sensors
            .iter()
            .flat_map(|s| s.safe_locations())
            .collect()
    }

    fn beacons(&self) -> HashSet<Coord> {
        self.sensors.iter().map(|s| s.closest_beacon_pos).collect()
    }

    fn min_x(&self) -> i64 {
        self.sensors.iter().map(|s| s.min_x()).min().unwrap()
    }

    fn max_x(&self) -> i64 {
        self.sensors.iter().map(|s| s.max_x()).max().unwrap()
    }

    fn is_safe(&self, c: Coord) -> bool {
        self.sensors.iter().any(|s| s.is_safe(c))
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    closest_beacon_pos: Coord,
}

impl Sensor {
    fn parse(input: &str) -> Sensor {
        let r = regex::Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        let captures = r.captures(input).expect("Each line should match the regex");

        let self_x = captures.get(1).unwrap().as_str().parse().unwrap();
        let self_y = captures.get(2).unwrap().as_str().parse().unwrap();

        let self_pos = Coord {
            x: self_x,
            y: self_y,
        };

        let beacon_x = captures.get(3).unwrap().as_str().parse().unwrap();
        let beacon_y = captures.get(4).unwrap().as_str().parse().unwrap();

        let beacon_pos = Coord {
            x: beacon_x,
            y: beacon_y,
        };

        Self {
            pos: self_pos,
            closest_beacon_pos: beacon_pos,
        }
    }

    fn distance_to_closest_sensor(&self) -> u64 {
        self.pos.dist(self.closest_beacon_pos)
    }

    fn safe_locations(&self) -> Vec<Coord> {
        let max_distance = self.distance_to_closest_sensor();
        let max_distance_i = max_distance as i64;

        let mut safe_locations = vec![];

        for x_diff in -max_distance_i..=max_distance_i {
            for y_diff in -max_distance_i..=max_distance_i {
                let x = self.pos.x + x_diff;
                let y = self.pos.y + y_diff;

                let c = Coord { x, y };

                if self.pos.dist(c) <= max_distance {
                    safe_locations.push(c);
                }
            }
        }

        safe_locations
    }

    fn min_x(&self) -> i64 {
        self.pos.x - self.distance_to_closest_sensor() as i64
    }

    fn max_x(&self) -> i64 {
        self.pos.x + self.distance_to_closest_sensor() as i64
    }

    fn is_safe(&self, c: Coord) -> bool {
        self.pos.dist(c) <= self.distance_to_closest_sensor()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    fn dist(&self, closest_beacon_pos: Coord) -> u64 {
        self.x.abs_diff(closest_beacon_pos.x) + self.y.abs_diff(closest_beacon_pos.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input_part_1() {
        let input = include_str!("example.input");
        let ans = part_1(input, 10);

        assert_eq!(ans, 26);
    }

    #[test]
    fn my_input_part_1() {
        let input = include_str!("my.input");
        let ans = part_1(input, 2000000);

        assert_eq!(ans, 4582667);
    }
}
