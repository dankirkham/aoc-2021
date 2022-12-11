#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use std::fs;
    use test::Bencher;

    use aoc_2021::day11::{part1, part2};

    #[bench]
    fn bench_11_part1(b: &mut Bencher) {
        b.iter(|| {
            let input = fs::read_to_string("input/11.txt").expect("unable to open file");
            part1(input.as_str())
        });
    }

    #[bench]
    fn bench_11_part2(b: &mut Bencher) {
        b.iter(|| {
            let input = fs::read_to_string("input/11.txt").expect("unable to open file");
            part2(input.as_str())
        });
    }
}
