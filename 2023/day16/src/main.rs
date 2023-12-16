use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

use advent::{determine_map_dims, Direction, Point, TerrainMap};

//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::{max,Reverse,Ordering};
//use std::collections::HashSet;
//use std::collections::HashMap;
//use std::collections::VecDeque;

//use std::iter::zip;

use std::collections::HashSet;
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

fn mirror_dir(mirror: char, dir: Direction) -> Direction {
  match mirror {
    '/' => match dir {
      Direction::North => Direction::East,
      Direction::East => Direction::North,
      Direction::South => Direction::West,
      Direction::West => Direction::South,
    },
    '\\' => match dir {
      Direction::North => Direction::West,
      Direction::East => Direction::South,
      Direction::South => Direction::East,
      Direction::West => Direction::North,
    },
    _ => panic!("not a mirror"),
  }
}

fn analyse_data(map: &TerrainMap<char>) -> (usize, usize) {
  let score1 = calc_energised(map, Point { x: 0, y: 0 }, Direction::East);

  let mut score2 = 0;

  let mut point = Point { x: 0, y: 0 };
  for x in 0..map.dims.width {
    point.x = x as isize;
    point.y = 0;
    let s = calc_energised(map, point, Direction::South);
    if s > score2 {
      score2 = s;
    }
    point.y = map.dims.height as isize - 1;
    let s = calc_energised(map, point, Direction::North);
    if s > score2 {
      score2 = s;
    }
  }

  for y in 0..map.dims.height {
    point.y = y as isize;
    point.x = 0;
    let s = calc_energised(map, point, Direction::East);
    if s > score2 {
      score2 = s;
    }
    point.x = map.dims.width as isize - 1;
    let s = calc_energised(map, point, Direction::West);
    if s > score2 {
      score2 = s;
    }
  }

  (score1, score2)
}

fn calc_energised(map: &TerrainMap<char>, point: Point, dir: Direction) -> usize {
  let mut energised = TerrainMap::<bool>::new(map.dims);
  let mut beam_queue = Vec::<(Point, Direction)>::new();

  let mut beam_dirs_seen = HashSet::<(Point, Direction)>::default();

  beam_queue.push((point, dir));

  while let Some((pos, dir)) = beam_queue.pop() {
    if beam_dirs_seen.contains(&(pos, dir)) {
      continue;
    }

    beam_dirs_seen.insert((pos, dir));
    energised.set(&pos, true);

    let tile = map.get(&pos);

    match tile {
      '.' => {
        let new_pos = pos.neighbour(dir);
        if map.dims.contains(&new_pos) {
          beam_queue.push((new_pos, dir));
        }
      }
      '/' => {
        let new_dir = mirror_dir(tile, dir);
        let new_pos = pos.neighbour(new_dir);
        if map.dims.contains(&new_pos) {
          beam_queue.push((new_pos, new_dir));
        }
      }
      '\\' => {
        let new_dir = mirror_dir(tile, dir);
        let new_pos = pos.neighbour(new_dir);
        if map.dims.contains(&new_pos) {
          beam_queue.push((new_pos, new_dir));
        }
      }
      '|' => {
        //pointy end
        if dir == Direction::South || dir == Direction::North {
          let new_pos = pos.neighbour(dir);
          if map.dims.contains(&new_pos) {
            beam_queue.push((new_pos, dir));
          }
        } else {
          //splits
          let new_pos1 = pos.neighbour(Direction::South);
          if map.dims.contains(&new_pos1) {
            beam_queue.push((new_pos1, Direction::South));
          }
          let new_pos2 = pos.neighbour(Direction::North);
          if map.dims.contains(&new_pos2) {
            beam_queue.push((new_pos2, Direction::North));
          }
        }
      }
      '-' => {
        //pointy end
        if dir == Direction::East || dir == Direction::West {
          let new_pos = pos.neighbour(dir);
          if map.dims.contains(&new_pos) {
            beam_queue.push((new_pos, dir));
          }
        } else {
          //splits
          let new_pos1 = pos.neighbour(Direction::East);
          if map.dims.contains(&new_pos1) {
            beam_queue.push((new_pos1, Direction::East));
          }
          let new_pos2 = pos.neighbour(Direction::West);
          if map.dims.contains(&new_pos2) {
            beam_queue.push((new_pos2, Direction::West));
          }
        }
      }
      _ => panic!("invalid map char"),
    }
  }

  let mut score = 0;
  for y in 0..energised.dims.height {
    for x in 0..energised.dims.width {
      if energised.getc(x as isize, y as isize) {
        score += 1;
      }
    }
  }

  score
}

fn load_data(filename: &str) -> TerrainMap<char> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut map = TerrainMap::<char>::new(determine_map_dims(&contents));

  for (y, line) in contents.lines().enumerate() {
    for (x, c) in line.chars().enumerate() {
      map.setc(x as isize, y as isize, c);
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
  let data = load_data("input16.txt");
  let (score1, score2) = analyse_data(&data);
  println!("score1: {score1}");
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let map = load_data("testinput.txt");
    let (score1, score2) = analyse_data(&map);
    assert_eq!(score1, 46);
    assert_eq!(score2, 51);
  }
}
