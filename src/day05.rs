fn read(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let (state, instructions) = input.split_once("\n\n").unwrap();

    let mut state = state.lines().rev();
    state.next().unwrap(); // header

    let state = state
        .map(|row| {
            row.chars()
                .skip(1)
                .enumerate()
                .filter_map(|(i, c)| if i % 4 == 0 { Some(c) } else { None })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let state = transpose(state);

    let state = state
        .into_iter()
        .map(|stack| {
            stack
                .into_iter()
                .filter(char::is_ascii_alphanumeric)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let instructions = instructions
        .lines()
        .map(|line| {
            let mut tokens = line.split(' ');

            tokens.next().unwrap(); // move
            let qty = tokens.next().unwrap().parse::<usize>().unwrap();
            tokens.next().unwrap(); // from
            let src = tokens.next().unwrap().parse::<usize>().unwrap();
            tokens.next().unwrap(); // to
            let dst = tokens.next().unwrap().parse::<usize>().unwrap();

            (qty, src, dst)
        })
        .collect::<Vec<_>>();

    (state, instructions)
}


fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

pub fn part1(input: &str) -> String {
    let (mut state, instructions) = read(input);

    instructions
        .into_iter()
        .for_each(|(qty, src, dst)| {
            for _ in 0..qty {
                let src_v = &mut state[src - 1];
                let val = src_v.pop().unwrap();
                let dst_v = &mut state[dst - 1];
                dst_v.push(val);
            }
        });

    state
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect::<String>()
}

pub fn part2(input: &str) -> String {
    let (mut state, instructions) = read(input);

    instructions
        .into_iter()
        .for_each(|(qty, src, dst)| {
            let mut transfer_stack: Vec<char> = Vec::with_capacity(qty);
            for _ in 0..qty {
                let src_v = &mut state[src - 1];
                let val = src_v.pop().unwrap();
                transfer_stack.push(val)
            }

            for _ in 0..qty {
                let val = transfer_stack.pop().unwrap();
                let dst_v = &mut state[dst - 1];
                dst_v.push(val);
            }
        });

    state
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect::<String>()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "MCD");
    }
}
