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

use advent::{Dims, Direction, Point, TerrainMap};

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

use enum_iterator::all;

/// Day 12 of Advent of Code 2024
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

  let dims = Dims {
    minx: 0,
    miny: 0,
    width: puzzle_input.lines().next().unwrap().len(),
    height: puzzle_input.lines().count(),
  };
  let mut tm = TerrainMap::<char>::new(dims);
  let mut regions_checked = TerrainMap::<bool>::new(dims);
  let mut point = Point::default();
  for line in puzzle_input.lines() {
    point.x = 0;
    for c in line.chars() {
      tm.set(&point, c);
      point.x += 1;
    }
    point.y += 1;
  }

  // could do the region detection but it doesn't really help
  // with perimeter/area so will just bruteforce

  let mut p = Point::default();
  for y in dims.miny..dims.height as isize {
    p.y = y;
    for x in dims.minx..dims.width as isize {
      p.x = x;

      if regions_checked.get(&p) {
        continue;
      }
      let mut q = Vec::<Point>::new();
      let mut area = 0;
      let mut perimeter = 0;

      q.push(p);
      regions_checked.set(&p, true);
      let cur_region = tm.get(&p);

      println!("Starting with {p}");

      while let Some(cur) = q.pop() {
        area += 1;
        for d in all::<Direction>() {
          let n = cur.neighbour(d);
          if !dims.contains(&n) {
            perimeter += 1;
          } else if regions_checked.get(&n) || tm.get(&n) != cur_region {
            if tm.get(&n) != cur_region {
              perimeter += 1;
            }
            println!("Rejecting from {cur} to {n}");
          } else {
            q.push(n);
            regions_checked.set(&n, true);
            println!("Expanding from {cur} to {n}");
          }
        }
      }

      println!("area {area}, perimeter {perimeter}");
      total += area * perimeter;
    }
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
  let mut tm = TerrainMap::<char>::new(dims);
  let mut regions_checked = TerrainMap::<bool>::new(dims);
  let mut point = Point::default();
  for line in puzzle_input.lines() {
    point.x = 0;
    for c in line.chars() {
      tm.set(&point, c);
      point.x += 1;
    }
    point.y += 1;
  }

  // could do the region detection but it doesn't really help
  // with perimeter/area so will just bruteforce

  let mut p = Point::default();
  for y in dims.miny..dims.height as isize {
    p.y = y;
    for x in dims.minx..dims.width as isize {
      p.x = x;

      if regions_checked.get(&p) {
        continue;
      }
      let mut q = Vec::<Point>::new();
      let mut area = 0;
      let mut perimeter = 0;

      // key is point b, value is (point a, direction vector pointing outside)
      // let mut perim_nodes_b = HashMap::<Point, (Point, Direction)>::default();

      q.push(p);
      regions_checked.set(&p, true);
      let cur_region = tm.get(&p);

      println!("Starting with {p}");

      while let Some(cur) = q.pop() {
        area += 1;
        for d in all::<Direction>() {
          let n = cur.neighbour(d);
          if !dims.contains(&n) {
            perimeter += 1;
          } else if regions_checked.get(&n) || tm.get(&n) != cur_region {
            if tm.get(&n) != cur_region {
              perimeter += 1;
            }
            println!("Rejecting from {cur} to {n}");
          } else {
            q.push(n);
            regions_checked.set(&n, true);
            println!("Expanding from {cur} to {n}");
          }
        }
      }

      println!("area {area}, perimeter {perimeter}");
      total += area * perimeter;
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

  let data = load_data("input12.txt");
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
    assert_eq!(result, 140);
    let data = load_data("testinput2.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 772);
    let data = load_data("testinput3.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 1930);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 80);
    let data = load_data("testinput1.tx2");
    let result = analyse_input2(&data);
    assert_eq!(result, 436);
    let data = load_data("testinput4.tx2");
    let result = analyse_input2(&data);
    assert_eq!(result, 236);
    let data = load_data("testinput5.tx2");
    let result = analyse_input2(&data);
    assert_eq!(result, 368);
    let data = load_data("testinput3.tx2");
    let result = analyse_input2(&data);
    assert_eq!(result, 1206);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input12.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input12.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
