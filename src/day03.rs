pub fn part1(input: &str) -> String {
    let result: u32 = input
        .lines()
        .map(|line| {
            let items: Vec<char> = line.chars().collect();

            if items.is_empty() {
                return 0;
            }

            let (left, right) = items.split_at(items.len() / 2);
            let letter = left
                .into_iter()
                .filter(|l| right.into_iter().find(|r| l == r).is_some())
                .cloned()
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
            let chars: Vec<Vec<char>> = lines.iter().map(|&line| line.chars().collect()).collect();

            let letter = *chars
                .get(0)
                .unwrap()
                .iter()
                .filter(|&c| chars[1..].iter().all(|v| v.contains(c)))
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
