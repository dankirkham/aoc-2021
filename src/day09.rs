use std::{char::ParseCharError, collections::HashSet, str::FromStr};

#[derive(Clone, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ParseCharError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Self::Up),
            "D" => Ok(Self::Down),
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => panic!("invalid direction"),
        }
    }
}

impl Point {
    pub fn is_adjacent(&self, other: &Point) -> bool {
        other.x >= self.x - 1
            && other.x <= self.x + 1
            && other.y >= self.y - 1
            && other.y <= self.y + 1
    }

    pub fn is_colinear(&self, other: &Point) -> bool {
        self.x == other.x || self.y == other.y
    }

    pub fn dir_to(&self, other: &Point) -> Direction {
        if self.x < other.x {
            Direction::Right
        } else if self.x > other.x {
            Direction::Left
        } else if self.y < other.y {
            Direction::Down
        } else if self.y > other.y {
            Direction::Up
        } else {
            panic!("No dir to");
        }
    }

    pub fn mov(&mut self, dir: Direction) {
        match dir {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    pub fn catch_up(&mut self, other: &Point) {
        if other.x > self.x && other.y > self.y {
            // SE
            self.x += 1;
            self.y += 1;
        } else if other.x < self.x && other.y > self.y {
            // SW
            self.x -= 1;
            self.y += 1;
        } else if other.x > self.x && other.y < self.y {
            // NE
            self.x += 1;
            self.y -= 1;
        } else if other.x < self.x && other.y < self.y {
            // NW
            self.x -= 1;
            self.y -= 1;
        } else {
            panic!("unreachable");
        }
    }
}

#[derive(Clone, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Knot {
    pub point: Point,
    pub parent: Option<Box<Knot>>,
}

impl Knot {
    pub fn new(parent: Option<Box<Knot>>) -> Self {
        Self {
            parent,
            ..Default::default()
        }
    }

    pub fn rope(length: usize) -> Self {
        let head = Knot::new(None);
        let mut next = head;

        for _ in 1..length {
            let tail = Knot::new(Some(Box::new(next)));
            next = tail;
        }

        next
    }

    pub fn mov(&mut self, dir: Direction) -> Option<Point> {
        match &mut self.parent {
            None => {
                self.point.mov(dir);
                Some(self.point.clone())
            }
            Some(parent) => {
                if let Some(head) = parent.mov(dir) {
                    if !head.is_adjacent(&self.point) {
                        if head.is_colinear(&self.point) {
                            let dir = self.point.dir_to(&head);
                            self.point.mov(dir);
                        } else {
                            self.point.catch_up(&head);
                        }
                        Some(self.point.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }
}

pub fn sim_rope(input: &str, length: usize) -> String {
    let moves = input.lines().map(|line| {
        let mut tokens = line.split(' ');
        let dir = tokens.next().unwrap().parse::<Direction>().unwrap();
        let val = tokens.next().unwrap().parse::<usize>().unwrap();
        (dir, val)
    });
    let mut visited: HashSet<Point> = HashSet::new();

    let mut tail = Knot::rope(length);
    visited.insert(tail.point.clone());

    moves.for_each(|(dir, val)| {
        for _ in 0..val {
            if let Some(point) = tail.mov(dir) {
                visited.insert(point.clone());
            }
        }
    });

    let result = visited.len();

    format!("{}", result)
}

pub fn part1(input: &str) -> String {
    sim_rope(input, 2)
}

pub fn part2(input: &str) -> String {
    sim_rope(input, 10)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT_LARGE: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    // const LARGE_INPUT: &str = include_str!("../input/09.txt");

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "13");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "1");
    }

    #[test]
    fn test_part2_large() {
        let result = part2(INPUT_LARGE);

        assert_eq!(result, "36");
    }

    // #[test]
    // fn test_part1_full() {
    //     let result = part1(LARGE_INPUT);

    //     assert_eq!(result, "1827");
    // }

    // #[test]
    // fn test_part2_full() {
    //     let result = part2(LARGE_INPUT);

    //     assert_eq!(result, "335580");
    // }
}
