use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::fmt;
//use std::str::FromStr;

use advent::{Dims, Point, TerrainMap};

//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::{max,Reverse,Ordering};
//use std::collections::HashSet;
//use std::collections::HashMap;

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

fn cmp_cols(map: &TerrainMap<char>, x1: usize, x2: usize) -> usize {
  let mut errors = 0;
  for y in 0..map.dims.height {
    if map.get(&Point{x:x1 as isize, y:y as isize}) != map.get( &Point{x:x2 as isize, y:y as isize} ) {
      errors += 1;
    }
  }
  errors
}

fn cmp_rows(map: &TerrainMap<char>, y1: usize, y2: usize) -> usize {
  let mut errors = 0;
  for x in 0..map.dims.width {
    if map.get(&Point{x:x as isize, y:y1 as isize}) != map.get( &Point{x:x as isize, y:y2 as isize} ) {
      errors += 1;
    }
  }
  errors
}



fn reflects_vert(map: &TerrainMap<char>, x: usize) -> usize {
  if x == 0 || x >= map.dims.width{ 
    panic!("bad input");
  }
  
  let cols = std::cmp::min(x, map.dims.width - x);

  let mut errors = 0;

  let mut x2;
  for x1 in x-cols..x {
    //x2 = 2* cols + (x-cols) - x1;
    x2 = x + x - x1-1;
    println!("comparing {cols} cols {x1}&{x2} for {x}");
    errors +=cmp_cols(map, x1, x2);
  }

  errors
}

fn reflects_horiz(map: &TerrainMap<char>, y: usize) -> usize {
  if y == 0 || y >= map.dims.height { 
    panic!("bad input");
  }

  let rows = std::cmp::min(y, map.dims.height - y);

  //println!("row range {rows}: {}-{},{}-{}", y-rows, y-1, 2*rows, 2*rows + (y-rows)-y+1);
  //println!("row range {rows}: {}-{},{}-{}", y-rows, y-1, rows+rows, rows+1);

  let mut errors = 0;
  let mut y2;
  for y1 in y-rows..y {
    //y2 = 2 * rows + (y-rows) - y1;
    //y2 = rows + y - y1;
    y2 = y + y -y1 -1;
    println!("comparing {rows} rows {y1}&{y2} for {y}");
    errors += cmp_rows(map, y1, y2);
  }

  errors
}


fn analyse_data(data: &[TerrainMap<char>]) -> (usize,usize) {
  let mut vert = 0;
  let mut horiz = 0;

  let mut smudgev = 0;
  let mut smudgeh = 0;

  for map in data {
    println!("analysing map {}x{}", map.dims.width, map.dims.height);
    let mut foundh = 0;
    let mut foundv = 0;
    let mut foundsh = 0;
    let mut foundsv = 0;
    for x in 1..map.dims.width {
      let errors = reflects_vert(map, x);
      if errors == 0 {
        vert += x;
        foundv += 1;
        println!("vert reflection at {x}");
      } else if errors == 1 {
        smudgev += x;
        foundsv += 1;
        println!("smudged vert reflection at {x}");
      }
    }
    for y in 1..map.dims.height {
      let errors = reflects_horiz(map, y);
      if errors == 0 {
        horiz += y;
        foundh += 1;
        println!("horiz reflection at {y}");
      }
      else if errors == 1 {
        smudgeh += y;
        foundsh += 1;
        println!("smudged horiz reflection at {y}");
      }
    }
    assert_eq!(foundsh + foundsv, 1);
    assert_eq!(foundh + foundv, 1);
  }

  (vert + 100 * horiz, smudgev + 100 * smudgeh)
}

fn lines_to_map( lines: &[String] ) -> TerrainMap<char> {
  let dims = Dims {minx:0, miny:0, width: lines[0].len(), height: lines.len()};
  let mut map = TerrainMap::new(dims);
  let mut p = Point {x:0, y:0};
  for l in lines.iter() {
    for c in l.chars() {
      map.set(&p, c);

      p.x += 1;
    }
    p.x = 0;
    p.y += 1;
  }

  map
}

fn load_data(filename: &str) -> Vec::<TerrainMap<char>> {
  let mut data = Vec::new();
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut linebuf = Vec::new();
  for line in contents.lines() {
    if line.is_empty() {
      data.push( lines_to_map( &linebuf ) );
      linebuf.clear();
    }
    else {
      linebuf.push(line.to_string());
    }
  //sequences.push( line.split(' ').map( |a| a.parse::<isize>().unwrap() ).collect() );
  //let r = sscanf::sscanf!(line, "{String} = ({String}, {String})").unwrap();
  }
  data.push( lines_to_map( &linebuf ) );

  data
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input13.txt");
  let (score1, score2) = analyse_data(&data);
  println!("score1: {score1}");
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput.txt");
    assert_eq!(data.len(), 2);

    assert_eq!(reflects_vert(&data[0], 5), 0);
    assert_eq!(reflects_horiz(&data[1], 4), 0);

    assert_eq!(reflects_horiz(&data[0], 3), 1);
    assert_eq!(reflects_horiz(&data[1], 1), 1);
  }
  #[test]
  fn test_load2() {
    let data = load_data("testinput.txt");
    let (score1,score2) = analyse_data(&data);
    assert_eq!(score1, 405);
    assert_eq!(score2, 400);
  }

}
