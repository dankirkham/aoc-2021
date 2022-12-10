use std::fs;

use aoc_2021::day10;

fn main() {
    let input = fs::read_to_string("input/10.txt").expect("couldn't open file");

    println!("{}", day10::part1(input.as_str()));
    println!("{}", day10::part2(input.as_str()));
}
