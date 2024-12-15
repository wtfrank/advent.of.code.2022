#![feature(test)]
extern crate test;

use clap::Parser;
use regex::Regex;
#[allow(unused_imports)]
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;
//use itertools::sorted;
//use std::collections::VecDeque;

use console::Term;

#[allow(unused_imports)]
use advent::{Dims, Direction, Point, TerrainMap};

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

#[allow(unused_imports)]
use enum_iterator::all;

/// Day 14 of Advent of Code 2024
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

type Velocity = Point;

fn analyse_input1(puzzle_input: &str, width: usize, height: usize) -> usize {
  let mut total = 1; // multiplicative identity

  let ra = Regex::new(r"^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$").unwrap();

  let dims = Dims {
    width,
    height,
    ..Default::default()
  };
  let mut robots = Vec::<(Point, Velocity)>::new();

  for line in puzzle_input.lines() {
    let c = ra.captures(line).expect("Robot pos/speed");
    if c.len() != 5 {
      panic!("bad match of a)");
    };
    let pos = Point {
      x: c.get(1).unwrap().as_str().parse::<isize>().unwrap(),
      y: c.get(2).unwrap().as_str().parse::<isize>().unwrap(),
    };
    let vel = Velocity {
      x: c.get(3).unwrap().as_str().parse::<isize>().unwrap(),
      y: c.get(4).unwrap().as_str().parse::<isize>().unwrap(),
    };

    println!("{},{} {},{}", pos.x, pos.y, vel.x, vel.y);
    robots.push((pos, vel));
  }

  visualise_robots(&robots, &dims);
  for _ in 1..=100 {
    for (p, v) in robots.iter_mut() {
      p.x += v.x;
      p.y += v.y;
      if p.x >= width as isize {
        p.x -= width as isize;
      }
      if p.x < 0 {
        p.x += width as isize;
      }
      if p.y >= height as isize {
        p.y -= height as isize;
      }
      if p.y < 0 {
        p.y += height as isize;
      }
    }
  }

  let mut tm = TerrainMap::<usize>::new(dims);

  for (p, _) in robots.iter() {
    println!("final pos: {p}");
    tm.set(p, tm.get(p) + 1);
  }

  let mut quad_total = 0;
  for y in 0..height / 2 {
    for x in 0..width / 2 {
      quad_total += tm.getc(x as isize, y as isize);
    }
  }
  println!("top left: {quad_total}");
  total *= quad_total;

  quad_total = 0;
  for y in (height / 2 + 1)..height {
    for x in 0..width / 2 {
      quad_total += tm.getc(x as isize, y as isize);
    }
  }
  println!("botton left: {quad_total}");
  total *= quad_total;

  quad_total = 0;
  for y in 0..height / 2 {
    for x in (width / 2 + 1)..width {
      quad_total += tm.getc(x as isize, y as isize);
    }
  }
  println!("top right: {quad_total}");
  total *= quad_total;

  quad_total = 0;
  for y in (height / 2 + 1)..height {
    for x in (width / 2 + 1)..width {
      quad_total += tm.getc(x as isize, y as isize);
    }
  }
  println!("botton right: {quad_total}");
  total *= quad_total;

  visualise_robots(&robots, &dims);
  total
}

fn visualise_robots(robots: &[(Point, Velocity)], dims: &Dims) {
  let term = Term::stdout();
  let _ = term.clear_screen();
  let mut tm = TerrainMap::<usize>::new(*dims);

  for (p, _) in robots.iter() {
    tm.set(p, tm.get(p) + 1);
  }
  for y in 0..dims.height {
    let mut chars = Vec::<char>::new();
    for x in 0..dims.width {
      let digit = tm.getc(x as isize, y as isize);
      if digit == 0 {
        chars.push(' ');
      } else {
        chars.push(char::from_digit(digit as u32, 10).unwrap());
      }
    }
    println!("{}", chars.into_iter().collect::<String>());
  }
}

fn analyse_input2(puzzle_input: &str, width: usize, height: usize) -> usize {
  let mut total = 0;

  let ra = Regex::new(r"^p=([0-9]+),([0-9]+) v=(-?[0-9]+),(-?[0-9]+)$").unwrap();

  let dims = Dims {
    width,
    height,
    ..Default::default()
  };
  let mut robots = Vec::<(Point, Velocity)>::new();

  for line in puzzle_input.lines() {
    let c = ra.captures(line).expect("Robot pos/speed");
    if c.len() != 5 {
      panic!("bad match of a)");
    };
    let pos = Point {
      x: c.get(1).unwrap().as_str().parse::<isize>().unwrap(),
      y: c.get(2).unwrap().as_str().parse::<isize>().unwrap(),
    };
    let vel = Velocity {
      x: c.get(3).unwrap().as_str().parse::<isize>().unwrap(),
      y: c.get(4).unwrap().as_str().parse::<isize>().unwrap(),
    };

    println!("{},{} {},{}", pos.x, pos.y, vel.x, vel.y);
    robots.push((pos, vel));
  }

  for s in 1..=1000000 {
    for (p, v) in robots.iter_mut() {
      p.x += v.x;
      p.y += v.y;
      if p.x >= width as isize {
        p.x -= width as isize;
      }
      if p.x < 0 {
        p.x += width as isize;
      }
      if p.y >= height as isize {
        p.y -= height as isize;
      }
      if p.y < 0 {
        p.y += height as isize;
      }
    }
    let mut tm = TerrainMap::<bool>::new(dims);

    let mut unique_places = true;
    for (p, _) in robots.iter() {
      if tm.get(p) {
        unique_places = false;
        break;
      }
      tm.set(p, true);
    }

    if unique_places {
      visualise_robots(&robots, &dims);
      println!("time: {s}");
      total = s;
      break;
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

  let data = load_data("input14.txt");
  let answer1 = analyse_input1(&data, 101, 103);
  println!("answer: {answer1} (36991 too low))");
  let answer2 = analyse_input2(&data, 101, 103);
  println!("answer2: {answer2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data, 11, 7);
    assert_eq!(result, 12);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data, 11, 7);
    assert_eq!(result, 80);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input14.txt");
    b.iter(|| black_box(analyse_input1(&data, 101, 103)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input14.txt");
    b.iter(|| black_box(analyse_input2(&data, 101, 103)));
  }
}
