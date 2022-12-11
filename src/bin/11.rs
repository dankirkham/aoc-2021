use std::fs;

use aoc_2021::day11;

fn main() {
    let input = fs::read_to_string("input/11.txt").expect("couldn't open file");

    println!("{}", day11::part1(input.as_str()));
    println!("{}", day11::part2(input.as_str()));
}
