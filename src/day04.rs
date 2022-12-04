pub fn part1(input: &str) -> String {
    let result = input
        .lines()
        .filter(|line| {
            let mut pair = line.split(",");
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();

            let mut lefts = left.split("-");
            let left_min = lefts.next().unwrap().parse::<u32>().unwrap();
            let left_max = lefts.next().unwrap().parse::<u32>().unwrap();

            let mut rights = right.split("-");
            let right_min = rights.next().unwrap().parse::<u32>().unwrap();
            let right_max = rights.next().unwrap().parse::<u32>().unwrap();

            if left_min >= right_min && left_max <= right_max {
                return true;
            }

            if right_min >= left_min && right_max <= left_max {
                return true;
            }

            false
        })
        .count();

    format!("{}", result)
}

pub fn part2(input: &str) -> String {
    let result = input
        .lines()
        .filter(|line| {
            let mut pair = line.split(",");
            let left = pair.next().unwrap();
            let right = pair.next().unwrap();

            let mut lefts = left.split("-");
            let left_min = lefts.next().unwrap().parse::<u32>().unwrap();
            let left_max = lefts.next().unwrap().parse::<u32>().unwrap();

            let mut rights = right.split("-");
            let right_min = rights.next().unwrap().parse::<u32>().unwrap();
            let right_max = rights.next().unwrap().parse::<u32>().unwrap();

            if left_max >= right_min && left_min <= right_max {
                return true;
            }

            if right_max >= left_min && right_min <= left_max {
                return true;
            }

            false
        })
        .count();

    format!("{}", result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "2");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "4");
    }
}
