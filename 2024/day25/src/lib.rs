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

enum ParseState {
  Start,
  Lock,
  Key,
}

type LockSpec = [usize; 5];
type KeySpec = [usize; 5];

fn parse_input(puzzle_input: &str) -> (Vec<LockSpec>, Vec<KeySpec>) {
  let mut locks: Vec<LockSpec> = Vec::new();
  let mut keys: Vec<KeySpec> = Vec::new();

  let mut state = ParseState::Start;
  let mut cur_lock = [0; 5];
  let mut cur_key = [0; 5];
  for line in puzzle_input.lines() {
    match state {
      ParseState::Start => {
        if line.chars().all(|c| c == '#') {
          state = ParseState::Lock;
          cur_lock = [0; 5];
        } else if line.chars().all(|c| c == '.') {
          state = ParseState::Key;
          cur_key = [0; 5];
        } else {
          panic!("Unexpected line {line}");
        }
      }
      ParseState::Lock => {
        if line.is_empty() {
          locks.push(cur_lock);
          state = ParseState::Start;
        } else {
          for (i, c) in line.chars().enumerate() {
            match c {
              '.' => (),
              '#' => cur_lock[i] += 1,
              _ => panic!("unexpected char {c}"),
            }
          }
        }
      }
      ParseState::Key => {
        if line.is_empty() {
          // final line is not included in the keys
          for v in cur_key.iter_mut() {
            *v -= 1;
          }
          keys.push(cur_key);
          state = ParseState::Start;
        } else {
          for (i, c) in line.chars().enumerate() {
            match c {
              '.' => (),
              '#' => cur_key[i] += 1,
              _ => panic!("unexpected char {c}"),
            }
          }
        }
      }
    }
  }

  match state {
    ParseState::Lock => locks.push(cur_lock),
    ParseState::Key => {
      for v in cur_key.iter_mut() {
        *v -= 1;
      }
      keys.push(cur_key)
    }
    _ => (),
  }

  println!("{} locks, {} keys", locks.len(), keys.len());
  (locks, keys)
}
// 3315 too low
pub fn analyse_input1(puzzle_input: &str) -> usize {
  let (locks, keys) = parse_input(puzzle_input);
  itertools::iproduct!(locks.iter(), keys.iter()).fold(0, |acc, (l, k)| acc + if compatible(l, k) { 1 } else { 0 })
}

pub fn compatible(lock: &LockSpec, key: &KeySpec) -> bool {
  std::iter::zip(lock, key).all(|(l, k)| l + k <= 5)
}
pub fn analyse_input2(_puzzle_input: &str) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1a() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 3);
  }

  #[test]
  fn test_load1b() {
    let data = load_data("testinput2.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 2024);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 0);
  }
}
