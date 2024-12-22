#![feature(test)]
extern crate test;
use aoc_2024_day22::*;

#[cfg(test)]
mod tests {
  use super::*;

  use test::{black_box, Bencher};
  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input22.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input22.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
