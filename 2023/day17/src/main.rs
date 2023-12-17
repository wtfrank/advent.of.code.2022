use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

use advent::{determine_map_dims, Direction, Point, TerrainMap};

use enum_iterator::all;
//use enum_iterator::{all,Sequence};

use priority_queue::PriorityQueue;
use std::cmp::Reverse;
//use std::cmp::{max,Reverse,Ordering};
use std::collections::HashMap;
use std::collections::HashSet;
//use std::collections::VecDeque;

//use std::iter::zip;

//use std::collections::HashSet;
//use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

//use std::collections::HashMap;

//use advent::{prime_factors, lcm};

/// Day 12 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

type PathNode = (Point, Direction, usize);

/* checks if path len is less than 4 */
fn path_len(prev: &HashMap<PathNode, PathNode>, node: PathNode) -> usize {
  let mut plen = 0;
  let mut prev_node = node;

  loop {
    match prev.get(&prev_node) {
      None => return plen,
      Some(p) => {
        plen += 1;
        prev_node = *p;
        if plen > 4 {
          return plen;
        }
      }
    }
  }
}

fn movement_ok_for_ultra_cruc(
  p: Point,
  neigh: Point,
  dir: Direction,
  prevd: Direction,
  priorc: usize,
  prev: &HashMap<PathNode, PathNode>,
  end_pos: Point,
) -> bool {
  if dir == prevd && priorc >= 9 {
    return false;
  }

  if dir != prevd {
    if path_len(prev, (p, prevd, priorc)) >= 4 {
      if priorc < 3 {
        println!("cannot change to {dir:?} from {p} {prevd:?}{priorc}");
        return false;
      }
    } else {
      //println!("skipping {neigh} {dir:?} due to path shortness: {p}, {prevd:?}, {priorc}");
      return false;
    }
  }

  if neigh == end_pos && (dir != prevd || priorc < 2) {
    return false;
  }

  true
}

fn movement_ok_for_cruc(dir: Direction, prevd: Direction, priorc: usize) -> bool {
  !(dir == prevd && priorc >= 2)
}

fn explore_dijk(
  map: &TerrainMap<usize>,
  start_pos: Point,
  ultra_cruc: bool,
) -> (HashMap<PathNode, PathNode>, HashMap<PathNode, usize>) {
  //triple is a position, the Direction you travelled to enter it, how many times you had travelled
  //that direction before you entered this point (so if you had already travelled 2x, you couldn't
  //leave this point from that same direction)
  let mut visited = HashSet::<PathNode>::default();
  let mut dist = HashMap::<PathNode, usize>::default();
  let mut prev = HashMap::<PathNode, PathNode>::default();
  let mut queue = PriorityQueue::<PathNode, Reverse<usize>>::default();

  let end_pos = Point {
    x: map.dims.width as isize - 1,
    y: map.dims.height as isize - 1,
  };

  let mut p = Point::default();
  for y in 0..map.dims.height {
    p.y = y as isize;
    for x in 0..map.dims.width {
      p.x = x as isize;
      for dir in all::<Direction>() {
        for i in 0..10 {
          dist.insert((p, dir, i), usize::MAX);
          queue.push((p, dir, i), Reverse(usize::MAX));
        }
      }
    }
  }

  for dir in all::<Direction>() {
    dist.insert((start_pos, dir, 0), 0);
    queue.change_priority(&(start_pos, dir, 0), Reverse(0));
  }

  while let Some(entry) = queue.pop() {
    let p = entry.0 .0;
    let prevd = entry.0 .1;
    let priorc = entry.0 .2;
    if entry.1 == Reverse(usize::MAX) {
      //println!("no route to {p} from {prevd:?}");
      continue;
    }
    visited.insert((p, prevd, priorc));
    println!(
      "Visited {p} from {prevd:?}{priorc}, cost {}",
      dist.get(&(p, prevd, priorc)).unwrap()
    );

    for dir in all::<Direction>() {
      if dir == prevd.reverse() {
        continue;
      }
      let neigh = p.neighbour(dir);
      if !map.dims.contains(&neigh) {
        continue;
      }

      let newc = match dir == prevd {
        true => {
          if p == start_pos {
            priorc
          } else {
            priorc + 1
          }
        }
        false => 0,
      };

      if visited.contains(&(neigh, dir, newc)) {
        continue;
      }

      if (ultra_cruc && !movement_ok_for_ultra_cruc(p, neigh, dir, prevd, priorc, &prev, end_pos))
        || (!ultra_cruc && !movement_ok_for_cruc(dir, prevd, priorc))
      {
        continue;
      }

      let new_dist = dist.get(&(p, prevd, priorc)).unwrap() + map.get(&neigh);
      if new_dist < *dist.get(&(neigh, dir, newc)).unwrap() {
        dist.insert((neigh, dir, newc), new_dist);
        queue.change_priority(&(neigh, dir, newc), Reverse(new_dist));
        prev.insert((neigh, dir, newc), (p, prevd, priorc));
        println!(
          "considering {neigh} {dir:?}{newc} from {p}{prevd:?}{priorc} with cost {new_dist}: {}+{}",
          dist.get(&(p, prevd, priorc)).unwrap(),
          map.get(&neigh)
        );
      }
    }
  }

  (prev, dist)
}

fn calc_heat_loss(
  _map: &TerrainMap<usize>,
  prev: &HashMap<PathNode, PathNode>,
  dist: &HashMap<PathNode, usize>,
  dest: Point,
  dest_dir: Direction,
  dest_count: usize,
) -> usize {
  let mut path = Vec::new();
  let mut cur = (dest, dest_dir, dest_count);
  let start = Point { x: 0, y: 0 };
  while cur.0 != start {
    println!("chl: {} {:?}{}", cur.0, cur.1, cur.2);
    let (prev, prevd, priorc) = prev.get(&cur).unwrap();
    let d = dist.get(&cur).unwrap();

    //loss += map.get

    path.push((cur.0, *d));

    cur = (*prev, *prevd, *priorc);
  }
  path.push((start, 0));
  path.reverse();
  for (p, d) in path {
    println!("path: {p} {d}");
  }
  0
}

fn calc_score(dist: &HashMap<PathNode, usize>, end_pos: Point) -> (usize, Direction, usize) {
  let mut score = usize::MAX;
  let mut end_dir = Direction::East;
  let mut end_count = usize::MAX;
  for d in all::<Direction>() {
    for c in 0..10 {
      match dist.get(&(end_pos, d, c)) {
        None => (),
        Some(s) => {
          if *s < score {
            score = *s;
            end_dir = d;
            end_count = c
          }
        }
      }
    }
  }
  for ((p, d, c), val) in dist {
    if *p == end_pos && *val != usize::MAX {
      println!("found end_pos {d:?}{c} with cost {val}");
    }
  }
  (score, end_dir, end_count)
}

fn analyse_data(map: &TerrainMap<usize>, ultra_cruc: bool, analyse: bool) -> usize {
  let start_pos = Point { x: 0, y: 0 };
  let (prev, dist) = explore_dijk(map, start_pos, ultra_cruc);

  let end_pos = Point {
    x: map.dims.width as isize - 1,
    y: map.dims.height as isize - 1,
  };

  let (score, end_dir, end_count) = calc_score(&dist, end_pos);

  println!("score: {score}, {end_dir:?} {end_count}");

  if analyse {
    calc_heat_loss(map, &prev, &dist, end_pos, end_dir, end_count);
  }

  score
}

fn load_data(filename: &str) -> TerrainMap<usize> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut map = TerrainMap::<usize>::new(determine_map_dims(&contents));

  for (y, line) in contents.lines().enumerate() {
    for (x, c) in line.chars().enumerate() {
      let n = c.to_digit(10).unwrap();
      map.setc(x as isize, y as isize, n as usize);
    }
  }

  map
  //for line in contents.lines() {
  //sequences.push( line.split(' ').map( |a| a.parse::<isize>().unwrap() ).collect() );
  //let r = sscanf::sscanf!(line, "{String} = ({String}, {String})").unwrap();
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  //let mut data = load_data("input15.txt");
  //let (score1, score2) = analyse_data(&mut data);
  let data = load_data("input17.txt");
  let (score1, score2) = (analyse_data(&data, false, false), analyse_data(&data, true, true));
  println!("score1: {score1}");
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput.txt");
    let score1 = analyse_data(&data, false, true);
    assert_eq!(score1, 102);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput.txt");
    let score2 = analyse_data(&data, true, true);
    assert_eq!(score2, 94);
  }

  #[test]
  fn test_load3() {
    let data = load_data("testinput2.txt");
    let score2 = analyse_data(&data, true, true);
    assert_eq!(score2, 71);
  }

  #[test]
  fn test_load3a() {
    let data = load_data("testinput2a.txt");
    let score2 = analyse_data(&data, true, true);
    assert_eq!(score2, 14);
  }

  #[test]
  fn test_load3b() {
    let data = load_data("testinput2b.txt");
    let score2 = analyse_data(&data, true, true);
    assert_eq!(score2, 13);
  }

  #[test]
  fn test_load3c() {
    let data = load_data("testinput2c.txt");
    let score2 = analyse_data(&data, true, true);
    assert_eq!(score2, 12);
  }

  #[test]
  fn test_load4() {
    let data = load_data("testinput4.txt");
    let score2 = analyse_data(&data, true, true);
    assert_eq!(score2, 55);
  }

  #[test]
  fn test_load5() {
    let data = load_data("testinput5.txt");
    let score2 = analyse_data(&data, true, true);
    assert_eq!(score2, 43);
  }
}
