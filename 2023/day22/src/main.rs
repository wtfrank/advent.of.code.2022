use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

use advent::{Interval, Point3};
//use advent::{determine_map_dims, pos_mod, Direction, Interval, Point3, TerrainMap3};

//use enum_iterator::all;
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
//use advent::Range;

/// Day 21 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn intersects(apos1: Point3, apos2: Point3, bpos1: Point3, bpos2: Point3) -> bool {
  let iax = Interval {
    start: apos1.x,
    length: 1 + apos2.x as usize - apos1.x as usize,
  };
  let ibx = Interval {
    start: bpos1.x,
    length: 1 + bpos2.x as usize - bpos1.x as usize,
  };
  let iay = Interval {
    start: apos1.y,
    length: 1 + apos2.y as usize - apos1.y as usize,
  };
  let iby = Interval {
    start: bpos1.y,
    length: 1 + bpos2.y as usize - bpos1.y as usize,
  };
  let iaz = Interval {
    start: apos1.z,
    length: 1 + apos2.z as usize - apos1.z as usize,
  };
  let ibz = Interval {
    start: bpos1.z,
    length: 1 + bpos2.z as usize - bpos1.z as usize,
  };

  iax.overlaps(&ibx) && iay.overlaps(&iby) && iaz.overlaps(&ibz)
}

fn can_drop1(i: usize, bricks: &[Brick]) -> bool {
  can_drop1_ignoring(i, bricks, None)
}

fn can_drop1_ignoring(i: usize, bricks: &[Brick], ignore: Option<usize>) -> bool {
  let brick = bricks.get(i).unwrap();
  if brick.pos1.z == 1 {
    return false;
  }

  let mut npos1 = brick.pos1;
  let mut npos2 = brick.pos2;

  npos1.z -= 1;
  npos2.z -= 1;

  for i2 in 0..i {
    if let Some(j) = ignore {
      if i2 == j {
        continue;
      }
    }
    let brick2 = bricks.get(i2).unwrap();

    if intersects(npos1, npos2, brick2.pos1, brick2.pos2) {
      return false;
    }
  }
  true
}

fn can_disintegrate(i: usize, bricks: &[Brick]) -> bool {
  for j in i + 1..bricks.len() {
    if can_drop1_ignoring(j, bricks, Some(i)) {
      return false;
    }
  }

  println!("can disintegrate: {i}");
  true
}

fn drop_bricks(bricks: &mut [Brick]) -> usize {
  let mut num_dropped = 0;
  for i in 0..bricks.len() {
    let mut can_drop = false;
    loop {
      if can_drop1(i, bricks) {
        let brick = bricks.get_mut(i).unwrap();
        brick.pos1.z -= 1;
        brick.pos2.z -= 1;
        can_drop = true;
      } else {
        //println!("brick {i} final: {:?}", bricks.get(i).unwrap());
        break;
      }
    }
    if can_drop {
      num_dropped += 1;
    }
  }
  num_dropped
}

fn analyse_data(bricks: &mut [Brick]) -> usize {
  bricks.sort_by(|a, b| std::cmp::min(a.pos1.z, b.pos2.z).cmp(&std::cmp::min(b.pos1.z, b.pos2.z)));
  drop_bricks(bricks);
  bricks.sort_by(|a, b| std::cmp::min(a.pos1.z, b.pos2.z).cmp(&std::cmp::min(b.pos1.z, b.pos2.z)));

  let mut disintegratable = 0;

  for i in 0..bricks.len() {
    if can_disintegrate(i, bricks) {
      disintegratable += 1;
    }
  }

  disintegratable
}

fn analyse_data2(bricks: &mut [Brick]) -> usize {
  bricks.sort_by(|a, b| std::cmp::min(a.pos1.z, b.pos2.z).cmp(&std::cmp::min(b.pos1.z, b.pos2.z)));
  drop_bricks(bricks);
  bricks.sort_by(|a, b| std::cmp::min(a.pos1.z, b.pos2.z).cmp(&std::cmp::min(b.pos1.z, b.pos2.z)));

  let mut chain = 0;

  for i in 0..bricks.len() {
    let mut bricks_copy = bricks.to_vec();
    bricks_copy.remove(i);
    let num_dropped = drop_bricks(&mut bricks_copy);
    println!("brick {i} caused {num_dropped} drops");
    chain += num_dropped;
  }

  chain
}

#[derive(Debug, Default, Clone)]
struct Brick {
  pos1: Point3,
  pos2: Point3,
}

fn load_data(filename: &str) -> Vec<Brick> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut bricks = Vec::new();

  for line in contents.lines() {
    let mut brick = Brick::default();
    let mut b = line.split('~');
    let start = b.next().unwrap();
    let mut s = start.split(',');
    brick.pos1.x = s.next().unwrap().parse::<isize>().unwrap();
    brick.pos1.y = s.next().unwrap().parse::<isize>().unwrap();
    brick.pos1.z = s.next().unwrap().parse::<isize>().unwrap();
    assert!(s.next().is_none());
    let end = b.next().unwrap();
    let mut e = end.split(',');
    brick.pos2.x = e.next().unwrap().parse::<isize>().unwrap();
    brick.pos2.y = e.next().unwrap().parse::<isize>().unwrap();
    brick.pos2.z = e.next().unwrap().parse::<isize>().unwrap();
    assert!(e.next().is_none());
    assert!(b.next().is_none());

    let mut num_dims = 0;
    if brick.pos1.x - brick.pos2.x != 0 {
      num_dims += 1;
    }
    if brick.pos1.y - brick.pos2.y != 0 {
      num_dims += 1;
    }
    if brick.pos1.z - brick.pos2.z != 0 {
      num_dims += 1;
    }
    if num_dims > 1 {
      println!("bulky brick: {brick:?}");
    }

    bricks.push(brick);
  }

  bricks
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

  let mut data = load_data("input22.txt");
  let score1 = analyse_data(&mut data);
  println!("score1: {score1}");
  let score2 = analyse_data2(&mut data);
  println!("score1: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let mut data = load_data("testinput.txt");
    let score1 = analyse_data(&mut data);
    assert_eq!(score1, 5);
  }
  #[test]
  fn test_load2() {
    let mut data = load_data("testinput.txt");
    let score2 = analyse_data2(&mut data);
    assert_eq!(score2, 7);
  }
}
