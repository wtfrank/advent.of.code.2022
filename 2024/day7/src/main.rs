#![feature(test)]
extern crate test;

use clap::Parser;
#[allow(unused_imports)]
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;

use enum_iterator::{all, Sequence};

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

/// Day 7 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Sequence)]
enum Op {
  #[default]
  Add,
  Multiply,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Sequence)]
enum Op2 {
  #[default]
  Add,
  Multiply,
  Concat,
}

fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

fn analyse_input1(puzzle_input: &str) -> usize {
  let mut total = 0;

  for line in puzzle_input.lines() {
    let mut split = line.split(":");
    let target = split.next().unwrap().parse::<usize>().unwrap();
    let operands: Vec<usize> = split
      .next()
      .unwrap()
      .split_whitespace()
      .map(|a| a.parse::<usize>().unwrap())
      .collect();

    if test_operands(target, operands[0], &operands[1..operands.len()]) {
      total += target;
    }
  }

  total
}

fn test_operands(target: usize, cur: usize, operands: &[usize]) -> bool {
  // println!("target: {target}, cur: {cur}, {}", operands.len());
  if cur > target {
    return false;
  }
  if operands.is_empty() {
    return cur == target;
  }

  for op in all::<Op>() {
    if test_operands(
      target,
      match op {
        Op::Add => cur + operands[0],
        Op::Multiply => cur * operands[0],
      },
      &operands[1..operands.len()],
    ) {
      return true;
    }
  }
  false
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;

  for line in puzzle_input.lines() {
    let mut split = line.split(":");
    let target = split.next().unwrap().parse::<usize>().unwrap();
    let operands: Vec<usize> = split
      .next()
      .unwrap()
      .split_whitespace()
      .map(|a| a.parse::<usize>().unwrap())
      .collect();

    if test_operands2(target, operands[0], &operands[1..operands.len()]) {
      total += target;
    }
  }

  total
}

fn test_operands2(target: usize, cur: usize, operands: &[usize]) -> bool {
  // println!("target: {target}, cur: {cur}, {}", operands.len());
  if cur > target {
    return false;
  }
  if operands.is_empty() {
    return cur == target;
  }

  for op in all::<Op2>() {
    if test_operands2(
      target,
      match op {
        Op2::Add => cur + operands[0],
        Op2::Multiply => cur * operands[0],
        Op2::Concat => {
          let pow = if operands[0] == 0 { 1 } else { 1 + operands[0].ilog10() };
          cur * 10_usize.pow(pow) + operands[0]
        }
      },
      &operands[1..operands.len()],
    ) {
      // println!("Success: {target}");
      return true;
    }
  }
  false
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input7.txt");
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
    assert_eq!(result, 3749);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 11387);
  }

  #[test]
  fn test_ops2() {
    // via addition
    assert!(test_operands2(27, 10, &[17]));
    // mult
    assert!(test_operands2(27, 3, &[9]));
    // concat
    assert!(test_operands2(271, 2, &[71]));
    assert!(test_operands2(271, 27, &[1]));
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input7.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input7.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
