use nalgebra::base::DMatrix;

fn read(input: &str) -> DMatrix<usize> {
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().chars().count();
    let iter = input
        .lines()
        .map(|line| line.chars().map(|c| c as usize - 48))
        .flatten();
    let mat = DMatrix::from_iterator(nrows, ncols, iter);
    mat
}

fn check_trees(mut depth: usize, iter: impl Iterator<Item = usize>) -> Vec<usize> {
    let mut v: Vec<usize> = vec![1];

    v.extend(iter.map(|new_depth| {
        if depth < new_depth {
            depth = new_depth;
            1_usize
        } else {
            0_usize
        }
    }));

    v.push(1);

    v
}

pub fn part1(input: &str) -> String {
    let forest = read(input);
    let (height, width) = forest.shape();

    let left = forest
        .row_iter()
        .map(|row| {
            let cols = row.columns_range(1..);
            let iter = cols.iter().cloned();

            check_trees(*row.index(0), iter)
        })
        .into_iter()
        .flatten();

    let right = forest
        .row_iter()
        .map(|row| {
            let cols = row.columns_range(..width - 2);
            let iter = cols.iter().cloned().rev();

            check_trees(*row.index(width - 1), iter)
        })
        .into_iter()
        .flatten();

    let top = forest
        .column_iter()
        .map(|column| {
            let rows = column.rows_range(0..);
            let iter = rows.iter().cloned();

            check_trees(*column.index(0), iter)
        })
        .into_iter()
        .flatten();

    let bottom = forest
        .column_iter()
        .map(|column| {
            let rows = column.rows_range(..height - 2);
            let iter = rows.iter().cloned().rev();

            check_trees(*column.index(height - 1), iter)
        })
        .into_iter()
        .flatten();

    let total = left
        .into_iter()
        .zip(right)
        .map(|(a, b)| a + b)
        .zip(top)
        .map(|(a, b)| a + b)
        .zip(bottom)
        .map(|(a, b)| a + b)
        .map(|v| dbg!(v))
        .filter(|&v| v > 0)
        .count();

    format!("{}", total)
}

fn calculate_view(x: usize, y: usize, forest: &DMatrix<usize>) -> usize {
    let (height, width) = forest.shape();

    let left = if x == 0 {
        0
    } else {
        let mut total = 0;
        let depth = *forest.index((y, x));
        let mut x = x - 1;
        loop {
            total += 1;

            if depth <= *forest.index((y, x)) {
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
        let depth = *forest.index((y, x));
        let mut x = x + 1;
        loop {
            total += 1;

            if depth <= *forest.index((y, x)) {
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
        let depth = *forest.index((y, x));
        let mut y = y - 1;
        loop {
            total += 1;

            if depth <= *forest.index((y, x)) {
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
        let depth = *forest.index((y, x));
        let mut y = y + 1;
        loop {
            total += 1;

            if depth <= *forest.index((y, x)) {
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
    let (height, width) = forest.shape();

    let mut scores: Vec<usize> = Vec::with_capacity(height * width);
    for y in 0..height {
        for x in 0..width {
            let score = calculate_view(x, y, &forest);
            scores.push(score);
        }
    }

    let result = scores.into_iter().max().unwrap();

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

    const LARGE_INPUT: &str = include_str!("../input/08.txt");

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

    // #[test]
    // fn test_part1_full() {
    //     let result = part1(LARGE_INPUT);

    //     assert_eq!(result, "1827");
    // }

    #[test]
    fn test_part2_full() {
        let result = part2(LARGE_INPUT);

        assert_eq!(result, "335580");
    }
}
