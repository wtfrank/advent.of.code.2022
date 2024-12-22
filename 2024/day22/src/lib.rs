use std::fs::File;
use std::io::Read;

#[allow(unused_imports)]
use advent::{Dims, Direction, Point, TerrainMap};

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

pub fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

pub fn analyse_input1(puzzle_input: &str) -> usize {
  let mut total = 0;
  for line in puzzle_input.lines() {
    let mut secret = line.parse::<usize>().unwrap();
    for _ in 1..=2000 {
      secret = evolve(secret);
    }
    total += secret;
    println!("new secret: {secret}");
  }
  total
}

fn evolve(secret: usize) -> usize {
  //left shift 6
  let mut n = secret * 64;
  n ^= secret;
  let secret = n % 16777216; // 2^24

  //right shift 5
  let mut n = secret / 32;
  n ^= secret;
  let secret = n % 16777216;

  //left shift 11
  let mut n = secret * 2048;
  n ^= secret;
  n % 16777216
}

pub fn analyse_input2(puzzle_input: &str) -> usize {
  let mut seq_values = HashMap::<(isize, isize, isize, isize), usize>::default();
  for line in puzzle_input.lines() {
    let mut monkey_seq_values = HashMap::<(isize, isize, isize, isize), usize>::default();

    let mut secret = line.parse::<usize>().unwrap();
    let mut deltas = Vec::new();
    for _ in 1..=2000 {
      let new_secret = evolve(secret);
      let price1 = secret % 10;
      let price2 = new_secret % 10;

      let delta = price2 as isize - price1 as isize;
      deltas.push(delta);
      secret = new_secret;

      if deltas.len() >= 4 {
        let seq = (
          deltas[deltas.len() - 4],
          deltas[deltas.len() - 3],
          deltas[deltas.len() - 2],
          deltas[deltas.len() - 1],
        );
        monkey_seq_values.entry(seq).or_insert(price2);

        // println!("{price2} ({price1},{delta}): {seq:?}");
      }
    }
    for (seq, val) in monkey_seq_values {
      *seq_values.entry(seq).or_insert(0) += val;
    }
    // println!("----------------------");
  }

  *seq_values.values().max().unwrap()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 37327623);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 23);
  }

  #[test]
  fn test_evolve() {
    assert_eq!(evolve(123), 15887950);
    assert_eq!(evolve(15887950), 16495136);
    assert_eq!(evolve(16495136), 527345);
    assert_eq!(evolve(527345), 704524);
    assert_eq!(evolve(704524), 1553684);
    assert_eq!(evolve(1553684), 12683156);
    assert_eq!(evolve(12683156), 11100544);
    assert_eq!(evolve(11100544), 12249484);
    assert_eq!(evolve(12249484), 7753432);
    assert_eq!(evolve(7753432), 5908254);
  }
}
