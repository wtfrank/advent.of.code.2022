use std::fs::File;
use std::io::Read;

#[allow(unused_imports)]
use advent::{Dims, Direction, Point, TerrainMap};

#[allow(unused_imports)]
use std::collections::VecDeque;

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

#[allow(unused_imports)]
use priority_queue::PriorityQueue;
#[allow(unused_imports)]
use std::cmp::Reverse;

#[allow(unused_imports)]
use enum_iterator::{all, Sequence};

#[allow(unused_imports)]
use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;

#[allow(unused_imports)]
use std::sync::OnceLock;

#[allow(unused_imports)]
use regex::Regex;

#[allow(unused_imports)]
use rand::{
  distributions::{Distribution, Uniform},
  prelude::*,
  Rng,
};

#[allow(unused_imports)]
use rand_chacha::ChaCha8Rng;

pub fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

fn parse_input(puzzle_input: &str) -> isize {
  let mut floor = 0_isize;
  for c in puzzle_input.chars() {
    floor += match c {
      '(' => 1,
      ')' => -1,
      '\r' => 0,
      '\n' => 0,
      _ => panic!("bad input {}", c as u8),
    };
  }
  floor
}

fn parse_input2(puzzle_input: &str) -> isize {
  let mut floor = 0_isize;
  let mut count = 0;
  for c in puzzle_input.chars() {
    floor += match c {
      '(' => 1,
      ')' => -1,
      '\r' => 0,
      '\n' => 0,
      _ => panic!("bad input {}", c as u8),
    };
    count += 1;
    if floor == -1 {
      return count;
    }
  }
  panic!("floor -1 not reached");
}

pub fn analyse_input1(puzzle_input: &str) -> isize {
  parse_input(puzzle_input)
}

pub fn analyse_input2(puzzle_input: &str) -> isize {
  parse_input2(puzzle_input)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1a() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 0);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 1);
  }
}
