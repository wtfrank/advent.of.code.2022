use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

use advent::{Dims, Direction, Point, TerrainMap};

use enum_iterator::all;
//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::Reverse;
//use std::cmp::{max,Reverse,Ordering};
//use std::collections::HashMap;
//use std::collections::HashSet;
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

fn analyse_data(trench: &TerrainMap<bool>) -> usize {
  let mut flood = TerrainMap::<bool>::new(trench.dims);

  let mut queue = Vec::new();
  let mut checked = TerrainMap::<bool>::new(trench.dims);

  queue.push(Point {
    x: trench.dims.minx,
    y: trench.dims.miny,
  });

  while let Some(p) = queue.pop() {
    flood.set(&p, true);

    for d in all::<Direction>() {
      let n = p.neighbour(d);
      if !trench.dims.contains(&n) {
        continue;
      }
      //println!("expand {n:?}");
      if checked.get(&n) {
        continue;
      }
      checked.set(&n, true);
      if trench.get(&n) {
        continue;
      }
      queue.push(n);
    }
  }

  let outside = flood.iter().fold(0, |acc, e| if *e { acc + 1 } else { acc });
  //let trench_size = &trench.iter().fold(0, |acc, e| if *e { acc + 1 } else {acc});
  let inside = trench.dims.width * trench.dims.height - outside;

  println!(
    "outside: {outside}, inside: {inside}, area: {}",
    trench.dims.width * trench.dims.height
  );

  inside
}

fn analyse_trenches(trenches: &TrenchPlan) -> usize {
  let mut points = Vec::new();

  let mut cur = Point { x: 0, y: 0 };

  let mut perim = 0;

  for (dir, dist) in trenches {
    match dir {
      Direction::East => cur.x += *dist as isize,
      Direction::West => cur.x -= *dist as isize,
      Direction::North => cur.y -= *dist as isize,
      Direction::South => cur.y += *dist as isize,
    };
    points.push(cur);
    perim += dist;
  }

  assert_eq!(cur, Point { x: 0, y: 0 });

  println!("perimeter: {perim}, total edges: {}", trenches.len());

  let mut area = 0;
  for i in 0..points.len() {
    let p1 = points[i];
    let p2 = points[(i + 1) % points.len()];

    area += (p1.y + p2.y) * (p1.x - p2.x);
  }

  assert!(area > 0);

  let area: usize = (area as usize + perim) / 2 + 1;
  println!("total area: {area}");
  area
}

fn char_to_dir(c: char) -> Direction {
  match c {
    'R' => Direction::East,
    'D' => Direction::South,
    'L' => Direction::West,
    'U' => Direction::North,
    _ => panic!("unexpected direction {c}"),
  }
}

fn char2_to_dir(c: char) -> Direction {
  match c {
    '0' => Direction::East,
    '1' => Direction::South,
    '2' => Direction::West,
    '3' => Direction::North,
    _ => panic!("unexpected direction {c}"),
  }
}

fn parse_so_called_hex(hex: &str) -> (Direction, usize) {
  assert_eq!(hex.len(), 6);
  let (h, d) = hex.split_at(5);
  let mut dist = 0;
  for c in h.chars() {
    dist = 16 * dist + c.to_digit(16).unwrap();
  }
  (char2_to_dir(d.chars().next().unwrap()), dist as usize)
}

type TrenchPlan = Vec<(Direction, usize)>;

fn load_data(filename: &str) -> (TerrainMap<bool>, TrenchPlan, TrenchPlan) {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut v = Vec::new();
  let mut v2 = Vec::new();

  let mut north = 0;
  let mut east = 0;
  let mut south = 0;
  let mut west = 0;

  let mut n2 = 0;
  let mut e2 = 0;
  let mut s2 = 0;
  let mut w2 = 0;
  for line in contents.lines() {
    let r = sscanf::sscanf!(line, "{char} {usize} (#{String})").unwrap();
    let d = char_to_dir(r.0);
    match d {
      Direction::North => {
        north += r.1;
      }
      Direction::East => {
        east += r.1;
      }
      Direction::South => {
        south += r.1;
      }
      Direction::West => {
        west += r.1;
      }
    };

    let (dir2, dist2) = parse_so_called_hex(&r.2);
    match dir2 {
      Direction::North => n2 += dist2,
      Direction::East => e2 += dist2,
      Direction::South => s2 += dist2,
      Direction::West => w2 += dist2,
    }
    v.push((d, r.1));
    v2.push((dir2, dist2));
  }

  println!("n:{north} e:{east} s:{south} w:{west}");
  println!("n:{n2} e:{e2} s:{s2} w:{w2}");
  let mut map = TerrainMap::<bool>::new(Dims {
    minx: -(west as isize),
    width: 2 * east + 1,
    miny: -(north as isize),
    height: 2 * south + 1,
  });

  let mut pos = Point { x: 0, y: 0 };
  map.set(&pos, true);
  for (dir, dist) in &v {
    for _ in 0..*dist {
      pos = pos.neighbour(*dir);
      map.set(&pos, true);
    }
  }

  /*
    for (y, line) in contents.lines().enumerate() {
      for (x, c) in line.chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        map.setc(x as isize, y as isize, n as usize);
      }
    }
  */

  (map, v, v2)
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

  //let (score1, score2) = analyse_data(&mut data);
  let (data, _, trenches2) = load_data("input18.txt");
  let score1 = analyse_data(&data);
  println!("score1: {score1}");
  let score2 = analyse_trenches(&trenches2);
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let (data, trenches1, _) = load_data("testinput.txt");
    let score1 = analyse_data(&data);
    assert_eq!(score1, 62);
    let score1 = analyse_trenches(&trenches1);
    assert_eq!(score1, 62);
  }

  #[test]
  fn test_load2() {
    let (_, _, trenches2) = load_data("testinput.txt");
    let score2 = analyse_trenches(&trenches2);
    assert_eq!(score2, 952408144115);
  }
}
