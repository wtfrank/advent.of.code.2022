#![feature(test)]
extern crate test;

use clap::Parser;
#[allow(unused_imports)]
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;

use advent::Dims;
use advent::Direction;
use advent::Point;
use advent::TerrainMap;

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

/// Day 6 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Map {
  #[default]
  Clear,
  Obstruction,
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
    width: puzzle_input.lines().count(),
    height: puzzle_input.lines().next().unwrap().len(),
  };

  let mut tm = TerrainMap::<Map>::new(dims);
  let mut visited = TerrainMap::<bool>::new(dims);

  let mut point = Point { x: 0, y: 0 };
  let mut guard = Point::default();
  let mut guard_dir = Direction::North;

  point.y = 0;
  for line in puzzle_input.lines() {
    point.x = 0;
    for c in line.chars() {
      match c {
        '.' => (),
        '#' => {
          tm.set(&point, Map::Obstruction);
        }
        '^' => {
          guard = point;
          guard_dir = Direction::North;
        }
        '>' => {
          guard = point;
          guard_dir = Direction::East;
        }
        'v' => {
          guard = point;
          guard_dir = Direction::South;
        }
        '<' => {
          guard = point;
          guard_dir = Direction::West;
        }
        _ => {
          println!("Unexpected map symbol {c}");
        }
      };

      point.x += 1;
    }
    point.y += 1;
  }

  visited.set(&guard, true);
  total += 1;

  'outer: loop {
    loop {
      let next = guard.neighbour(guard_dir);
      if !dims.contains(&next) {
        break 'outer;
      }
      if tm.get(&next) == Map::Obstruction {
        guard_dir = guard_dir.rotate_cw();
      } else {
        guard = next;
        break;
      }
    }
    if !visited.get(&guard) {
      visited.set(&guard, true);
      total += 1;
    }
  }

  total
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;

  let dims = Dims {
    minx: 0,
    miny: 0,
    width: puzzle_input.lines().count(),
    height: puzzle_input.lines().next().unwrap().len(),
  };

  let mut tm = TerrainMap::<Map>::new(dims);
  let mut visited = TerrainMap::<Option<Direction>>::new(dims);

  let mut point = Point { x: 0, y: 0 };
  let mut guard = Point::default();
  let mut guard_dir = Direction::North;

  point.y = 0;
  for line in puzzle_input.lines() {
    point.x = 0;
    for c in line.chars() {
      match c {
        '.' => (),
        '#' => {
          tm.set(&point, Map::Obstruction);
        }
        '^' => {
          guard = point;
          guard_dir = Direction::North;
        }
        '>' => {
          guard = point;
          guard_dir = Direction::East;
        }
        'v' => {
          guard = point;
          guard_dir = Direction::South;
        }
        '<' => {
          guard = point;
          guard_dir = Direction::West;
        }
        _ => {
          println!("Unexpected map symbol {c}");
        }
      };

      point.x += 1;
    }
    point.y += 1;
  }

  let start_guard = guard;
  let start_guard_dir = guard_dir;

  let mut block_candidates = Vec::new();

  visited.set(&guard, Some(guard_dir));

  'outer: loop {
    loop {
      let next = guard.neighbour(guard_dir);
      if !dims.contains(&next) {
        break 'outer;
      }
      if tm.get(&next) == Map::Obstruction {
        guard_dir = guard_dir.rotate_cw();
      } else {
        guard = next;
        break;
      }
    }
    if visited.get(&guard).is_none() {
      visited.set(&guard, Some(guard_dir));
      block_candidates.push(guard);
    }
  }

  // we now know the original path, so rerun the path for each blockable location and see if it loops

  for blocked_loc in block_candidates {
    let mut visited = TerrainMap::<Option<Direction>>::new(dims);
    guard = start_guard;
    guard_dir = start_guard_dir;
    'outer: loop {
      loop {
        let next = guard.neighbour(guard_dir);
        if !dims.contains(&next) {
          break 'outer;
        }
        if tm.get(&next) == Map::Obstruction || next.eq(&blocked_loc) {
          guard_dir = guard_dir.rotate_cw();
        } else {
          guard = next;
          break;
        }
      }
      if visited.get(&guard).is_none() {
        visited.set(&guard, Some(guard_dir));
      } else if visited.get(&guard).unwrap() == guard_dir {
        total += 1;
        println!("blocked loc {blocked_loc} led to loop (overlapped at {guard} {guard_dir:?}");
        break;
      }
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

  let data = load_data("input6.txt");
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
    assert_eq!(result, 41);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 6);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input6.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input6.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
