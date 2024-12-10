#![feature(test)]
extern crate test;

use clap::Parser;
#[allow(unused_imports)]
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;

use advent::{Dims, Point};

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

/// Day 8 of Advent of Code 2024
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

  println!("Width: {}, Height: {}", dims.width, dims.height);

  let mut antennae = HashMap::<char, Vec<Point>>::default();
  let mut point = Point::default();
  for line in puzzle_input.lines() {
    point.x = 0;
    for c in line.chars() {
      if c != '.' {
        antennae.entry(c).or_insert(Vec::new()).push(point);
      }
      point.x += 1;
    }
    point.y += 1;
  }

  for a in antennae.keys() {
    println!("frequency {a} has {} antennae", antennae.get(a).unwrap().len());
  }

  let mut antinodes = HashMap::<Point, usize>::default();

  for a in antennae.keys() {
    let locs = antennae.get(a).unwrap();
    for i in 0..locs.len() {
      for j in (i + 1)..locs.len() {
        // antinode
        let x_delta = locs[j].x - locs[i].x;
        let y_delta = locs[j].y - locs[i].y;

        let an1 = Point {
          x: locs[i].x - x_delta,
          y: locs[i].y - y_delta,
        };
        let an2 = Point {
          x: locs[j].x + x_delta,
          y: locs[j].y + y_delta,
        };

        if dims.contains(&an1) {
          let _ = antinodes.entry(an1).or_insert(0).saturating_add(1);
        } else {
          println!("antinode: {an1} outside map");
        }
        if dims.contains(&an2) {
          let _ = antinodes.entry(an2).or_insert(0).saturating_add(1);
        } else {
          println!("antinode: {an2} outside map");
        }
      }
    }
  }

  for a in antinodes.keys() {
    println!("antinode: {a} {}", antinodes.get(a).unwrap());
  }

  total += antinodes.keys().len();

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

  println!("Width: {}, Height: {}", dims.width, dims.height);

  let mut antennae = HashMap::<char, Vec<Point>>::default();
  let mut point = Point::default();
  for line in puzzle_input.lines() {
    point.x = 0;
    for c in line.chars() {
      if c != '.' {
        antennae.entry(c).or_insert(Vec::new()).push(point);
      }
      point.x += 1;
    }
    point.y += 1;
  }

  for a in antennae.keys() {
    println!("frequency {a} has {} antennae", antennae.get(a).unwrap().len());
  }

  let mut antinodes = HashMap::<Point, usize>::default();

  for a in antennae.keys() {
    let locs = antennae.get(a).unwrap();
    for i in 0..locs.len() {
      for j in (i + 1)..locs.len() {
        // for part 2 we have to find every integer point on the line defined by these two points
        // antinode

        // the cases are: one dimension = 0, in which case we set the other delta to 0
        // there is a gcd. in which case we divide each dimension by it
        // the deltas are coprime, where we do nothing
        //
        // then we choose a point and extend in each direction until we exceed the border
        let mut x_delta = locs[j].x - locs[i].x;
        let mut y_delta = locs[j].y - locs[i].y;

        let g = advent::gcds(x_delta, y_delta);
        println!("gcd {g} of {x_delta},{y_delta}");
        if x_delta == 0 {
          y_delta = 1;
        } else if y_delta == 0 {
          x_delta = 1;
        } else if g > 1 {
          x_delta /= g;
          y_delta /= g;
        }

        let mut next = locs[i];
        if dims.contains(&next) {
          let _ = antinodes.entry(next).or_insert(0).saturating_add(1);
        }
        loop {
          next = Point {
            x: next.x + x_delta,
            y: next.y + y_delta,
          };
          // println!("Checking {next} +");
          if dims.contains(&next) {
            let _ = antinodes.entry(next).or_insert(0).saturating_add(1);
          } else {
            break;
          }
        }
        next = locs[i];
        loop {
          next = Point {
            x: next.x - x_delta,
            y: next.y - y_delta,
          };
          // println!("Checking {next} -");
          if dims.contains(&next) {
            let _ = antinodes.entry(next).or_insert(0).saturating_add(1);
          } else {
            break;
          }
        }
      }
    }
  }

  for a in antinodes.keys() {
    println!("antinode: {a} {}", antinodes.get(a).unwrap());
  }

  total += antinodes.keys().len();

  total
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input8.txt");
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
    assert_eq!(result, 14);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 34);

    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 9);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input8.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input8.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
