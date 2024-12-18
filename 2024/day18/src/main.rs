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

/// Day 18 of Advent of Code 2024
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

#[derive(Default, Debug, Clone, Copy, PartialEq, FromPrimitive)]
enum MapEntity {
  #[default]
  Clear = 0,
  Byte = 1,
}

fn parse_line(line: &str) -> Point {
  let mut coords = line.split(",");
  let x = coords.next().unwrap().parse::<isize>().unwrap();
  let y = coords.next().unwrap().parse::<isize>().unwrap();
  assert!(coords.next().is_none());
  Point { x, y }
}

fn analyse_input1(puzzle_input: &str) -> usize {
  let mut li = puzzle_input.lines();
  let max_coord = li.next().unwrap().parse::<usize>().unwrap();
  let max_drop = li.next().unwrap().parse::<usize>().unwrap();
  let dims = Dims {
    height: max_coord + 1,
    width: max_coord + 1,
    ..Default::default()
  };

  println!("Height {}, Width {}", dims.height, dims.width);

  let mut tm = TerrainMap::<MapEntity>::new(dims);

  let mut count = 0;
  for line in li {
    let p = parse_line(line);
    tm.set(&p, MapEntity::Byte);
    count += 1;
    if count == max_drop {
      break;
    }
  }

  let start = Point { x: 0, y: 0 };
  let end = Point {
    x: max_coord as isize,
    y: max_coord as isize,
  };

  assert!(tm.dims.contains(&start));
  assert!(tm.dims.contains(&end));

  let steps = search_route(start, end, &tm);
  steps.unwrap()
}

fn search_route(start: Point, end: Point, tm: &TerrainMap<MapEntity>) -> Option<usize> {
  let mut pq = PriorityQueue::<Point, Reverse<usize>>::default();
  let mut expanded = HashSet::<Point>::default();

  pq.push(start, Reverse(0));

  let mut cost: Option<usize> = None;
  while let Some((node, priority)) = pq.pop() {
    //println!("checking {node:?}");
    if node == end {
      cost = Some(priority.0);
      break;
    }
    expanded.insert(node);
    for d in all::<Direction>() {
      let n = node.neighbour(d);
      if !tm.dims.contains(&n) {
        continue;
      }
      if tm.get(&n) == MapEntity::Byte {
        continue;
      }
      if expanded.contains(&n) {
        continue;
      }
      let entry = pq.get(&n);
      if entry.is_none() {
        pq.push(n, Reverse(priority.0 + 1));
      } else if entry.unwrap().1 .0 > priority.0 + 1 {
        pq.change_priority(&n, Reverse(priority.0 + 1));
      }
    }
  }

  cost
}

fn analyse_input2(puzzle_input: &str) -> Point {
  let mut li = puzzle_input.lines();
  let max_coord = li.next().unwrap().parse::<usize>().unwrap();
  let max_drop = li.next().unwrap().parse::<usize>().unwrap();
  let dims = Dims {
    height: max_coord + 1,
    width: max_coord + 1,
    ..Default::default()
  };

  println!("Height {}, Width {}", dims.height, dims.width);

  let mut tm = TerrainMap::<MapEntity>::new(dims);

  let mut p = Point::default();

  let mut count = 0;
  for line in li.by_ref() {
    let mut coords = line.split(",");
    p.x = coords.next().unwrap().parse::<isize>().unwrap();
    p.y = coords.next().unwrap().parse::<isize>().unwrap();
    assert!(coords.next().is_none());
    tm.set(&p, MapEntity::Byte);
    count += 1;
    if count == max_drop {
      break;
    }
  }

  let start = Point { x: 0, y: 0 };
  let end = Point {
    x: max_coord as isize,
    y: max_coord as isize,
  };

  assert!(tm.dims.contains(&start));
  assert!(tm.dims.contains(&end));

  for next in li {
    count += 1;
    let next = parse_line(next);
    tm.set(&next, MapEntity::Byte);
    let steps = search_route(start, end, &tm);
    if steps.is_some() {
      println!("{count}: route exists with length {}", steps.unwrap());
    } else {
      return next;
    }
  }
  panic!("Route was not blocked");
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input18.txt");
  let answer1 = analyse_input1(&data);
  println!("answer: {answer1}");
  let answer2 = analyse_input2(&data);
  println!("answer2: {answer2:?}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 22);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, Point { x: 6, y: 1 });
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};
  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input18.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input18.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
