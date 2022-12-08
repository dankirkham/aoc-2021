fn read(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c as usize - 48).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn part1(input: &str) -> String {
    let forest = read(input);
    let height = forest.len();
    let width = forest[0].len();
    let mut visible: Vec<Vec<bool>> = vec![vec![false; width]; height];

    // Left
    for y in 0..height {
        visible[y][0] = true;

        let mut depth = forest[y][0];
        let mut x = 1;
        while x < width {
            if depth < forest[y][x] {
                depth = forest[y][x];
                visible[y][x] = true;
            }
            x += 1;
        }
    }

    // Right
    for y in (0..height).rev() {
        visible[y][width - 1] = true;

        let mut depth = forest[y][width - 1];
        let mut x = width - 2;
        loop {
            if depth < forest[y][x] {
                depth = forest[y][x];
                visible[y][x] = true;
            }
            if x > 0 {
                x -= 1;
            } else {
                break;
            }
        }
    }

    // Top
    for x in 0..width {
        visible[0][x] = true;

        let mut depth = forest[0][x];
        let mut y = 1;
        while y < height {
            if depth < forest[y][x] {
                depth = forest[y][x];
                visible[y][x] = true;
            }
            y += 1;
        }
    }

    // Bottom
    for x in (0..width).rev() {
        visible[height - 1][x] = true;

        let mut depth = forest[height - 1][x];
        let mut y = height - 2;
        loop {
            if depth < forest[y][x] {
                depth = forest[y][x];
                visible[y][x] = true;
            }
            if y > 0 {
                y -= 1;
            } else {
                break;
            }
        }
    }

    let result: usize = visible
        .into_iter()
        .map(|row| row.into_iter().filter(|v| *v).count())
        .sum();
    format!("{}", result)
}

fn calculate_view(x: usize, y: usize, forest: &Vec<Vec<usize>>) -> usize {
    let height = forest.len();
    let width = forest[0].len();

    let left = if x == 0 {
        0
    } else {
        let mut total = 0;
        let depth = forest[y][x];
        let mut x = x - 1;
        loop {
            total += 1;

            if depth <= forest[y][x] {
                break;
            }

            if x == 0 {
                break;
            } else {
                x -= 1;
            }
        }
        total
    };

    let right = if x == width - 1 {
        0
    } else {
        let mut total = 0;
        let depth = forest[y][x];
        let mut x = x + 1;
        loop {
            total += 1;

            if depth <= forest[y][x] {
                break;
            }

            if x == width - 1 {
                break;
            } else {
                x += 1;
            }
        }
        total
    };

    let up = if y == 0 {
        0
    } else {
        let mut total = 0;
        let depth = forest[y][x];
        let mut y = y - 1;
        loop {
            total += 1;

            if depth <= forest[y][x] {
                break;
            }

            if y == 0 {
                break;
            } else {
                y -= 1;
            }
        }
        total
    };

    let down = if y == height - 1 {
        0
    } else {
        let mut total = 0;
        let depth = forest[y][x];
        let mut y = y + 1;
        loop {
            total += 1;

            if depth <= forest[y][x] {
                break;
            }

            if y == height - 1 {
                break;
            } else {
                y += 1;
            }
        }
        total
    };

    left * right * up * down
}

pub fn part2(input: &str) -> String {
    let forest = read(input);
    let height = forest.len();
    let width = forest[0].len();

    let mut scores: Vec<usize> = Vec::with_capacity(height * width);
    for y in 0..height {
        for x in 0..width {
            let score = calculate_view(x, y, &forest);
            scores.push(score);
        }
    }

    let result = scores
        .into_iter()
        .max()
        .unwrap();

    format!("{}", result)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() {
        let result = part1(INPUT);

        assert_eq!(result, "21");
    }

    #[test]
    fn test_part2() {
        let result = part2(INPUT);

        assert_eq!(result, "8");
    }
}
