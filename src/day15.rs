use std::str::FromStr;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
struct ParseSensorError;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
enum Cell {
    #[default]
    Empty,
    Sensor,
    Beacon,
    NotABeacon,
}

#[derive(Debug)]
struct Map {
    cells: HashMap<Point, Cell>,
}

impl Map {
    fn new(sensors: Vec<Sensor>, row: i64) -> Self {
        let mut cells = HashMap::<Point, Cell>::new();

        sensors.iter().for_each(|sensor| {
            cells.insert(sensor.location, Cell::Sensor);
            cells.insert(sensor.closest_beacon, Cell::Beacon);
        });

        let mut s = Self {
            cells
        };

        sensors.iter().for_each(|sensor| {
            s.mark(sensor.location, sensor.location.distance(&sensor.closest_beacon), row);
        });

        s
    }

    fn mark(&mut self, location: Point, distance: u64, row: i64) {
        let row_point = Point { x: location.x, y: row };

        let row_point_distance = location.distance(&row_point);
        if row_point_distance <= distance {
            match self.cells.get(&row_point) {
                None => {
                    self.cells.insert(row_point, Cell::NotABeacon);
                },
                Some(_) => (),
            }

            // Travel right
            let mut right_point = Point { x: row_point.x + 1, y: row_point.y };
            while right_point.distance(&location) <= distance {
                match self.cells.get(&right_point) {
                    None => {
                        self.cells.insert(right_point, Cell::NotABeacon);
                    },
                    Some(_) => (),
                }

                right_point = Point { x: right_point.x + 1, y: right_point.y };
            }

            // Travel left
            let mut left_point = Point { x: row_point.x - 1, y: row_point.y };
            while left_point.distance(&location) <= distance {
                match self.cells.get(&left_point) {
                    None => {
                        self.cells.insert(left_point, Cell::NotABeacon);
                    },
                    Some(_) => (),
                }

                left_point = Point { x: left_point.x - 1, y: left_point.y };
            }
        }
    }

    fn non_beacons_in_row(&self, row: i64) -> usize {
        self.cells
            .iter()
            .filter(|(k, _)| k.y == row)
            // .filter(|(_, &v)| v == Cell::NotABeacon || v == Cell::Sensor)
            .filter(|(_, &v)| v == Cell::NotABeacon)
            .count()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn distance(&self, other: &Self) -> u64 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u64
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    closest_beacon: Point,
}

impl FromStr for Sensor {
    type Err = ParseSensorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = s.split(|c| " =,:".contains(c)).collect::<Vec<_>>();

        Ok(Sensor {
            location: Point {
                x: tokens[3].parse::<i64>().unwrap(),
                y: tokens[6].parse::<i64>().unwrap(),
            },
            closest_beacon: Point {
                x: tokens[13].parse::<i64>().unwrap(),
                y: tokens[16].parse::<i64>().unwrap(),
            },
        })
    }
}

pub fn part1_row(input: &str, row: i64) -> String {
    let sensors = input
        .lines()
        .map(Sensor::from_str)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    let map = Map::new(sensors, row);

    let result = map.non_beacons_in_row(row);

    format!("{}", result)
}

pub fn part1(input: &str) -> String {
    part1_row(input, 2000000)
}

pub fn part2(input: &str) -> String {
    let result: u32 = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(str::parse::<u32>)
                .map(Result::unwrap_or_default)
                .sum()
        })
        .fold(vec![0; 3], |mut vec, next: u32| {
            vec.push(next);
            vec.sort();
            vec.reverse();
            vec.pop();
            vec
        })
        .into_iter()
        .sum();

    format!("{}", result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_part1() {
        let result = part1_row(INPUT, 10);

        assert_eq!(result, "26");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "45000");
    }
}
