#![feature(test)]
extern crate test;

use clap::Parser;
#[allow(unused_imports)]
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;
use itertools::sorted;
use std::collections::VecDeque;

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

/// Day 9 of Advent of Code 2024
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
  let mut total = 0;

  let mut free_block_list = VecDeque::<usize>::new();
  let mut block_usage = HashMap::<usize, usize>::default();

  let mut reading_blanks = false;
  let mut block_number = 0_usize;
  let mut file_id = 0_usize;
  for c in puzzle_input.chars().map(|a| a.to_digit(10).unwrap() as usize) {
    match reading_blanks {
      true => {
        for _ in 0..c {
          free_block_list.push_back(block_number);
          block_number += 1;
        }
      }
      false => {
        for _ in 0..c {
          block_usage.insert(block_number, file_id);
          block_number += 1;
        }
        file_id += 1;
      }
    }
    reading_blanks = !reading_blanks;
  }

  println!("START");
  for k in sorted(block_usage.keys()) {
    let v = block_usage.get(k).unwrap();
    println!("block {k} file id {v}");
  }

  let mut last_used_block = block_number - 1;
  while !free_block_list.is_empty() {
    let first_free = free_block_list.pop_front().unwrap();
    while !block_usage.contains_key(&last_used_block) {
      last_used_block -= 1;
    }
    if last_used_block <= first_free {
      break;
    }
    println!("filling empty block {first_free} from {last_used_block}");
    let v = block_usage.remove(&last_used_block).unwrap();
    block_usage.insert(first_free, v);
    last_used_block -= 1;
  }

  println!("END");
  for k in sorted(block_usage.keys()) {
    let v = block_usage.get(k).unwrap();
    println!("block {k} file id {v}");
    total += k * v;
  }

  total
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;

  let mut free_block_list2 = VecDeque::<(usize, usize)>::new();
  let mut block_usage2 = HashMap::<usize, (usize, usize)>::default();

  let mut reading_blanks = false;
  let mut block_number = 0_usize;
  let mut file_id = 0_usize;
  for c in puzzle_input.chars().map(|a| a.to_digit(10).unwrap() as usize) {
    match reading_blanks {
      true => {
        free_block_list2.push_back((block_number, c));
        block_number += c;
      }
      false => {
        block_usage2.insert(block_number, (file_id, c));
        block_number += c;
        file_id += 1;
      }
    }
    reading_blanks = !reading_blanks;
  }

  let keys_copy: Vec<usize> = block_usage2.keys().copied().collect();
  for k in sorted(keys_copy).rev() {
    let (file_no, length) = *block_usage2.get(&k).unwrap();
    for i in 0..free_block_list2.len() {
      let (free_block, free_length) = *free_block_list2.get(i).unwrap();
      if free_block >= k {
        break;
      }

      if free_length < length {
        continue;
      }

      block_usage2.remove(&k).unwrap();
      block_usage2.insert(free_block, (file_no, length));

      free_block_list2.remove(i);
      let new_free_length = free_length - length;
      let new_free_start = free_block + length;
      if new_free_length > 0 {
        free_block_list2.insert(i, (new_free_start, new_free_length));
      }

      break;
    }
  }

  for k in block_usage2.keys() {
    let (file_no, length) = *block_usage2.get(k).unwrap();
    for i in *k..(*k + length) {
      total += file_no * i;
    }
  }

  total
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input9.txt");
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
    assert_eq!(result, 1928);
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
    let data = load_data("input9.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input9.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
