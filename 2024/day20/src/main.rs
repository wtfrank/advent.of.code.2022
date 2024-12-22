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

/// Day 20 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, FromPrimitive)]
enum MapEntity {
  #[default]
  Clear = 0,
  Wall = 1,
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
    height: puzzle_input.lines().count(),
    width: puzzle_input.lines().next().unwrap().len(),
    ..Default::default()
  };

  let mut tm = TerrainMap::<MapEntity>::new(dims);

  let mut start: Option<Point> = None;
  let mut end: Option<Point> = None;
  let mut p = Point::default();
  for line in puzzle_input.lines() {
    for c in line.chars() {
      match c {
        '.' => tm.set(&p, MapEntity::Clear),
        '#' => tm.set(&p, MapEntity::Wall),
        'S' => {
          tm.set(&p, MapEntity::Clear);
          start = Some(p)
        }
        'E' => {
          tm.set(&p, MapEntity::Clear);
          end = Some(p)
        }
        _ => panic!("unexpected input"),
      };
      p.x += 1;
    }
    p.y += 1;
    p.x = 0;
  }

  let start = start.unwrap();
  let end = end.unwrap();

  println!("start: {start:?}, end: {end:?}");
  let (cost, prev) = search_route(start, end, &tm);

  let uncheat_cost = cost.unwrap();
  let route = derive_route(start, end, &prev);

  println!("original route length {}: {route:?}", route.len());

  let mut cheat_count = HashMap::<usize, usize>::default();
  for p in &route {
    for d in all::<Direction>() {
      let c1 = p.neighbour(d);
      if !tm.dims.contains(&c1) {
        continue;
      }
      if tm.get(&c1) != MapEntity::Wall {
        continue;
      }

      let old_c1 = tm.get(&c1);
      tm.set(&c1, MapEntity::Clear);
      let (cost, prev2) = search_route(start, end, &tm);
      tm.set(&c1, old_c1);
      let cheat_cost = cost.unwrap();
      let saving = uncheat_cost - cheat_cost;
      if saving == 0 {
        continue;
      }
      // if cheat starts from a later point in the route than the
      // other part of the cheat then it wasn't a valid cheat and
      // must have been accounted for from the earlier point in
      // the route.
      // likewise if the cheat opens a path such that the route
      // now enters from an earlier point in the route and don't
      // follow to the later part of the old route, then the
      // cheat was already accounted for, and isn't a valid cheat
      let cheat_route = derive_route(start, end, &prev2);
      let idx1 = cheat_route.iter().position(|r| *r == *p);
      if idx1.is_none() {
        continue;
      }
      let idx1 = idx1.unwrap();
      let idx2 = cheat_route.iter().position(|r| *r == c1).unwrap();
      if idx2 < idx1 {
        continue;
      }

      println!("cheat {p} {c1} saves {saving}");
      *cheat_count.entry(saving).or_insert(0) += 1;

      if saving >= 100 {
        total += 1;
      }
    }
  }

  for s in itertools::sorted(cheat_count.keys()) {
    println!("{} save {s}", cheat_count.get(s).unwrap());
  }
  total
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;

  let dims = Dims {
    height: puzzle_input.lines().count(),
    width: puzzle_input.lines().next().unwrap().len(),
    ..Default::default()
  };

  let mut tm = TerrainMap::<MapEntity>::new(dims);

  let mut start: Option<Point> = None;
  let mut end: Option<Point> = None;
  let mut p = Point::default();
  for line in puzzle_input.lines() {
    for c in line.chars() {
      match c {
        '.' => tm.set(&p, MapEntity::Clear),
        '#' => tm.set(&p, MapEntity::Wall),
        'S' => {
          tm.set(&p, MapEntity::Clear);
          start = Some(p)
        }
        'E' => {
          tm.set(&p, MapEntity::Clear);
          end = Some(p)
        }
        _ => panic!("unexpected input"),
      };
      p.x += 1;
    }
    p.y += 1;
    p.x = 0;
  }

  let start = start.unwrap();
  let end = end.unwrap();

  println!("start: {start:?}, end: {end:?}");
  let (_cost, prev) = search_route(start, end, &tm);

  // let uncheat_cost = cost.unwrap();
  let route = derive_route(start, end, &prev);

  println!("original route length {}: {route:?}", route.len());

  let mut cheat_count = HashMap::<usize, usize>::default();
  for r in 0..route.len() {
    let p = route[r];

    /*
        let mut tm2 = tm.clone();
        for y in 0_isize..tm2.dims.height as isize {
          for x in 0_isize..tm2.dims.width as isize {
            if y.abs_diff(p.y) + x.abs_diff(p.x) >= 20 || !tm2.dims.containsc(x, y) {
              continue;
            }
            if tm2.getc(x, y) == MapEntity::Clear {
              continue;
            }
            if adjacent_to_existing_route(x, y, &route[0..r]) {
              continue;
            }
            tm2.setc(x, y, MapEntity::Clear);
          }
        }
        let (cost, prev2) = search_route(start, end, &tm2);
        let cheat_cost = cost.unwrap();
        let saving = uncheat_cost - cheat_cost;
        if saving == 0 {
          continue;
        }
        // if cheat starts from a later point in the route than the
        // other part of the cheat then it wasn't a valid cheat and
        // must have been accounted for from the earlier point in
        // the route.
        // likewise if the cheat opens a path such that the route
        // now enters from an earlier point in the route and don't
        // follow to the later part of the old route, then the
        // cheat was already accounted for, and isn't a valid cheat
        let cheat_route = derive_route(start, end, &prev2);
        let idx1 = cheat_route.iter().position(|r| *r == p);
        if idx1.is_none() {
          continue;
        }
        let idx1 = idx1.unwrap();
        if r < route.len() - 1 {
          let p2 = route[r + 1];
          // if we don't change direction immediately, we could have cheated later
          let idx2 = cheat_route.iter().position(|r| *r == p2);
          if idx2.is_some() {
            continue;
          }
        }

        // not clear if for any given cheat start loc, we're supposed to find
        // all possible rejoin locations, or only the best one for that given cheat.
        // let c2 = find_rejoin_loc(p, route, cheat_route);
        //
    */

    for (r2, p2) in route.iter().enumerate().skip(r) {
      let cheat_len = p2.y.abs_diff(p.y) + p2.x.abs_diff(p.x);
      if cheat_len > 20 {
        continue;
      }

      let saving = r2 - r - cheat_len;
      if saving == 0 {
        continue;
      }

      // distance saved is distance forward in list - cheat_len

      println!("cheat {p} saves {saving}");
      *cheat_count.entry(saving).or_insert(0) += 1;

      if saving >= 100 {
        total += 1;
      }
    }
  }

  for s in itertools::sorted(cheat_count.keys()) {
    println!("{} save {s}", cheat_count.get(s).unwrap());
  }
  total
}

fn derive_route(start: Point, end: Point, prev: &HashMap<Point, Point>) -> Vec<Point> {
  let mut route = Vec::<Point>::new();

  let mut cur = end;
  while cur != start {
    route.push(cur);
    let p = *prev.get(&cur).unwrap();
    cur = p;
  }

  route.push(start);

  route.reverse();
  route
}

fn search_route(start: Point, end: Point, tm: &TerrainMap<MapEntity>) -> (Option<usize>, HashMap<Point, Point>) {
  let mut pq = PriorityQueue::<Point, Reverse<usize>>::default();
  let mut expanded = HashSet::<Point>::default();
  let mut prev = HashMap::<Point, Point>::default();

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
      if tm.get(&n) == MapEntity::Wall {
        continue;
      }
      if expanded.contains(&n) {
        continue;
      }
      let entry = pq.get(&n);
      if entry.is_none() {
        pq.push(n, Reverse(priority.0 + 1));
        prev.insert(n, node);
      } else if entry.unwrap().1 .0 > priority.0 + 1 {
        pq.change_priority(&n, Reverse(priority.0 + 1));
        prev.insert(n, node);
      }
    }
  }

  (cost, prev)
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input20.txt");
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
    assert_eq!(result, 6);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 16);
  }

  // ---------------------------------------------
  /*
    use test::{black_box, Bencher};
    #[bench]
    fn bench_part1(b: &mut Bencher) {
      let data = load_data("input20.txt");
      b.iter(|| black_box(analyse_input1(&data)));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
      let data = load_data("input20.txt");
      b.iter(|| black_box(analyse_input2(&data)));
    }
  */
}
