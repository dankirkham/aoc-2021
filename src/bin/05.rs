use std::fs;

use aoc_2021::day05;

fn main() {
    let input = fs::read_to_string("input/05.txt").expect("couldn't open file");

    println!("{}", day05::part1(input.as_str()));
    println!("{}", day05::part2(input.as_str()));
}
