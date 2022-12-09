use std::fs;

use aoc_2021::day01;

fn main() {
    let input = fs::read_to_string("input/01.txt").expect("couldn't open file");

    println!("{}", day01::part1(input.as_str()));
    println!("{}", day01::part2(input.as_str()));
}
