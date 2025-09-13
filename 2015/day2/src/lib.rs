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

fn paper_needed(dims: &[isize; 3]) -> isize {
  let mut total = 0;
  let mut lowest_area;

  let area = dims[0] * dims[1];
  lowest_area = area;
  total += 2 * area;
  let area = dims[0] * dims[2];
  if area < lowest_area {
    lowest_area = area;
  }
  total += 2 * area;
  let area = dims[1] * dims[2];
  if area < lowest_area {
    lowest_area = area;
  }
  total += 2 * area;

  total += lowest_area;
  total
}

fn parse_input1(puzzle_input: &str) -> isize {
  let mut acc: [isize; 3] = [0_isize; 3];
  let mut count = 0;
  let mut total = 0;

  for c in puzzle_input.chars() {
    match c {
      'x' => count += 1,
      '\n' => {
        total += paper_needed(&acc);
        count = 0;
        acc = [0_isize; 3];
      }
      '0'..='9' => acc[count] = acc[count] * 10 + (c as u8 - b'0') as isize,
      _ => panic!("bad input {}", c as u8),
    };
  }
  total
}

fn ribbon_needed(dims: &[isize; 3]) -> isize {
  let mut lowest_perim;

  let perim = 2 * (dims[0] + dims[1]);
  lowest_perim = perim;
  let perim = 2 * (dims[0] + dims[2]);
  if perim < lowest_perim {
    lowest_perim = perim;
  }
  let perim = 2 * (dims[1] + dims[2]);
  if perim < lowest_perim {
    lowest_perim = perim;
  }

  lowest_perim + dims[0] * dims[1] * dims[2]
}

fn parse_input2(puzzle_input: &str) -> isize {
  let mut acc: [isize; 3] = [0_isize; 3];
  let mut count = 0;
  let mut total = 0;

  for c in puzzle_input.chars() {
    match c {
      'x' => count += 1,
      '\n' => {
        let rn = ribbon_needed(&acc);
        total += rn;
        count = 0;
        acc = [0_isize; 3];
      }
      '0'..='9' => acc[count] = acc[count] * 10 + (c as u8 - b'0') as isize,
      _ => panic!("bad input {}", c as u8),
    };
  }
  total
}

pub fn analyse_input1(puzzle_input: &str) -> isize {
  parse_input1(puzzle_input)
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
    assert_eq!(result, 101);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 48);
  }
}
