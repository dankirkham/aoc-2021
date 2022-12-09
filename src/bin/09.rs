use std::fs;

use aoc_2021::day09;

fn main() {
    let input = fs::read_to_string("input/09.txt").expect("couldn't open file");

    println!("{}", day09::part1(input.as_str()));
    println!("{}", day09::part2(input.as_str()));
}
