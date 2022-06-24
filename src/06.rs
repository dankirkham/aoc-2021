use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use tqdm_rs;
use std::collections::VecDeque;

fn main() {
    let mut text_lines = read_lines("./input/06.txt").unwrap();
    let line = text_lines.next().unwrap().unwrap();
    let fish = line.split(",").map(|v| v.parse().unwrap());
    let mut counts: VecDeque<u64> = VecDeque::from(vec![0; 9]);
    fish.for_each(|c: usize| counts[c] += 1);

    for _ in tqdm_rs::Tqdm::new(0..256) {
        let new_fish = counts.pop_front().unwrap();
        counts[6] += new_fish;
        counts.push_back(new_fish);

        for (i, c) in counts.iter().enumerate() {
            println!("{}: {}", i, c);
        }
    }

    let total_fish = counts.iter().fold(0, |sum, c| sum + c);
    println!("Fish: {}", total_fish);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
