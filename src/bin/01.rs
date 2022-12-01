use aoc_2021::day01;

fn main() {
    let example = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
    let input = include_str!("../../input/01.txt");

    println!("{}", day01::part1(example));
    println!("{}", day01::part2(example));

    println!("{}", day01::part1(input));
    println!("{}", day01::part2(input));
}

