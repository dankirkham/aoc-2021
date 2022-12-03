use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let items: Vec<char> = line.chars().collect();

            let length = items.len();
            if length == 0 {
                return 0;
            }

            let middle = length / 2;
            let left = &items[0..middle];
            let right = &items[middle..length];
            let exists_left: HashSet<char> = HashSet::from_iter(left.iter().map(|x| *x));
            let exists_right: HashSet<char> = HashSet::from_iter(right.iter().map(|x| *x));

            let letter = exists_left
                .intersection(&exists_right)
                .map(|x| *x)
                .next()
                .unwrap_or(' ');

            match letter {
                'a'..='z' => letter as u32 - 96,
                'A'..='Z' => letter as u32 - 38,
                _ => 0,
            }
        })
        .sum();

    format!("{}", result)
}

pub fn part2(input: &str) -> String {
    let rucksacks: Vec<&str> = input.lines().collect::<Vec<_>>();

    let result: u32 = rucksacks
        .chunks(3)
        .map(|lines| {
            let collisions = lines
                .iter()
                .map(|line| HashSet::from_iter(line.chars()))
                .fold(None, |left, right: HashSet<char>| match left {
                    None => Some(right),
                    Some(left) => Some(HashSet::from_iter(left.intersection(&right).map(|v| *v))),
                })
                .unwrap();

            if collisions.len() == 0 {
                return 0;
            }

            let letter = *collisions.iter().next().unwrap();
            match letter {
                'a'..='z' => letter as u32 - 96,
                'A'..='Z' => letter as u32 - 38,
                _ => 0,
            }
        })
        .sum();

    format!("{}", result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "157");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "70");
    }
}
