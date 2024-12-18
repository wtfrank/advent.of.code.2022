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
use enum_iterator::all;

/// Day 15 of Advent of Code 2024
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

#[derive(Default, Clone, Copy, PartialEq)]
enum MapEntity {
  #[default]
  Empty,
  Crate,
  Wall,
}

fn analyse_input1(puzzle_input: &str) -> usize {
  let mut total = 0;

  let mut height = 0;
  for l in puzzle_input.lines() {
    if l.is_empty() {
      break;
    }
    height += 1;
  }
  let dims = Dims {
    height,
    width: puzzle_input.lines().next().unwrap().len(),
    ..Default::default()
  };

  println!("Height {}, Width {}", dims.height, dims.width);

  let mut tm = TerrainMap::<MapEntity>::new(dims);

  let mut p = Point::default();
  let mut start: Option<Point> = None;

  let mut moves = Vec::<Direction>::new();
  let mut checking_path = false;
  for line in puzzle_input.lines() {
    if line.is_empty() {
      checking_path = true;
      continue;
    }
    if checking_path {
      for c in line.chars() {
        moves.push(match c {
          '^' => Direction::North,
          '>' => Direction::East,
          'v' => Direction::South,
          '<' => Direction::West,
          _ => panic!("unexpected direction"),
        });
      }
    } else {
      for c in line.chars() {
        let e = match c {
          '#' => MapEntity::Wall,
          'O' => MapEntity::Crate,
          '.' => MapEntity::Empty,
          '@' => {
            start = Some(p);
            MapEntity::Empty
          }

          _ => panic!("unexpected char"),
        };
        tm.set(&p, e);
        p.x += 1;
      }
      p.y += 1;
      p.x = 0;
    }
  }

  let mut cur = start.expect("robot start loc not found");
  for m in moves.iter() {
    let next = cur.neighbour(*m);
    match tm.get(&next) {
      MapEntity::Empty => {
        cur = next;
      }
      MapEntity::Wall => {}
      MapEntity::Crate => {
        // check back until we find a wall or empty space
        // then if empty space swap first and last and move robot
        let mut next2 = next;
        loop {
          next2 = next2.neighbour(*m);
          match tm.get(&next2) {
            MapEntity::Wall => break,
            MapEntity::Empty => {
              tm.set(&next2, MapEntity::Crate);
              tm.set(&next, MapEntity::Empty);
              cur = next;
              break;
            }
            _ => continue,
          }
        }
      }
    }

    // draw_map(&tm, &cur);
  }

  for y in 0..dims.height {
    for x in 0..dims.width {
      if tm.getc(x as isize, y as isize) == MapEntity::Crate {
        total += x + 100 * y;
      }
    }
  }

  total
}
#[allow(dead_code)]
fn draw_map(map: &TerrainMap<MapEntity>, robot: &Point) {
  for y in 0..map.dims.height {
    let mut chars = Vec::<char>::new();
    for x in 0..map.dims.width {
      if robot.x == x as isize && robot.y == y as isize {
        chars.push('@');
      } else {
        chars.push(match map.getc(x as isize, y as isize) {
          MapEntity::Empty => '.',
          MapEntity::Crate => 'O',
          MapEntity::Wall => '#',
        });
      }
    }
    let s: String = chars.into_iter().collect();
    println!("{s}");
  }
}

#[allow(dead_code)]
fn draw_map2(map: &TerrainMap<MapEntity2>, robot: &Point) {
  for y in 0..map.dims.height {
    let mut chars = Vec::<char>::new();
    for x in 0..map.dims.width {
      if robot.x == x as isize && robot.y == y as isize {
        chars.push('@');
      } else {
        chars.push(match map.getc(x as isize, y as isize) {
          MapEntity2::Empty => '.',
          MapEntity2::CrateLeft => '[',
          MapEntity2::CrateRight => ']',
          MapEntity2::Wall => '#',
        });
      }
    }
    let s: String = chars.into_iter().collect();
    println!("{s}");
  }
}

#[derive(Default, Clone, Copy, PartialEq)]
enum MapEntity2 {
  #[default]
  Empty,
  CrateLeft,
  CrateRight,
  Wall,
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;

  let mut height = 0;
  for l in puzzle_input.lines() {
    if l.is_empty() {
      break;
    }
    height += 1;
  }
  let dims = Dims {
    height,
    width: puzzle_input.lines().next().unwrap().len() * 2,
    ..Default::default()
  };

  println!("Height {}, Width {}", dims.height, dims.width);

  let mut tm = TerrainMap::<MapEntity2>::new(dims);

  let mut p = Point::default();
  let mut start: Option<Point> = None;

  let mut moves = Vec::<Direction>::new();
  let mut checking_path = false;
  for line in puzzle_input.lines() {
    if line.is_empty() {
      checking_path = true;
      continue;
    }
    if checking_path {
      for c in line.chars() {
        moves.push(match c {
          '^' => Direction::North,
          '>' => Direction::East,
          'v' => Direction::South,
          '<' => Direction::West,
          _ => panic!("unexpected direction"),
        });
      }
    } else {
      for c in line.chars() {
        let e = match c {
          '#' => MapEntity2::Wall,
          'O' => MapEntity2::CrateLeft,
          '.' => MapEntity2::Empty,
          '@' => {
            start = Some(p);
            MapEntity2::Empty
          }

          _ => panic!("unexpected char"),
        };
        tm.set(&p, e);
        p.x += 1;
        let e = match c {
          '#' => MapEntity2::Wall,
          'O' => MapEntity2::CrateRight,
          '.' => MapEntity2::Empty,
          '@' => MapEntity2::Empty,
          _ => panic!("unexpected char"),
        };
        tm.set(&p, e);
        p.x += 1;
      }
      p.y += 1;
      p.x = 0;
    }
  }

  let mut cur = start.expect("robot start loc not found");
  // draw_map2(&tm, &cur);
  for m in moves.iter() {
    if can_push(*m, cur, &tm) {
      do_push(*m, cur, &mut tm);
      cur = cur.neighbour(*m);
    }
    // draw_map2(&tm, &cur);
  }

  for y in 0..dims.height {
    for x in 0..dims.width {
      if tm.getc(x as isize, y as isize) == MapEntity2::CrateLeft {
        total += x + 100 * y;
      }
    }
  }

  total
}

fn can_push(dir: Direction, from: Point, tm: &TerrainMap<MapEntity2>) -> bool {
  let next = from.neighbour(dir);
  match tm.get(&next) {
    MapEntity2::Empty => true,
    MapEntity2::Wall => false,
    MapEntity2::CrateLeft => {
      let next2 = next.neighbour(Direction::East);
      match dir {
        Direction::East => can_push(dir, next2, tm),
        Direction::West => can_push(dir, next, tm),
        Direction::North => can_push(dir, next, tm) && can_push(dir, next2, tm),
        Direction::South => can_push(dir, next, tm) && can_push(dir, next2, tm),
      }
    }
    MapEntity2::CrateRight => {
      let next2 = next.neighbour(Direction::West);
      match dir {
        Direction::East => can_push(dir, next, tm),
        Direction::West => can_push(dir, next2, tm),
        Direction::North => can_push(dir, next, tm) && can_push(dir, next2, tm),
        Direction::South => can_push(dir, next, tm) && can_push(dir, next2, tm),
      }
    }
  }
}

fn do_push(dir: Direction, from: Point, tm: &mut TerrainMap<MapEntity2>) {
  let next = from.neighbour(dir);
  match tm.get(&next) {
    MapEntity2::Empty => {}
    MapEntity2::Wall => {
      panic!("Not supposed to hit a wall while pushing");
    }
    MapEntity2::CrateLeft => {
      let next2 = next.neighbour(Direction::East);
      match dir {
        Direction::East => do_push(dir, next, tm),
        Direction::West => do_push(dir, next, tm),
        Direction::North => {
          do_push(dir, next, tm);
          do_push(dir, next2, tm)
        }
        Direction::South => {
          do_push(dir, next, tm);
          do_push(dir, next2, tm)
        }
      }
    }
    MapEntity2::CrateRight => {
      let next2 = next.neighbour(Direction::West);
      match dir {
        Direction::East => do_push(dir, next, tm),
        Direction::West => do_push(dir, next, tm),
        Direction::North => {
          do_push(dir, next, tm);
          do_push(dir, next2, tm)
        }
        Direction::South => {
          do_push(dir, next, tm);
          do_push(dir, next2, tm)
        }
      }
    }
  };

  tm.set(&next, tm.get(&from));
  tm.set(&from, MapEntity2::Empty);
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input15.txt");
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
    assert_eq!(result, 10092);
  }

  #[test]
  fn test_load1b() {
    let data = load_data("testinput2.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 2028);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 9021);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input15.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input15.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
