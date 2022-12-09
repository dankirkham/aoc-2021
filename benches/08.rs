#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use aoc_2021::day08::{part1, part2};
    use std::fs;

    use test::Bencher;

    #[bench]
    fn bench_08_part1(b: &mut Bencher) {
        b.iter(|| {
            let input = fs::read_to_string("input/08.txt").expect("unable to open file");
            part1(input.as_str())
        });
    }

    #[bench]
    fn bench_08_part2(b: &mut Bencher) {
        b.iter(|| {
            let input = fs::read_to_string("input/08.txt").expect("unable to open file");
            part2(input.as_str())
        });
    }
}
