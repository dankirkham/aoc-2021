#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use aoc_2021::day04::{part1, part2};
    use test::Bencher;

    #[bench]
    fn bench_04_part1(b: &mut Bencher) {
        let input = include_str!("../input/04.txt");

        b.iter(|| part1(input));
    }

    #[bench]
    fn bench_04_part2(b: &mut Bencher) {
        let input = include_str!("../input/04.txt");

        b.iter(|| part2(input));
    }
}