use std::fs;

use aoc_2021::day06;

fn main() {
    let input = fs::read_to_string("input/06.txt").expect("couldn't open file");

    println!("{}", day06::part1(input.as_str()));
    println!("{}", day06::part2(input.as_str()));
}
