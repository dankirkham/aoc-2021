use std::fs;

use aoc_2021::day15;

fn main() {
    let input = fs::read_to_string("input/15.txt").expect("couldn't open file");

    println!("{}", day15::part1(input.as_str()));
    println!("{}", day15::part2(input.as_str()));
}
