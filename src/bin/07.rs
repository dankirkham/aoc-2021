use std::fs;

use aoc_2021::day07;

fn main() {
    let input = fs::read_to_string("input/07.txt").expect("couldn't open file");

    println!("{}", day07::part1(input.as_str()));
    println!("{}", day07::part2(input.as_str()));
}
