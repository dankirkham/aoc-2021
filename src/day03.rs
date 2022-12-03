use std::collections::{VecDeque, HashSet};

pub fn part1(input: &str) -> String {
    let result: u32 = input.lines().map(|line| {
        let items: VecDeque<char> = line
            .chars()
            .collect();

        let mut exists_left: HashSet<char> = HashSet::new();
        let mut exists_right: HashSet<char> = HashSet::new();

        if items.len() == 0 {
            return 0;
        }

        let mut left = 0;
        let mut right = items.len() - 1;

        while left < right {
            let l = items[left];
            let r = items[right];

            left += 1;
            right -= 1;

            exists_left.insert(l);
            exists_right.insert(r);
        }

        let collisions: Vec<char> = exists_left.intersection(&exists_right).map(|x| *x).collect();

        let letter = match collisions.get(0) {
            Some(c) => *c,
            None => ' ',
        };

        match letter {
            'a'..='z' => letter as u32 - 'a' as u32 + 1,
            'A'..='Z' => letter as u32 - 'A' as u32 + 27,
            _ => 0,
        }
    })
    .sum();

    format!("{}", result)
}

pub fn part2(input: &str) -> String {
    let result: u32 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|lines| {
            let collisions = lines
                .iter()
                .map(|line| {
                    let items: HashSet<char> = HashSet::from_iter(line.chars());
                    items
                })
                .fold(None, |left, right| match left {
                    None => Some(right),
                    Some(left) => Some(HashSet::from_iter(left.intersection(&right).map(|v| *v))),
                })
                .unwrap();

            if collisions.len() == 0 {
                return 0;
            }

            let letter = *collisions.iter().next().unwrap();
            match letter {
                'a'..='z' => letter as u32 - 'a' as u32 + 1,
                'A'..='Z' => letter as u32 - 'A' as u32 + 27,
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
