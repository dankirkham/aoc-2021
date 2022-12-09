use std::fs;

use aoc_2021::day03;

fn main() {
    let input = fs::read_to_string("input/03.txt").expect("couldn't open file");

    println!("{}", day03::part1(input.as_str()));
    println!("{}", day03::part2(input.as_str()));
}
