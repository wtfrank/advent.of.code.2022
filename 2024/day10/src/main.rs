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

use enum_iterator::all;

use advent::{Dims, Direction, Point, TerrainMap};

/// Day 10 of Advent of Code 2024
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
  let dims = Dims {
    minx: 0,
    miny: 0,
    width: puzzle_input.lines().next().unwrap().len(),
    height: puzzle_input.lines().count(),
  };
  let mut tm = TerrainMap::<usize>::new(dims);

  let mut trailheads = HashSet::<Point>::default();

  let mut point = Point::default();
  for line in puzzle_input.lines() {
    for height in line.chars().map(|c| c.to_digit(10).unwrap() as usize) {
      if height == 0 {
        trailheads.insert(point);
      }
      tm.set(&point, height);
      point.x += 1;
    }
    point.y += 1;
    point.x = 0;
  }

  println!("{} trailheads", trailheads.len());

  let mut total_ends_reached = 0;
  for t in trailheads.iter() {
    let mut ends_reached = HashSet::<Point>::default();
    search_trailhead(&tm, *t, &mut ends_reached);
    //  println!("after trailhead {tn}: {total} {}", ends_reached.len());
    total_ends_reached += ends_reached.len();
  }

  total_ends_reached
}

fn search_trailhead(tm: &TerrainMap<usize>, point: Point, ends_reached: &mut HashSet<Point>) -> usize {
  let mut total = 0;
  let cur_height = tm.get(&point);
  if cur_height == 9 {
    ends_reached.insert(point);
    return 1;
  }
  for d in all::<Direction>() {
    let n = point.neighbour(d);
    if !tm.dims.contains(&n) {
      continue;
    }
    if tm.get(&n) != cur_height + 1 {
      continue;
    }
    total += search_trailhead(tm, n, ends_reached);
  }
  total
}

fn search_trailhead2(tm: &TerrainMap<usize>, point: Point) -> usize {
  let mut total = 0;
  let cur_height = tm.get(&point);
  if cur_height == 9 {
    return 1;
  }
  for d in all::<Direction>() {
    let n = point.neighbour(d);
    if !tm.dims.contains(&n) {
      continue;
    }
    if tm.get(&n) != cur_height + 1 {
      continue;
    }
    total += search_trailhead2(tm, n);
  }
  total
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;

  let dims = Dims {
    minx: 0,
    miny: 0,
    width: puzzle_input.lines().next().unwrap().len(),
    height: puzzle_input.lines().count(),
  };
  let mut tm = TerrainMap::<usize>::new(dims);

  let mut trailheads = HashSet::<Point>::default();

  let mut point = Point::default();
  for line in puzzle_input.lines() {
    for height in line.chars().map(|c| c.to_digit(10).unwrap() as usize) {
      if height == 0 {
        trailheads.insert(point);
      }
      tm.set(&point, height);
      point.x += 1;
    }
    point.y += 1;
    point.x = 0;
  }

  println!("{} trailheads", trailheads.len());

  for t in trailheads.iter() {
    total += search_trailhead2(&tm, *t);
    // println!("after trailhead {tn}: {total} {}", ends_reached.len());
  }

  total
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input10.txt");
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
    assert_eq!(result, 1);

    let data = load_data("testinput2.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 36);
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
    let data = load_data("input10.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input10.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
