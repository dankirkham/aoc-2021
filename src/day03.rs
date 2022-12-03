use std::collections::HashSet;

pub fn part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let items: Vec<char> = line.chars().collect();

            if items.is_empty() {
                return 0;
            }

            let (left, right) = items.split_at(items.len() / 2);
            let exists_left: HashSet<char> = HashSet::from_iter(left.iter().copied());
            let exists_right: HashSet<char> = HashSet::from_iter(right.iter().copied());

            let letter = exists_left
                .intersection(&exists_right)
                .copied()
                .next()
                .unwrap();

            match letter.is_ascii_uppercase() {
                true => letter as u32 - 38,
                _ => letter as u32 - 96,
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
                    Some(left) => Some(HashSet::from_iter(left.intersection(&right).copied())),
                })
                .unwrap();

            if collisions.is_empty() {
                return 0;
            }

            let letter = *collisions.iter().next().unwrap();
            match letter.is_ascii_uppercase() {
                true => letter as u32 - 38,
                _ => letter as u32 - 96,
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
