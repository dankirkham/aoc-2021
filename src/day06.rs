use std::collections::VecDeque;

fn to_val(c: char) -> usize {
    c as usize - 97
}

pub fn part1(input: &str) -> String {
    let mut counts: Vec<usize> = vec![0; 26];
    let mut seq: VecDeque<char> = VecDeque::with_capacity(4);

    let mut magic_pos = None;
    for (pos, c) in input.chars().enumerate() {
        if seq.len() >= 4 {
            let dec = seq.pop_front().unwrap();
            counts[to_val(dec)] -= 1;
        }

        counts[to_val(c)] += 1;
        seq.push_back(c);

        if seq.len() >= 4 {
            let pass = seq.iter().fold(true, |pass, val| {
                if !pass {
                    false
                } else {
                    counts[to_val(*val)] == 1
                }
            });

            if pass {
                magic_pos = Some(pos + 1);
                break;
            }
        }
    }

    format!("{}", magic_pos.unwrap())
}

pub fn part2(input: &str) -> String {
    let mut counts: Vec<usize> = vec![0; 26];
    let mut seq: VecDeque<char> = VecDeque::with_capacity(14);

    let mut magic_pos = None;
    for (pos, c) in input.chars().enumerate() {
        if seq.len() >= 14 {
            let dec = seq.pop_front().unwrap();
            counts[to_val(dec)] -= 1;
        }

        counts[to_val(c)] += 1;
        seq.push_back(c);

        if seq.len() >= 14 {
            let pass = seq.iter().fold(true, |pass, val| {
                if !pass {
                    false
                } else {
                    counts[to_val(*val)] == 1
                }
            });

            if pass {
                magic_pos = Some(pos + 1);
                break;
            }
        }
    }

    format!("{}", magic_pos.unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "7");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "19");
    }
}
