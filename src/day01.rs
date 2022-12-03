pub fn part1(input: &str) -> String {
    let result = input
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(str::parse::<u32>)
                .map(Result::unwrap_or_default)
                .sum()
        })
        .fold(u32::MIN, u32::max);

    format!("{}", result)
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

    const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "24000");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "45000");
    }
}
