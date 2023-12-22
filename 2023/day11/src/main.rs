use clap::Parser;
use std::fs::File;
use std::io::Read;

use advent::{Dims, Point, TerrainMap};

//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::{max,Reverse,Ordering};
use std::collections::HashSet;
//use std::collections::HashMap;

//use std::iter::zip;

//use std::collections::HashSet;
//use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

//use std::collections::HashMap;

//use advent::{prime_factors, lcm};

/// Day 3 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn empty_col(x: usize, map: &TerrainMap<bool>) -> bool {
  let mut p = Point { x: x as isize, y: 0 };
  for y in 0..map.dims.height {
    p.y = y as isize;
    if map.get(&p) {
      return false;
    }
  }
  true
}

fn empty_row(y: usize, map: &TerrainMap<bool>) -> bool {
  let mut p = Point { x: 0, y: y as isize };
  for x in 0..map.dims.width {
    p.x = x as isize;
    if map.get(&p) {
      return false;
    }
  }
  true
}

fn expand_galaxy2(map1: &TerrainMap<bool>) -> (HashSet<isize>, HashSet<isize>) {
  let mut expand_rows = HashSet::<isize>::default();
  let mut expand_cols = HashSet::<isize>::default();

  for x in 0..map1.dims.width {
    if empty_col(x, map1) {
      expand_cols.insert(x as isize);
    }
  }

  for y in 0..map1.dims.height {
    if empty_row(y, map1) {
      expand_rows.insert(y as isize);
    }
  }

  (expand_rows, expand_cols)
}

fn determine_map_dims(data: &str) -> Dims {
  let mut width = 0;
  let mut height = 0;
  for l in data.lines() {
    height += 1;
    let w = l.len();
    if w > width {
      width = w;
    }
  }

  Dims {
    width,
    height,
    ..Default::default()
  }
}

fn load_data(filename: &str) -> TerrainMap<bool> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut map = TerrainMap::<bool>::new(determine_map_dims(&contents));

  let mut p = Point { x: 0, y: 0 };
  for line in contents.lines() {
    for c in line.chars() {
      if c == '#' {
        map.set(&p, true);
      }
      p.x += 1;
    }
    p.x = 0;
    p.y += 1;
  }

  //sequences.push( line.split(' ').map( |a| a.parse::<isize>().unwrap() ).collect() );
  //let r = sscanf::sscanf!(line, "{String} = ({String}, {String})").unwrap();
  map
}

fn find_galaxies(map: &TerrainMap<bool>) -> Vec<Point> {
  let mut galaxies = Vec::<Point>::new();

  let mut p = Point::default();
  for y in 0..map.dims.height {
    p.y = y as isize;
    for x in 0..map.dims.width {
      p.x = x as isize;

      if map.get(&p) {
        galaxies.push(p);
      }
    }
  }

  galaxies
}

fn analyse_data(map: &TerrainMap<bool>, expansion: usize) -> usize {
  let galaxies = find_galaxies(map);

  let (expand_rows, expand_cols) = expand_galaxy2(map);

  let mut galaxies2 = Vec::<Point>::new();

  for g in galaxies.iter() {
    let mut p = *g;

    for r in expand_rows.iter() {
      if g.y > *r {
        p.y += expansion as isize;
      }
    }

    for c in expand_cols.iter() {
      if g.x > *c {
        p.x += expansion as isize;
      }
    }

    galaxies2.push(p);
  }

  let mut score = 0;
  for a in 0..galaxies2.len() {
    for b in a + 1..galaxies2.len() {
      let ga = galaxies2[a];
      let gb = galaxies2[b];

      let dist = (ga.x - gb.x).abs() + (ga.y - gb.y).abs();
      score += dist;
    }
  }

  score as usize
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input11.txt");
  //let data = expand_galaxy(&data);
  let score1 = analyse_data(&data, 1);
  let score2 = analyse_data(&data, 999_999);
  println!("score1: {score1}");
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput1.txt");
    //let data = expand_galaxy( &data );
    //let expanded_data = load_data( "testinput2.txt" );
    //assert_eq!(data, expanded_data);
    let score = analyse_data(&data, 1);
    assert_eq!(score, 374);
    let score = analyse_data(&data, 9);
    assert_eq!(score, 1030);
    let score = analyse_data(&data, 99);
    assert_eq!(score, 8410);
  }
}
