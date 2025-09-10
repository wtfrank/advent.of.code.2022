#![feature(test)]
extern crate test;
use aoc_2015_day1::*;
use test::{black_box, Bencher};

#[cfg(test)]
mod tests {
  use super::*;

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input1.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input1.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
