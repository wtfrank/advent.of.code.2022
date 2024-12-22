#![feature(test)]
extern crate test;

use clap::Parser;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;
//use itertools::sorted;
//use std::collections::VecDeque;

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
use enum_iterator::all;

use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;

/// Day 19 of Advent of Code 2024
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

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
enum TowelColour {
  White = 0,
  Blue = 1,
  Black = 2,
  Red = 3,
  Green = 4,
}

impl std::fmt::Display for TowelColour {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        TowelColour::White => "w",
        TowelColour::Blue => "u",
        TowelColour::Black => "b",
        TowelColour::Red => "r",
        TowelColour::Green => "g",
      }
    )
  }
}

struct TrieNode {
  children: [Option<Box<TrieNode>>; 5],
  is_terminal: bool,
  depth: usize,
}

impl TrieNode {
  fn new() -> Self {
    TrieNode {
      children: [None, None, None, None, None],
      is_terminal: false,
      depth: 0,
    }
  }
  fn insert(&mut self, key: &[TowelColour]) {
    let mut t = self;
    for i in 0..key.len() {
      let idx = key[i] as usize;
      if t.children[idx].is_none() {
        t.children[idx] = Some(Box::new(TrieNode::new()));
      }
      let l = key.len() - i;
      if t.depth < l {
        t.depth = l;
      }

      t = t.children[idx].as_mut().unwrap().as_mut();
    }
    t.is_terminal = true;
  }

  /*
  fn search(&self, key: &[TowelColour]) -> bool {
    let mut t = self;
    for i in 0..key.len() {
      let idx = key[i] as usize;
      if t.children[idx].is_none() {
        return false;
      }
      t = t.children[idx].as_ref().unwrap().as_ref();
    }
    true
  }*/

  fn search_exact(&self, key: &[TowelColour]) -> bool {
    let mut t = self;
    for i in 0..std::cmp::min(key.len(), t.depth) {
      let idx = key[i] as usize;
      if t.children[idx].is_none() {
        return false;
      }
      if key.len() - i > t.depth {
        return false;
      }
      t = t.children[idx].as_ref().unwrap().as_ref();
    }
    t.is_terminal
  }
}

fn char_to_towel_colour(c: char) -> TowelColour {
  match c {
    'w' => TowelColour::White,
    'u' => TowelColour::Blue,
    'b' => TowelColour::Black,
    'r' => TowelColour::Red,
    'g' => TowelColour::Green,
    _ => panic!("unexpected character {c}"),
  }
}

fn analyse_input1(puzzle_input: &str) -> usize {
  let mut total = 0;
  let mut li = puzzle_input.lines();
  let mut trie = TrieNode::new();
  let towels: Vec<&str> = li.next().unwrap().split(", ").collect();
  for towel in towels {
    let towel = towel.chars().map(char_to_towel_colour).collect::<Vec<TowelColour>>();

    trie.insert(&towel);
  }

  println!("trie depth: {}", trie.depth);

  for pattern in li.skip(1).by_ref() {
    let pattern = pattern.chars().map(char_to_towel_colour).collect::<Vec<TowelColour>>();

    if pattern_possible(&pattern, &mut trie) {
      total += 1;
    }
  }

  total
}

fn pattern_possible(pattern: &[TowelColour], trie: &mut TrieNode) -> bool {
  // println!("Checking {pattern:?}");
  if pattern.is_empty() {
    return true;
  }
  for i in 1..=pattern.len() {
    let subslice = &pattern[0..i];
    // println!("\\-{i} {subslice:?}");
    if trie.search_exact(subslice) && pattern_possible(&pattern[i..], trie) {
      return true;
    }
  }
  false
}

fn pattern_count<'a, 'b>(
  pattern: &'b [TowelColour],
  trie: &mut TrieNode,
  cache: &'a mut HashMap<Vec<TowelColour>, usize>,
) -> usize
where
  'b: 'a,
{
  if pattern.is_empty() {
    return 1;
  }
  if cache.contains_key(pattern) {
    return *cache.get(pattern).unwrap();
  }
  //println!("Checking {pattern:?}");

  let mut count = 0;
  for i in 1..=pattern.len() {
    let subslice = &pattern[0..i];
    // println!("\\-{i} {count} {subslice:?}");
    if trie.search_exact(subslice) {
      let c = pattern_count(&pattern[i..], trie, cache);
      count += c;
      cache.insert(pattern[i..].to_vec(), c);
      if c > 0 {
        // println!("{subslice:?} has {c} arrangements of {:?}", &pattern[i..]);
      }
    }
  }
  count
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;
  let mut li = puzzle_input.lines();
  let mut trie = TrieNode::new();
  let towels: Vec<&str> = li.next().unwrap().split(", ").collect();
  for towel in towels {
    let towel = towel.chars().map(char_to_towel_colour).collect::<Vec<TowelColour>>();

    trie.insert(&towel);
  }

  println!("trie depth: {}", trie.depth);

  let mut cache = HashMap::<Vec<TowelColour>, usize>::default();

  for pattern in li.skip(1).by_ref() {
    let pattern = pattern.chars().map(char_to_towel_colour).collect::<Vec<TowelColour>>();

    let c = pattern_count(&pattern, &mut trie, &mut cache);
    println!("total: {c} arrangements of {pattern:?}");
    total += c;
  }

  total
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input19.txt");
  let answer1 = analyse_input1(&data);
  println!("answer: {answer1}");
  let answer2 = analyse_input2(&data);
  println!("answer2: {answer2:?} (693043560673292 too high)");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 6);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 16);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};
  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input19.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input19.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
