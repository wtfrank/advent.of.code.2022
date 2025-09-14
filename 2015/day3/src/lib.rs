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

fn parse_input1(puzzle_input: &str) -> isize {
  let mut pos = Point { x: 0, y: 0 };
  let mut visited = HashMap::<Point, usize>::default();

  visited.insert(pos, 1);
  for c in puzzle_input.chars() {
    match c {
      '>' => pos.x += 1,
      '^' => pos.y -= 1,
      'v' => pos.y += 1,
      '<' => pos.x -= 1,
      '\n' | '\r' => (),
      _ => panic!("bad input {}", c as u8),
    };

    visited.entry(pos).and_modify(|c| *c += 1).or_insert(1);
  }
  visited.len() as isize
}

fn parse_input2(puzzle_input: &str) -> isize {
  let mut pos = Point { x: 0, y: 0 };
  let mut pos2 = Point { x: 0, y: 0 };
  let mut visited = HashMap::<Point, usize>::default();

  visited.insert(pos, 2);
  let mut iter = puzzle_input.chars();
  while let Some(c) = iter.next() {
    match c {
      '>' => pos.x += 1,
      '^' => pos.y -= 1,
      'v' => pos.y += 1,
      '<' => pos.x -= 1,
      '\n' | '\r' => (),
      _ => panic!("bad input {}", c as u8),
    };

    visited.entry(pos).and_modify(|c| *c += 1).or_insert(1);

    match iter.next() {
      Some(c) => {
        match c {
          '>' => pos2.x += 1,
          '^' => pos2.y -= 1,
          'v' => pos2.y += 1,
          '<' => pos2.x -= 1,
          '\n' | '\r' => (),
          _ => panic!("bad input {}", c as u8),
        };
      }
      None => break,
    }

    visited.entry(pos2).and_modify(|c| *c += 1).or_insert(1);
  }
  visited.len() as isize
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
    assert_eq!(result, 4);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 11);
  }
}
