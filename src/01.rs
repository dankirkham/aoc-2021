use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    {
        let lines = read_lines("./input/01.txt").unwrap();

        let (increases, _) = lines
            .map(|val| val.unwrap()) // unwrap result
            .map(|val| -> i32 {val.parse().unwrap()}) // parse int
            .fold((-1 as i32, 0 as i32), |(num, prev), curr| (if curr > prev { num + 1 } else { num }, curr));

        println!("Increases: {}", increases);
    }

    {
        let lines = read_lines("./input/01.txt").unwrap();

        let (filtered, _) = lines
            .map(|val| val.unwrap()) // unwrap result
            .map(|val| -> i32 {val.parse().unwrap()}) // parse int
            .fold((Vec::new(), (0 as i32, 0 as i32)), |(mut avg, (w1, w2)), val| ({avg.push(w1 + w2 + val); avg}, (w2, val)));

        let sliced = &filtered[2..];

        for line in sliced.iter() {
            println!("{}", line);
        }

        let (increases, _) = sliced.iter()
            .fold((-1 as i32, 0 as i32), |(num, prev), curr| (if curr > &prev { num + 1 } else { num }, *curr));

        println!("Increases2: {}", increases);
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
