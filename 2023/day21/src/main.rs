use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

use advent::{determine_map_dims, pos_mod, Direction, Point, TerrainMap};

use enum_iterator::all;
//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::Reverse;
//use std::cmp::{max,Reverse,Ordering};
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

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

fn analyse_data(map: &TerrainMap<char>, start: Point, steps: usize) -> (usize, usize) {
  let mut queue = VecDeque::new();
  let mut visited = HashSet::<Point>::default();

  visited.insert(start);
  queue.push_back((start, 0));

  let mut reached_count = 0;
  let mut cur_count = 0;
  while let Some((pos, count)) = queue.pop_front() {
    if count == steps {
      reached_count += 1;
    }

    if count != cur_count {
      cur_count = count;
      visited.clear();
      //println!("reset visited");
    }

    //println!("reached {pos} at {count}");
    if count >= steps {
      continue;
    }
    for d in all::<Direction>() {
      let next = pos.neighbour(d);
      if !map.dims.contains(&next) {
        continue;
      }

      if map.get(&next) == '#' {
        continue;
      }

      if visited.contains(&next) {
        continue;
      }

      visited.insert(next);
      queue.push_back((next, count + 1));
    }
  }

  (reached_count, 0)
}

fn predict_perimeter(cur_count: usize, past_perims: &[usize], past_perims_offsets: &[usize]) -> usize {
  let term2 = past_perims_offsets[cur_count % 131];
  let predicted_perim =
    ((cur_count / 131) - 1) * (past_perims[131] + term2 as usize) + past_perims[cur_count % 131 + 131];

  predicted_perim
}

fn analyse_data2(map: &TerrainMap<char>, start: Point, steps: usize) -> (usize, usize) {
  let mut queue = VecDeque::new();
  let mut visited = HashSet::<Point>::default();

  let mut reached_cache = HashMap::<usize, usize>::default();
  let mut perim_cache = HashMap::<usize, usize>::default();
  let mut past_perims = Vec::new();
  let mut past_perims_offsets = Vec::<usize>::new();

  visited.insert(start);
  queue.push_back((start, 0));

  let mut perimeter_gardens = 0;
  let mut cur_count = 0;
  while let Some((pos, count)) = queue.pop_front() {
    if count != cur_count {
      let mut interior_gardens = 0;
      if cur_count >= 2 {
        interior_gardens = *reached_cache.get(&(cur_count - 2)).unwrap();
      }
      let total_gardens = perimeter_gardens + interior_gardens;
      if cur_count < 262 {
        past_perims.push(perimeter_gardens);
        //println!("cached at {cur_count} {perimeter_gardens}");
      } else if cur_count < 393 {
        let predicted_perim = ((cur_count / 131) - 1) * past_perims[131] + past_perims[cur_count % 131 + 131];
        let offset = perimeter_gardens - predicted_perim;
        past_perims_offsets.push(offset);
      } else {
        let term2 = past_perims_offsets[cur_count % 131];
        let predicted_perim =
          ((cur_count / 131) - 1) * (past_perims[131] + term2 as usize) + past_perims[cur_count % 131 + 131];
        let offset = perimeter_gardens as isize - predicted_perim as isize;
        println!("predicted perim: {predicted_perim}, offset: {offset}");
        break;
      }
      if cur_count % 131 <= 2 {
        let mut perim_delta = 0;
        let mut inter_delta = 0;
        let mut prev_inter_delta = 0;
        if cur_count >= 131 {
          perim_delta = perimeter_gardens as isize - *perim_cache.get(&(cur_count - 131)).unwrap() as isize;
          inter_delta = interior_gardens as isize - *reached_cache.get(&(cur_count - 131)).unwrap() as isize;
        }
        if cur_count >= 262 {
          prev_inter_delta = interior_gardens as isize - *reached_cache.get(&(cur_count - 262)).unwrap() as isize;
        }
        let idd = inter_delta - prev_inter_delta;
        println!("After {cur_count} steps, perim {perimeter_gardens} + inter {interior_gardens} = {total_gardens}. perim delta {perim_delta}, interior delta {inter_delta}, prev {prev_inter_delta}, diff {idd}");
      }
      println!("After {cur_count} steps, perim {perimeter_gardens} + inter {interior_gardens} = {total_gardens}");
      reached_cache.insert(cur_count, total_gardens);
      perim_cache.insert(cur_count, perimeter_gardens);
      perimeter_gardens = 0;
      cur_count = count;
    }
    perimeter_gardens += 1;

    //println!("reached {pos} at {count}");
    if count >= steps {
      continue;
    }
    for d in all::<Direction>() {
      let next = pos.neighbour(d);
      let mut next_wrap = next;
      next_wrap.x = pos_mod(next.x, map.dims.width as isize);
      next_wrap.y = pos_mod(next.y, map.dims.height as isize);

      if map.get(&next_wrap) == '#' {
        continue;
      }

      if visited.contains(&next) {
        continue;
      }

      visited.insert(next);
      queue.push_back((next, count + 1));
    }
  }

  let interior_gardens = *reached_cache.get(&(cur_count - 2)).unwrap();
  let total_gardens = perimeter_gardens + interior_gardens;
  println!("After {cur_count} steps, perim {perimeter_gardens} + inter {interior_gardens} = {total_gardens}");

  let mut prev2 = *reached_cache.get(&(cur_count - 3)).unwrap()
    + predict_perimeter(cur_count - 1, &past_perims, &past_perims_offsets);

  let mut prev =
    *reached_cache.get(&(cur_count - 2)).unwrap() + predict_perimeter(cur_count, &past_perims, &past_perims_offsets);

  println!("prev2: {prev2}, prev: {prev}");
  loop {
    let np2 = prev2 + predict_perimeter(cur_count + 1, &past_perims, &past_perims_offsets);
    let np = prev + predict_perimeter(cur_count + 2, &past_perims, &past_perims_offsets);

    prev2 = np2;
    prev = np;
    cur_count += 2;
    if cur_count % 1_000_000 <= 2 {
      println!("{} - {prev2}, {} - {prev}", cur_count - 1, cur_count);
    }
    if cur_count >= steps {
      println!("{} - {prev2}, {} - {prev}", cur_count - 1, cur_count);
      return (prev, 0);
    }
  }

  //(total_gardens, 0)
}

fn load_data(filename: &str) -> (TerrainMap<char>, Point) {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut data = TerrainMap::<char>::new(determine_map_dims(&contents));

  let mut start_pos = Point::default();
  let mut pos = Point::default();
  for line in contents.lines() {
    for c in line.chars() {
      data.set(&pos, c);

      if c == 'S' {
        start_pos = pos;
      }

      pos.x += 1;
    }
    pos.y += 1;
    pos.x = 0;
  }

  (data, start_pos)
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
  let (data, start) = load_data("input21.txt");
  println!("width: {}, height: {}", data.dims.width, data.dims.height);
  let (score1, _) = analyse_data(&data, start, 64);
  println!("score1: {score1}");
  //let (score2, _) = analyse_data2(&data, start, 1000);
  let (score2, _) = analyse_data2(&data, start, 26501365);
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let (data, start) = load_data("testinput.txt");
    let (score1, _) = analyse_data(&data, start, 6);
    assert_eq!(score1, 16);
  }
  #[test]
  fn test_load2() {
    let (data, start) = load_data("testinput.txt");
    let (score1, _) = analyse_data2(&data, start, 6);
    assert_eq!(score1, 16);
    let (score1, _) = analyse_data2(&data, start, 10);
    assert_eq!(score1, 50);
    let (score1, _) = analyse_data2(&data, start, 50);
    assert_eq!(score1, 1594);
    let (score1, _) = analyse_data2(&data, start, 100);
    assert_eq!(score1, 6536);
    let (score1, _) = analyse_data2(&data, start, 500);
    assert_eq!(score1, 167004);
    let (score1, _) = analyse_data2(&data, start, 1000);
    assert_eq!(score1, 668697);
    let (score1, _) = analyse_data2(&data, start, 5000);
    assert_eq!(score1, 16733044);
  }
}
