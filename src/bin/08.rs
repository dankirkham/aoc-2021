use std::fs;

use aoc_2021::day08;

fn main() {
    let input = fs::read_to_string("input/08.txt").expect("couldn't open file");

    println!("{}", day08::part1(input.as_str()));
    println!("{}", day08::part2(input.as_str()));
}
