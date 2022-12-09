use std::fs;

use aoc_2021::day02;

fn main() {
    let input = fs::read_to_string("input/02.txt").expect("couldn't open file");

    println!("{}", day02::part1(input.as_str()));
    println!("{}", day02::part2(input.as_str()));
}
