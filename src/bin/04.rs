use std::fs;

use aoc_2021::day04;

fn main() {
    let input = fs::read_to_string("input/04.txt").expect("couldn't open file");

    println!("{}", day04::part1(input.as_str()));
    println!("{}", day04::part2(input.as_str()));
}
