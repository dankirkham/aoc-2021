use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub trait Bingo {
    fn is_bingo(&self) -> bool;
    fn mark(&mut self, number: &i32) -> bool;
}

#[derive(Default, Copy, Clone)]
pub struct Cell {
    number: i32,
    marked: bool,
}

impl Cell {
    pub fn new(number: i32) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

pub struct CellArray([Cell; 25]);

impl CellArray {
    pub fn from_vec(v: &Vec<i32>) -> Self {
        let mut ca: CellArray = CellArray ([Cell::default(); 25]);
        for i in 0..25 {
            ca.0[i] = Cell::new(*v.get(i).unwrap());
        }
        ca
    }

    pub fn transpose_from(o: &Self) -> Self {
        let mut ca: CellArray = CellArray ([Cell::default(); 25]);
            ca.0 = [
                o.0[0], o.0[5], o.0[10], o.0[15], o.0[20],
                o.0[1], o.0[6], o.0[11], o.0[16], o.0[21],
                o.0[2], o.0[7], o.0[12], o.0[17], o.0[22],
                o.0[3], o.0[8], o.0[13], o.0[18], o.0[23],
                o.0[4], o.0[9], o.0[14], o.0[19], o.0[24],
            ];
            ca
    }
}

impl Bingo for CellArray {
    fn is_bingo(&self) -> bool {
        let rows = [
            &self.0[0..5],
            &self.0[5..10],
            &self.0[10..15],
            &self.0[15..20],
            &self.0[20..25],
        ];

        match rows.iter().find(|row| {
            let is_bingo = row.iter().fold(true, |bingo, cell| {
                if cell.marked && bingo {
                    true
                } else {
                    false
                }
            });

            if is_bingo {
                true
            } else {
                false
            }
        }) {
            Some(_) => true,
            None => false,
        }
    }

    fn mark(&mut self, number: &i32) -> bool {
        self.0.iter_mut().fold(false, |marked, cell| {
            if !marked && cell.number == *number {
                cell.marked = true;
                true
            } else {
                marked
            }
        })
    }
}

pub struct Board {
    board: CellArray,
    transpose_board: CellArray,
}

impl Bingo for Board {
    fn is_bingo(&self) -> bool {
        self.board.is_bingo() || self.transpose_board.is_bingo()
    }

    fn mark(&mut self, number: &i32) -> bool {
        if self.board.mark(number) {
            self.transpose_board.mark(number)
        } else {
            if self.transpose_board.mark(number) {
                self.board.mark(number)
            } else {
                false
            }
        }
    }
}

impl Board {
    pub fn from_str(s: &str) -> Self {
        let numbers: Vec<i32> = s.split(' ')
            .filter(|&x| !x.is_empty())
            .map(|n| n.parse().expect("expected a number"))
            .collect();
        let board = CellArray::from_vec(&numbers);
        let transpose_board = CellArray::transpose_from(&board);

        Board {
            board,
            transpose_board,
        }
    }

    pub fn score(&self) -> i32 {
        self.board.0.iter().fold(0, |score, cell| {
            match cell.marked {
                false => score + cell.number,
                true => score,
            }
        })
    }
}

fn main() {
    {
        let mut lines = read_lines("./input/04.txt").unwrap();

        let numbers: Vec<i32> = lines
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|s| s.parse().expect("Invalid int on first line"))
            .collect();

        let mut boards: Vec<Board> = Vec::new();
        while let Some(_) = lines.next() {
            let mut s = String::new();
            for _ in 0..5 {
                let line = lines.next().expect("File terminated early").unwrap();
                s.push_str(line.trim());
                s.push(' ');
            }

            boards.push(Board::from_str(&s));
        }

        let mut numbers_iter = numbers.iter();

        let mut number: i32 = 0;
        for _ in 0..4 {
            number = *numbers_iter.next().unwrap();
            boards.iter_mut().for_each(|board| {
                board.mark(&number);
            });
        }

        while let Some(n) = numbers_iter.next() {
            number = *n;
            println!("Number: {}", number);
            boards.iter_mut().for_each(|board| {
                board.mark(&number);
            });

            // let bingo = boards.iter().fold(false, |bingo, board| {
            //     bingo || board.is_bingo()
            // });

            // if bingo {
            //     println!("Bingo!");
            //     break;
            // }

            let non_bingos: Vec<&Board> = boards.iter().filter(|board| {
                !board.is_bingo()
            }).collect();

            if non_bingos.len() == 1 {
                break;
            }
        }

        // let winner = boards.iter().find(|board| board.is_bingo()).unwrap();
        // let score = winner.score();
        // println!("Score: {}", score);
        // println!("Last number: {}", number);
        // println!("Product: {}", score * number);

        let loser: &mut Board = boards.iter_mut().find(|board| !board.is_bingo()).unwrap();

        while let Some(n) = numbers_iter.next() {
            number = *n;
            println!("Number: {}", number);
            loser.mark(&number);

            if loser.is_bingo() {
                println!("Reverse Bingo!");
                break;
            }
        }

        println!("Last number: {}", number);
        let score = loser.score();
        println!("Score: {}", score);
        println!("Product: {}", score * number);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
