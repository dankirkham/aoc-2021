use std::fs::File;
use std::fmt;
use std::fmt::Write;
use std::io::{self, BufRead};
use std::path::Path;

pub struct Point (usize, usize);

impl Point {
    pub fn from_str(s: &str) -> Self {
        let mut strs = s.split(',');
        Self (
            strs.next().unwrap().parse().unwrap(),
            strs.next().unwrap().parse().unwrap(),
        )
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.0, self.1)
    }
}

pub enum LineType {
    Horizontal,
    Vertical,
    Diagonal,
}

pub struct Line {
    a: Point,
    b: Point,
    line_type: LineType
}

impl Line {
    pub fn from_str(s: &str) -> Self {
        let mut strs = s.split(" -> ");
        let a = Point::from_str(strs.next().unwrap());
        let b = Point::from_str(strs.next().unwrap());

        let mut line_type = LineType::Diagonal;
        if a.0 == b.0 {
            line_type = LineType::Vertical;
        } else if a.1 == b.1 {
            line_type = LineType::Horizontal;
        }

        Self {
            a,
            b,
            line_type,
        }
    }

    fn paint_horizontal(&self, diagram: &mut Diagram) -> () {
        let data = diagram.data.as_mut_slice();
        let y = self.a.1;
        let start = self.a.0.min(self.b.0);
        let finish = self.a.0.max(self.b.0) + 1;
        for x in start..finish {
            data[y * diagram.width + x] += 1;
        }
    }

    fn paint_vertical(&self, diagram: &mut Diagram) -> () {
        let data = diagram.data.as_mut_slice();
        let x = self.a.0;
        let start = self.a.1.min(self.b.1);
        let finish = self.a.1.max(self.b.1) + 1;
        for y in start..finish {
            data[y * diagram.width + x] += 1;
        }
    }

    fn paint_diagonal(&self, diagram: &mut Diagram) -> () {
        let data = diagram.data.as_mut_slice();

        let (mut x, mut y) = (self.a.0, self.a.1);
        let (start_x, start_y) = (self.b.0, self.b.1);

        let x_positive = x < start_x;
        let y_positive = y < start_y;

        loop {
            data[y * diagram.width + x] += 1;

            if x == start_x || y == start_y {
                break;
            }

            if x_positive {
                x += 1;
            } else {
                x -= 1;
            }

            if y_positive {
                y += 1;
            } else {
                y -= 1;
            }
        }
    }

    fn paint(&self, diagram: &mut Diagram) -> () {
        match &self.line_type {
            LineType::Horizontal => self.paint_horizontal(diagram),
            LineType::Vertical => self.paint_vertical(diagram),
            LineType::Diagonal => self.paint_diagonal(diagram),
            // LineType::Diagonal => (),
        }
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> {}", self.a, self.b)
    }
}

pub struct Diagram {
    data: Vec<i32>,
    width: usize,
    height: usize,
}

impl Diagram {
    pub fn new(lines: &Vec<Line>) -> Self {
        let points = lines.iter().map(|line| [&line.a, &line.b]).flatten();

        let (max_x, max_y) = points.fold((0, 0), |(width, height), p| {
            (width.max(p.0), height.max(p.1))
        });

        let width = max_x + 1;
        let height = max_y + 1;

        Self {
            data: vec![0; width * height],
            width,
            height,
        }
    }

    pub fn dangerous_areas(&self) -> usize {
        self.data.iter().filter(|&v| *v >= 2).count()
    }
}

impl fmt::Display for Diagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        let mut data_iter = self.data.iter();
        for _ in 0..self.width {
            for _ in 0..self.height {
                if let Some(num) = data_iter.next() {
                    write!(s, "{}", num).unwrap();
                } else {
                    write!(s, ".").unwrap();
                }
            }
            write!(s, "\n").unwrap();
        }
        write!(f, "{}", s)
    }
}

fn main() {
    let mut text_lines = read_lines("./input/05.txt").unwrap();

    let mut lines: Vec<Line> = Vec::new();
    while let Some(line) = text_lines.next() {
        let s = line.unwrap();
        lines.push(Line::from_str(&s));
    }

    let mut diagram = Diagram::new(&lines);
    lines.iter().for_each(|line| line.paint(&mut diagram));

    println!("Dangerous Areas: {}", diagram.dangerous_areas());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
