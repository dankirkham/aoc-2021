use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub enum Direction {
    Forward,
    Down,
    Up,
}

impl Direction {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "forward" => Some(Direction::Forward),
            "down" => Some(Direction::Down),
            "up" => Some(Direction::Up),
            _ => None,
        }
    }
}

pub struct Record {
    direction: Direction,
    distance: i32,
}

impl Record {
    fn from_str(s: &str) -> Option<Self> {
        let mut tokens = s.split(' ');

        let direction_str = tokens.next();
        if let None = direction_str {
            return None;
        }
        let direction = Direction::from_str(direction_str.unwrap());
        if direction.is_none() {
            return None;
        }

        let distance_str = tokens.next();
        if let None = distance_str {
            return None;
        }
        let distance = distance_str.unwrap().parse();
        if let Err(e) = distance {
            println!("Error: {}", e);
            return None;
        }

        Some(Self {
            direction: direction.unwrap(),
            distance: distance.unwrap(),
        })
    }
}

#[derive(Default)]
pub struct State {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

fn main() {
    {
        let lines = read_lines("./input/02.txt").unwrap();
        let state = State::default();

        let next_state = lines
            .map(|val| val.unwrap()) // unwrap result
            .map(|val| -> Record {
                Record::from_str(&val).unwrap()
            }) // parse
            .fold(state, |s, r| -> State {
                match r.direction {
                    Direction::Forward => State {
                        horizontal: s.horizontal + r.distance,
                        depth: s.depth + r.distance * s.aim,
                        aim: s.aim
                    },
                    Direction::Down => State {
                        horizontal: s.horizontal,
                        depth: s.depth,
                        aim: s.aim + r.distance
                    },
                    Direction::Up => State {
                        horizontal: s.horizontal,
                        depth: s.depth,
                        aim: s.aim - r.distance
                    },
                }
            });

        println!("Horizontal: {}", next_state.horizontal);
        println!("Depth: {}", next_state.depth);
        println!("Aim: {}", next_state.aim);
        println!("Product: {}", next_state.horizontal * next_state.depth);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
