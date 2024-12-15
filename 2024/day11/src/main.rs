#![feature(test)]
extern crate test;

use clap::Parser;
#[allow(unused_imports)]
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;
//use itertools::sorted;
//use std::collections::VecDeque;

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

/// Day 11 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

fn analyse_input1(puzzle_input: &str) -> usize {
  let mut cur_numbers = Vec::<usize>::new();
  let mut prev_numbers = Vec::<usize>::new();
  for n in puzzle_input.split_whitespace().map(|i| i.parse::<usize>().unwrap()) {
    cur_numbers.push(n);
  }

  let iters = 25;
  for _ in 0..iters {
    for n in cur_numbers.iter() {
      if *n == 0 {
        prev_numbers.push(1);
      } else {
        let ndigits = n.checked_ilog10().unwrap() + 1;
        if ndigits % 2 == 0 {
          let lhs = n / 10_usize.pow(ndigits / 2);
          let rhs = n % 10_usize.pow(ndigits / 2);
          prev_numbers.push(lhs);
          prev_numbers.push(rhs);
        } else {
          prev_numbers.push(n * 2024);
        }
      }
    }

    (cur_numbers, prev_numbers) = (prev_numbers, cur_numbers);
    prev_numbers.clear();
  }

  cur_numbers.len()
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut cur_numbers = HashMap::<usize, usize>::default();
  let mut prev_numbers = HashMap::<usize, usize>::default();
  for n in puzzle_input.split_whitespace().map(|i| i.parse::<usize>().unwrap()) {
    let e = cur_numbers.entry(n).or_insert(0);
    *e = e.saturating_add(1);
  }

  let iters = 75;
  for _ in 0..iters {
    for (n, quantity) in cur_numbers.iter() {
      if *n == 0 {
        let e = prev_numbers.entry(1).or_insert(0);
        *e = e.saturating_add(*quantity);
      } else {
        let ndigits = n.checked_ilog10().unwrap() + 1;
        if ndigits % 2 == 0 {
          let lhs = n / 10_usize.pow(ndigits / 2);
          let rhs = n % 10_usize.pow(ndigits / 2);
          let e = prev_numbers.entry(lhs).or_insert(0);
          *e = e.saturating_add(*quantity);
          let e = prev_numbers.entry(rhs).or_insert(0);
          *e = e.saturating_add(*quantity);
        } else {
          let e = prev_numbers.entry(n * 2024).or_insert(0);
          *e = e.saturating_add(*quantity);
        }
      }
    }

    (cur_numbers, prev_numbers) = (prev_numbers, cur_numbers);
    prev_numbers.clear();
    println!("total numbers: {}", cur_numbers.len());
  }

  cur_numbers.iter().fold(0, |a, (_, v)| a + v)
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input11.txt");
  let answer1 = analyse_input1(&data);
  println!("answer: {answer1}");
  let answer2 = analyse_input2(&data);
  println!("answer2: {answer2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 55312);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 2858);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input11.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input11.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
