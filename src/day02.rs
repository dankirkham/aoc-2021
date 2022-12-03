use std::cmp::Ordering;

enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl From<char> for Choice {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("invalid char"),
        }
    }
}

fn to_ordering(c: char) -> Ordering {
    match c {
        'X' => Ordering::Less,
        'Y' => Ordering::Equal,
        'Z' => Ordering::Greater,
        _ => panic!("invalid char"),
    }
}

impl Choice {
    pub fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Rock => match other {
                Self::Rock => Ordering::Equal,
                Self::Paper => Ordering::Less,
                Self::Scissors => Ordering::Greater,
            },
            Self::Paper => match other {
                Self::Rock => Ordering::Greater,
                Self::Paper => Ordering::Equal,
                Self::Scissors => Ordering::Less,
            },
            Self::Scissors => match other {
                Self::Rock => Ordering::Less,
                Self::Paper => Ordering::Greater,
                Self::Scissors => Ordering::Equal,
            },
        }
    }

    pub fn target(&self, other: Ordering) -> Self {
        match self {
            Self::Rock => match other {
                Ordering::Equal => Self::Rock,
                Ordering::Less => Self::Scissors,
                Ordering::Greater => Self::Paper,
            },
            Self::Paper => match other {
                Ordering::Greater => Self::Scissors,
                Ordering::Equal => Self::Paper,
                Ordering::Less => Self::Rock,
            },
            Self::Scissors => match other {
                Ordering::Less => Self::Paper,
                Ordering::Greater => Self::Rock,
                Ordering::Equal => Self::Scissors,
            },
        }
    }

    pub fn value(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

pub fn part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|round| {
            let mut choices = round
                .split(" ")
                .map(|v| {
                    v.chars().map(|c| {
                        let c: Choice = c.into();
                        c
                    })
                })
                .flatten();

            let them = match choices.next() {
                Some(them) => them,
                None => return 0,
            };
            let us = choices.next().unwrap();

            match us.cmp(&them) {
                Ordering::Greater => 6 + us.value(),
                Ordering::Equal => 3 + us.value(),
                Ordering::Less => 0 + us.value(),
            }
        })
        .sum();

    format!("{}", result)
}

pub fn part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|round| {
            let mut choices = round.split(" ").map(|v| v.chars()).flatten();

            let them: Choice = choices.next().unwrap().into();
            let outcome = choices.next().unwrap();
            let target = to_ordering(outcome);
            let us = them.target(target);

            match target {
                Ordering::Greater => 6 + us.value(),
                Ordering::Equal => 3 + us.value(),
                Ordering::Less => 0 + us.value(),
            }
        })
        .sum();

    format!("{}", result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "15");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "12");
    }
}
