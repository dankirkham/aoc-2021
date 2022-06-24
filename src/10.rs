use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Points = u64;

#[derive(PartialEq, Copy, Clone)]
enum Token {
    LeftCurly,
    LeftPointy,
    LeftRound,
    LeftSquare,
    RightCurly,
    RightPointy,
    RightRound,
    RightSquare,
}

impl Token {
    pub fn from_char(c: &char) -> Option<Self> {
        match c {
            '{' => Some(Token::LeftCurly),
            '<' => Some(Token::LeftPointy),
            '(' => Some(Token::LeftRound),
            '[' => Some(Token::LeftSquare),
            '}' => Some(Token::RightCurly),
            '>' => Some(Token::RightPointy),
            ')' => Some(Token::RightRound),
            ']' => Some(Token::RightSquare),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Token::LeftCurly => '{',
            Token::LeftPointy => '<',
            Token::LeftRound => '(',
            Token::LeftSquare => '[',
            Token::RightCurly => '}',
            Token::RightPointy => '>',
            Token::RightRound => ')',
            Token::RightSquare => ']',
        }
    }

    pub fn to_points(&self) -> Points {
        match self {
            Token::LeftCurly => 0,
            Token::LeftPointy => 0,
            Token::LeftRound => 0,
            Token::LeftSquare => 0,
            Token::RightCurly => 3,
            Token::RightPointy => 4,
            Token::RightRound => 1,
            Token::RightSquare => 2,
        }
    }
}

fn parse_1<'a>(iter: &mut impl Iterator<Item = &'a Token>, expected: Option<Token>) -> Option<Points> {
    loop {
        if let Some(t) = iter.next() {
            if let Some(e) = &expected {
                if *t == *e {
                    return None;
                }
            }

            let result = match *t {
                Token::LeftCurly => parse_1(iter, Some(Token::RightCurly)),
                Token::LeftPointy => parse_1(iter, Some(Token::RightPointy)),
                Token::LeftRound => parse_1(iter, Some(Token::RightRound)),
                Token::LeftSquare => parse_1(iter, Some(Token::RightSquare)),
                Token::RightCurly => Some(1197),
                Token::RightPointy => Some(25137),
                Token::RightRound => Some(3),
                Token::RightSquare => Some(57),
            };

            if result.is_some() {
                return result;
            }
        } else {
            if expected.is_none() {
                return None;
            } else {
                // panic!("terminated before we could fail!");
                return None;
            }
        }
    }
}

fn parse_2<'a>(iter: &mut impl Iterator<Item = &'a Token>, bad_tokens: &mut Vec<Token>, expected: Option<Token>) -> bool {
    loop {
        if let Some(t) = iter.next() {
            if let Some(e) = &expected {
                if *t == *e {
                    return false;
                }
            }

            let result = match *t {
                Token::LeftCurly => parse_2(iter, bad_tokens, Some(Token::RightCurly)),
                Token::LeftPointy => parse_2(iter, bad_tokens, Some(Token::RightPointy)),
                Token::LeftRound => parse_2(iter, bad_tokens, Some(Token::RightRound)),
                Token::LeftSquare => parse_2(iter, bad_tokens, Some(Token::RightSquare)),
                _ => parse_2(iter, bad_tokens, expected),
            };

            if result {
                return true;
            }
        } else {
            if expected.is_none() {
                return true;
            } else {
                bad_tokens.push(expected.unwrap());
                return false;
            }
        }
    }
}

fn main() {
    let lines = read_lines("./input/10.txt").unwrap();
    // let lines = read_lines("./input/10-sample.txt").unwrap();

    let all_tokens: Vec<Vec<Token>> = lines
        .map(|val| val.unwrap())
        .map(|line|
            line
                .chars()
                .map(|c| Token::from_char(&c).unwrap())
                .collect::<Vec<Token>>()
        )
        .collect();

    let scores_1: Vec<Points> = all_tokens.iter()
        .map(|tokens| parse_1(&mut tokens.iter(), None))
        .map(|val| val.unwrap_or(0))
        .collect();

    let count_1 = scores_1.iter().fold(0, |a, b| a + b);
    println!("Part 1: {}", count_1);

    let incomplete = all_tokens.iter().zip(scores_1.iter())
        .fold(Vec::<&Vec<Token>>::new(), |mut inc, (tokens, score)| {
            if *score == 0 {
                inc.push(tokens);
            }
            inc
        });

    let mut scores: Vec<String> = incomplete.iter()
        .map(|tokens| {
            let mut bad_tokens: Vec<Token> = Vec::new();
            parse_2(&mut tokens.iter(), &mut bad_tokens, None);
            bad_tokens
        })
        .map(|bad_tokens| bad_tokens.iter().fold(0, |a, b| a * 5 + b.to_points()))
        .map(|num| num.to_string())
        .collect();

    scores.sort();

    println!("Part 2: {}", scores[scores.len() / 2]);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
