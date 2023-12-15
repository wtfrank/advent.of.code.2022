use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

use advent::{determine_map_dims, Point, TerrainMap};

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

fn tip_n(data: &mut TerrainMap<char>) -> usize {
  let mut load = 0;

  for x in 0..data.dims.width {
    let mut first_free = 0;
    for y in 0..data.dims.height {
      match data.get(&Point {
        x: x as isize,
        y: y as isize,
      }) {
        '.' => {
          //first_free = y;
        }
        '#' => {
          first_free = y + 1;
        }
        'O' => {
          let mut pos = y;
          if first_free <= y {
            pos = first_free;
            first_free += 1;
          }
          if pos < y {
            data.set(
              &Point {
                x: x as isize,
                y: pos as isize,
              },
              'O',
            );
            data.set(
              &Point {
                x: x as isize,
                y: y as isize,
              },
              '.',
            );
          }
          let weight = data.dims.height - pos;
          //println!("adding weight {weight}");
          load += weight;
        }
        _ => {
          panic!("unexpected char");
        }
      }
    }
  }

  load
}

fn tip_s(data: &mut TerrainMap<char>) -> usize {
  let mut load = 0;

  for x in 0..data.dims.width {
    let mut first_free: isize = data.dims.height as isize - 1;
    for y2 in 0..data.dims.height {
      let y = data.dims.height - y2 - 1;
      match data.get(&Point {
        x: x as isize,
        y: y as isize,
      }) {
        '.' => {
          //first_free = y;
        }
        '#' => {
          first_free = y as isize - 1;
        }
        'O' => {
          let mut pos = y as isize;
          if first_free >= y as isize {
            pos = first_free;
            first_free -= 1;
          }
          if pos > y as isize {
            data.set(&Point { x: x as isize, y: pos }, 'O');
            data.set(
              &Point {
                x: x as isize,
                y: y as isize,
              },
              '.',
            );
          }
          let weight = data.dims.height - pos as usize;
          load += weight;
        }
        _ => {
          panic!("unexpected char");
        }
      }
    }
  }

  load
}

fn tip_w(data: &mut TerrainMap<char>) -> usize {
  let mut load = 0;

  for y in 0..data.dims.height {
    let mut first_free = 0;
    for x in 0..data.dims.width {
      match data.get(&Point {
        x: x as isize,
        y: y as isize,
      }) {
        '.' => {}
        '#' => {
          first_free = x + 1;
        }
        'O' => {
          let mut pos = x;
          if first_free <= x {
            pos = first_free;
            first_free += 1;
          }
          if pos < x {
            data.set(
              &Point {
                x: pos as isize,
                y: y as isize,
              },
              'O',
            );
            data.set(
              &Point {
                x: x as isize,
                y: y as isize,
              },
              '.',
            );
          }
          let weight = data.dims.width - pos;
          load += weight;
        }
        _ => {
          panic!("unexpected char");
        }
      }
    }
  }

  load
}

fn tip_e(data: &mut TerrainMap<char>) -> usize {
  let mut load = 0;

  for y in 0..data.dims.height {
    let mut first_free = data.dims.width as isize - 1;
    for x2 in 0..data.dims.width {
      let x = data.dims.width - x2 - 1;
      match data.get(&Point {
        x: x as isize,
        y: y as isize,
      }) {
        '.' => {}
        '#' => {
          first_free = x as isize - 1;
        }
        'O' => {
          let mut pos = x as isize;
          if first_free >= x as isize {
            pos = first_free;
            first_free -= 1;
          }
          if pos > x as isize {
            data.set(&Point { x: pos, y: y as isize }, 'O');
            data.set(
              &Point {
                x: x as isize,
                y: y as isize,
              },
              '.',
            );
          }
          let weight = data.dims.width as isize - pos;
          load += weight as usize;
        }
        _ => {
          panic!("unexpected char");
        }
      }
    }
  }

  load
}

fn calc_load(data: &TerrainMap<char>) -> usize {
  let mut load = 0;
  for x in 0..data.dims.width {
    for y in 0..data.dims.height {
      if data.get(&Point {
        x: x as isize,
        y: y as isize,
      }) == 'O'
      {
        load += data.dims.height - y;
      }
    }
  }
  load
}

fn cycle(data: &mut TerrainMap<char>) -> usize {
  tip_n(data);
  tip_w(data);
  tip_s(data);
  tip_e(data);
  calc_load(data)
}

fn check_for_cycle(data: &Vec<usize>) -> (bool, usize) {
  //subtract array at various offsets
  //to establish a cycle, we want to see 5 repeats

  let max_cycle_len = data.len() / 5;
  for c in 2..max_cycle_len {
    let mut found = true;
    let start = data.len() - 4 * c;
    for i in start..data.len() {
      if data[i] != data[i - c] {
        found = false;
        break;
      }
    }
    if found {
      println!("found cycle of length {c}");
      return (true, c);
    }
  }
  (false, 0)
}

fn analyse_data(data: &mut TerrainMap<char>) -> (usize, usize) {
  let load = tip_n(data);

  let mut loads = Vec::new();
  let mut cycle_value;
  let mut count = 0;
  loop {
    count += 1;
    let load2 = cycle(data);
    loads.push(load2);
    println!("score: {load2} {count}");

    let cycle_found;
    (cycle_found, cycle_value) = check_for_cycle(&loads);

    if cycle_found {
      // we need to find the value at position 10e9 % cycle + 5*cycle
      let pos = 4 * cycle_value - 1 + 1_000_000_000 % cycle_value;
      let pos_value = *loads.get(pos).unwrap();
      println!("pos {pos}, value{pos_value}");
      return (load, pos_value);
    }
  }
}

fn load_data(filename: &str) -> TerrainMap<char> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut map = TerrainMap::new(determine_map_dims(&contents));

  let mut p = Point {
    y: 0,
    ..Default::default()
  };
  for line in contents.lines() {
    p.x = 0;
    for c in line.chars() {
      map.set(&p, c);
      p.x += 1;
    }
    p.y += 1;
    p.x = 0;
    //sequences.push( line.split(' ').map( |a| a.parse::<isize>().unwrap() ).collect() );
    //let r = sscanf::sscanf!(line, "{String} = ({String}, {String})").unwrap();
  }

  map
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let mut data = load_data("input14.txt");
  let (score1, score2) = analyse_data(&mut data);
  println!("score1: {score1}");
  println!("score2: {score2}");
}

#[cfg(test)]
fn nicely(data: &TerrainMap<char>) -> String {
  let mut s = String::new();

  let mut p = Point::default();

  for y in 0..data.dims.height {
    p.y = y as isize;
    for x in 0..data.dims.width {
      p.x = x as isize;
      s.push(data.get(&p));
    }
    s.push('\n');
  }
  s
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let mut data = load_data("testinput.txt");
    let (score1, _score2) = analyse_data(&mut data);
    assert_eq!(score1, 136);
  }
  #[test]
  fn test_load2() {
    let mut data = load_data("testinput.txt");

    /*
        let mut load2 = cycle(data);
        print_nicely(data);
        println!("score: {load2}");
        load2 = cycle(data);
        print_nicely(data);
        println!("score: {load2}");
        load2 = cycle(data);
        print_nicely(data);
        println!("score: {load2}");
    */
    let (_score1, score2) = analyse_data(&mut data);
    assert_eq!(score2, 64);
  }

  #[test]
  fn test_load3() {
    let mut data = load_data("testinput.txt");
    cycle(&mut data);
    let s = nicely(&data);
    let s2 = r#".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....
"#;

    assert_eq!(s, s2);
    cycle(&mut data);
    let s3 = nicely(&data);
    let s4 = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O
"#;
    assert_eq!(s3, s4);

    cycle(&mut data);
    let s5 = nicely(&data);
    let s6 = r#".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O
"#;
    assert_eq!(s5, s6);
  }
}
