use rayon::prelude::*;
use std::collections::HashSet;

#[inline(never)]
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

            parsed.cannot_contain_beacon(c) && !beacons.contains(&c)
        })
        .count()
}

#[inline(never)]
pub fn part_2<const N: usize>(input: &str, max: i64) -> usize {
    let parsed = Zone::parse(input);

    let mut cannont_contain_sensor: [bool; N] = [false; N];

    for s in parsed.sensors {
        s.mark_no_beacon(&mut cannont_contain_sensor, max);
    }

    let pos = cannont_contain_sensor
        .iter()
        .position(|&x| x == false)
        .unwrap();

    let x = pos / (max as usize);
    let y = pos % (max as usize);

    x * 4000000 + y

    // parsed
    //     .safe_locations()
    //     .iter()
    //     .filter(|c| !beacons.contains(c) && c.y == row)
    //     .count()

    // let xs: Vec<_> = (0..max).collect();

    // let options: Vec<_> = xs
    //     .iter()
    //     .filter_map(|&x| {
    //         for y in 0..max {
    //             let c = Coord { x, y };

    //             if !parsed.cannot_contain_beacon(c) {
    //                 dbg!(c);
    //                 return Some(c.x * 4000000 + c.y);
    //             }
    //         }

    //         None
    //     })
    //     .collect();

    // assert_eq!(options.len(), 1);

    // options[0]
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

    #[inline(never)]
    fn safe_locations(&self) -> HashSet<Coord> {
        self.sensors
            .iter()
            .flat_map(|s| s.safe_locations())
            .collect()
    }

    #[inline(never)]
    fn beacons(&self) -> HashSet<Coord> {
        self.sensors.iter().map(|s| s.closest_beacon_pos).collect()
    }

    #[inline(never)]
    fn min_x(&self) -> i64 {
        self.sensors.iter().map(|s| s.min_x()).min().unwrap()
    }

    #[inline(never)]
    fn max_x(&self) -> i64 {
        self.sensors.iter().map(|s| s.max_x()).max().unwrap()
    }

    #[inline(never)]
    fn cannot_contain_beacon(&self, c: Coord) -> bool {
        self.sensors.iter().any(|s| s.cannot_contain_beacon(c))
    }
}

#[derive(Debug)]
struct Sensor {
    pos: Coord,
    closest_beacon_pos: Coord,
    disance_to_closest_beacon: u64,
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

        let distance_to_closest_beacon = self_pos.dist(beacon_pos);

        Self {
            pos: self_pos,
            closest_beacon_pos: beacon_pos,
            disance_to_closest_beacon: distance_to_closest_beacon,
        }
    }

    #[inline(never)]
    fn distance_to_closest_beacon(&self) -> u64 {
        self.disance_to_closest_beacon
    }

    #[inline(never)]
    fn safe_locations(&self) -> Vec<Coord> {
        let max_distance = self.distance_to_closest_beacon();
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

    #[inline(never)]
    fn min_x(&self) -> i64 {
        self.pos.x - self.distance_to_closest_beacon() as i64
    }

    #[inline(never)]
    fn max_x(&self) -> i64 {
        self.pos.x + self.distance_to_closest_beacon() as i64
    }

    #[inline(never)]
    fn cannot_contain_beacon(&self, c: Coord) -> bool {
        self.pos.dist(c) <= self.distance_to_closest_beacon()
    }

    fn mark_no_beacon<const N: usize>(
        &self,
        cannot_contain_beacon: &mut [bool; N],
        max_value: i64,
    ) {
        let max_distance = self.distance_to_closest_beacon();
        let max_distance_i = max_distance as i64;

        for x_diff in -max_distance_i..=max_distance_i {
            let max_y = max_distance_i - x_diff.abs();

            for y_diff in -max_y..=max_y {
                let x = self.pos.x + x_diff;
                let y = self.pos.y + y_diff;

                if x >= 0 && x < max_value && y >= 0 && y < max_value {
                    cannot_contain_beacon[((x * max_value) + y) as usize] = true;
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Coord {
    #[inline(never)]
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

    #[test]
    fn example_input_part_2() {
        let input = include_str!("example.input");
        let ans = part_2::<{ 20 * 20 }>(input, 20);

        assert_eq!(ans, 56000011);
    }

    #[test]
    fn my_input_part_2() {
        let input = include_str!("my.input");
        let ans = part_2::<{ 4000000 * 4000000 }>(input, 4000000);

        assert_eq!(ans, 56000011);
    }
}
