use clap::Parser;
use std::fs::File;
use std::io::Read;

use advent::{Dims, Point, TerrainMap};

use enum_iterator::{all, Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::{max,Reverse,Ordering};
use std::collections::HashMap;
use std::collections::HashSet;

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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let (mut data, start) = load_data("testinput1.txt");
    let (score, loop_map) = analyse_data(&mut data, start);
    assert_eq!(score, 4);
    let score2 = find_inside(&data, &loop_map);
    assert_eq!(score2, 1);
  }
  #[test]
  fn test_load2() {
    let (mut data, start) = load_data("testinput2.txt");
    let (score, loop_map) = analyse_data(&mut data, start);
    assert_eq!(score, 8);
    let score2 = find_inside(&data, &loop_map);
    assert_eq!(score2, 1);
  }
  #[test]
  fn test_load3() {
    let (mut data, start) = load_data("testinput3.txt");
    let (_score, loop_map) = analyse_data(&mut data, start);
    let score2 = find_inside(&data, &loop_map);
    assert_eq!(score2, 4);
  }
  #[test]
  fn test_load4() {
    let (mut data, start) = load_data("testinput4.txt");
    let (_score, loop_map) = analyse_data(&mut data, start);
    let score2 = find_inside(&data, &loop_map);
    assert_eq!(score2, 8);
  }
  #[test]
  fn test_load5() {
    let (mut data, start) = load_data("testinput5.txt");
    let (_score, loop_map) = analyse_data(&mut data, start);
    let score2 = find_inside(&data, &loop_map);
    assert_eq!(score2, 10);
  }
}

//pushed edges may not be valid points
//could implement this as a state machine iterator
fn adjacent_edges(p: Point, pipe: Pipe) -> Vec<Point> {
  let mut edges = Vec::new();

  match pipe {
    Pipe::Ground => (),
    Pipe::NS => {
      edges.push(Point { x: p.x, y: p.y - 1 });
      edges.push(Point { x: p.x, y: p.y + 1 });
    }
    Pipe::EW => {
      edges.push(Point { x: p.x - 1, y: p.y });
      edges.push(Point { x: p.x + 1, y: p.y });
    }
    Pipe::NE => {
      edges.push(Point { x: p.x, y: p.y - 1 });
      edges.push(Point { x: p.x + 1, y: p.y });
    }
    Pipe::NW => {
      edges.push(Point { x: p.x, y: p.y - 1 });
      edges.push(Point { x: p.x - 1, y: p.y });
    }
    Pipe::SW => {
      edges.push(Point { x: p.x, y: p.y + 1 });
      edges.push(Point { x: p.x - 1, y: p.y });
    }
    Pipe::SE => {
      edges.push(Point { x: p.x, y: p.y + 1 });
      edges.push(Point { x: p.x + 1, y: p.y });
    }
    Pipe::NESW => {
      edges.push(Point { x: p.x, y: p.y - 1 });
      edges.push(Point { x: p.x + 1, y: p.y });
      edges.push(Point { x: p.x, y: p.y + 1 });
      edges.push(Point { x: p.x - 1, y: p.y });
    }
  }

  edges
}

fn analyse_data(map: &mut TerrainMap<Pipe>, start: Point) -> (usize, TerrainMap<bool>) {
  let mut stack = Vec::<Point>::new();
  let mut visited = HashSet::<Point>::default();
  let mut dist = HashMap::<Point, usize>::default();

  stack.push(start);
  dist.insert(start, 0);

  while let Some(v) = stack.pop() {
    if visited.contains(&v) {
      continue;
    }

    visited.insert(v);

    for w in adjacent_edges(v, map.get(&v)) {
      //println!("checking adjacent {v} {:?} {w}", map.get(&v));
      if w.x < 0
        || w.y < 0
        || w.x >= map.dims.minx + map.dims.width as isize
        || w.y >= map.dims.miny + map.dims.height as isize
      {
        continue;
      }
      if !adjacent_edges(w, map.get(&w)).contains(&v) {
        //println!("skipping non-adjacent edge {v} {:?} {w} {:?}", map.get(&v), map.get(&w));
        continue;
      }

      let old_dist = *dist.get(&v).unwrap();
      if dist.contains_key(&w) {
        //println!("already contained distance {} vs {} for {w} (coming from {v})", *dist.get(&w).unwrap(), old_dist+1);
      }

      if !dist.contains_key(&w) || *dist.get(&w).unwrap() != old_dist - 1 {
        //don't add the distance for going A->B->A again
        dist.insert(w, old_dist + 1);
      }

      //println!("pushing {w} with dist {}", old_dist+1);

      stack.push(w);
    }
  }

  let mut max_dist = 0;
  let mut max_dist_point = start;

  for w in adjacent_edges(start, map.get(&start)) {
    if w.x < 0
      || w.y < 0
      || w.x >= map.dims.minx + map.dims.width as isize
      || w.y >= map.dims.miny + map.dims.height as isize
    {
      continue;
    }
    if !adjacent_edges(w, map.get(&w)).contains(&start) {
      continue;
    }

    let d = *dist.get(&w).unwrap();
    //println!("checking {w} dist {d}");
    if d > max_dist {
      max_dist = d;
      max_dist_point = w;
    }
  }

  assert_ne!(start, max_dist_point);
  println!("max dist point is {max_dist_point}: {max_dist}");

  if max_dist % 2 == 1 {
    max_dist += 1;
  }

  let furthest_point = max_dist / 2;

  let mut loop_map = TerrainMap::<bool>::new(map.dims);

  let mut cur = max_dist_point;
  let cur_dist = *dist.get(&cur).unwrap();
  let mut prev = start;

  println!("starting loop search at {cur} dist {cur_dist}, prev{prev}");

  loop_map.set(&start, true);
  loop_map.set(&max_dist_point, true);
  loop {
    let mut found = false;
    //println!("find adjacent: cur {cur}, {:?}", map.get(&cur));
    for w in adjacent_edges(cur, map.get(&cur)) {
      if w.x < 0
        || w.y < 0
        || w.x >= map.dims.minx + map.dims.width as isize
        || w.y >= map.dims.miny + map.dims.height as isize
      {
        continue;
      }
      //println!("expand: w{w}, prev{prev}, cur{cur}, {:?}", map.get(&cur));
      if w == prev {
        continue;
      }
      //println!("found {w} from {cur} (prev {prev})");
      prev = cur;
      cur = w;
      loop_map.set(&cur, true);
      found = true;
      break;
    }
    assert!(found);
    if cur == max_dist_point || map.get(&cur) == Pipe::NESW {
      break;
    }
  }

  //change start into it's true location
  let mut found = false;
  for p in all::<Pipe>() {
    if p == Pipe::Ground {
      continue;
    }
    if p == Pipe::NESW {
      continue;
    }

    let mut all_loop = true;
    let mut valid = true;
    for w in adjacent_edges(cur, p) {
      if w.x < 0
        || w.y < 0
        || w.x >= map.dims.minx + map.dims.width as isize
        || w.y >= map.dims.miny + map.dims.height as isize
      {
        all_loop = false;
        continue;
      }

      if !loop_map.get(&w) {
        all_loop = false;
      }

      if !adjacent_edges(w, map.get(&w)).contains(&start) {
        valid = false;
      }
    }
    if all_loop && valid {
      println!("Start was of type {p:?}");
      map.set(&start, p);
      found = true;
      break;
    }
  }
  assert!(found);

  /*
  loop {
    loop_map.set(&cur, true);
    println!("added {cur} to loop at dist {cur_dist}");

   if cur == start {
      break;
    }

    let mut found = false;
    for w in adjacent_edges(cur, map.get(&cur)) {
      if w.x < 0 || w.y < 0 ||
        w.x >= map.dims.minx + map.dims.width as isize ||
          w.y >= map.dims.miny + map.dims.height as isize {
        continue
      }
      let d = *dist.get(&w).unwrap();
      println!("checking dist of {w}: {d}");
      if d + 1 == cur_dist || w == start {
        cur = w;
        cur_dist = d;
        found = true;
      }
    }
    assert_eq!(found, true);
  }*/

  (furthest_point, loop_map)
}

fn find_inside(map: &TerrainMap<Pipe>, loop_map: &TerrainMap<bool>) -> usize {
  //send a ray across
  let mut out_count = 0;
  let mut ins_count = 0;

  let mut outside;
  let mut loop_length = 0;
  for y in 0..map.dims.height {
    let mut run_start = Pipe::Ground;
    let mut inside_run = false;
    outside = true;
    for x in 0..map.dims.width {
      let p = Point {
        x: x as isize,
        y: y as isize,
      };

      let on_loop = loop_map.get(&p);
      if on_loop {
        loop_length += 1;

        if map.get(&p) == Pipe::NS {
          assert!(!inside_run);
          outside = !outside;
          continue;
        }

        if !inside_run {
          inside_run = true;
          run_start = map.get(&p);
          assert!(run_start == Pipe::NS || run_start == Pipe::NE || run_start == Pipe::SE);
        } else {
          let cur_pipe = map.get(&p);
          if cur_pipe == Pipe::EW {
            continue;
          }

          match run_start {
            Pipe::NS => {
              run_start = cur_pipe;
              outside = !outside;
            }
            Pipe::NE => match cur_pipe {
              Pipe::SW => {
                outside = !outside;
                inside_run = false;
              }
              Pipe::NW => {
                inside_run = false;
              }
              _ => panic!("unexpected pipe in run {cur_pipe:?}"),
            },
            Pipe::SE => match cur_pipe {
              Pipe::NW => {
                outside = !outside;
                inside_run = false;
              }
              Pipe::SW => {
                inside_run = false;
              }
              _ => panic!("unexpected pipe in run {cur_pipe:?}"),
            },
            _ => {
              panic!("unexpected start run of pipe {run_start:?}")
            }
          }
        }

        continue;
      }
      match outside {
        true => {
          out_count += 1;
        }
        false => {
          ins_count += 1;
          println!("Found inside at {p}");
        }
      }
    }
  }

  println!("outside {out_count}, inside {ins_count}, loop_length {loop_length}");

  println!("{loop_map:?}");

  assert_eq!(loop_length + out_count + ins_count, map.dims.width * map.dims.height);

  ins_count
}

#[allow(dead_code)]
fn analyse_data_bfs(map: &TerrainMap<Pipe>, start: Point) -> usize {
  let mut queue = Vec::<Point>::new();
  let mut idx = 0; //increment when we remove something
  let mut visited = HashSet::<Point>::default();
  let mut dist = HashMap::<Point, usize>::default();
  let mut prev = HashMap::<Point, Point>::default();

  visited.insert(start);
  queue.push(start);
  dist.insert(start, 0);

  while idx < queue.len() {
    let v = *queue.get(idx).unwrap();
    idx += 1;

    if v == start {
      break;
    }

    for w in adjacent_edges(v, map.get(&v)) {
      if w.x < 0
        || w.y < 0
        || w.x >= map.dims.minx + map.dims.width as isize
        || w.y >= map.dims.miny + map.dims.height as isize
      {
        continue;
      }
      if !visited.contains(&w) {
        visited.insert(w);
        prev.insert(w, v);
        dist.insert(w, dist.get(&v).unwrap() + 1);
        queue.push(w);
      }
    }
  }

  0
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Default, Copy, Clone, PartialEq, Debug, Sequence)]
enum Pipe {
  #[default]
  Ground,
  NS,
  EW,
  NE,
  NW,
  SW,
  SE,
  NESW,
}

fn char_to_pipe(c: char) -> Pipe {
  match c {
    '.' => Pipe::Ground,
    '|' => Pipe::NS,
    '-' => Pipe::EW,
    'L' => Pipe::NE,
    'J' => Pipe::NW,
    '7' => Pipe::SW,
    'F' => Pipe::SE,
    'S' => Pipe::NESW,
    _ => panic!("unexpected char {c}"),
  }
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

fn load_data(filename: &str) -> (TerrainMap<Pipe>, Point) {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut map = TerrainMap::<Pipe>::new(determine_map_dims(&contents));

  let mut start: Point = Default::default();
  let mut p = Point { x: 0, y: 0 };
  for line in contents.lines() {
    for c in line.chars() {
      let cp = char_to_pipe(c);
      map.set(&p, cp);
      if cp == Pipe::NESW {
        println!("found start: {p}");
        start = p;
      }

      p.x += 1;
    }
    p.x = 0;
    p.y += 1;
  }

  //sequences.push( line.split(' ').map( |a| a.parse::<isize>().unwrap() ).collect() );
  //let r = sscanf::sscanf!(line, "{String} = ({String}, {String})").unwrap();
  (map, start)
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let (mut data, start) = load_data("input10.txt");
  let (score1, loop_map) = analyse_data(&mut data, start);
  println!("score1: {score1}");
  let score2 = find_inside(&data, &loop_map);
  println!("score2: {score2}");
}
