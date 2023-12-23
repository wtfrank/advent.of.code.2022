use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

use advent::{determine_map_dims, Direction, Point, TerrainMap};
//use advent::{Interval, Point3};

use enum_iterator::all;
//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::Reverse;
//use std::cmp::{max,Reverse,Ordering};
use std::cmp::Ordering;
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
//use advent::Range;

/// Day 23 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn expand(map: &TerrainMap<char>, visited: &TerrainMap<bool>, point: &Point) -> Vec<Point> {
  let mut neigh = Vec::new();

  let c = map.get(point);
  if matches!(c, '^' | '>' | 'v' | '<') {
    let p = match c {
      '^' => point.neighbour(Direction::North),
      '>' => point.neighbour(Direction::East),
      'v' => point.neighbour(Direction::South),
      '<' => point.neighbour(Direction::West),
      _ => panic!("sd"),
    };

    if map.dims.contains(&p) && !visited.get(&p) && map.get(&p) != '#' {
      neigh.push(p);
    }
  } else {
    for d in all::<Direction>() {
      let p = point.neighbour(d);
      if !map.dims.contains(&p) {
        continue;
      }
      if visited.get(&p) {
        continue;
      }
      if map.get(&p) == '#' {
        continue;
      }
      neigh.push(p);
    }
  }

  neigh
}

fn expand2(map: &TerrainMap<char>, visited: &TerrainMap<bool>, point: &Point) -> Vec<Point> {
  let mut neigh = Vec::new();

  for d in all::<Direction>() {
    let p = point.neighbour(d);
    if !map.dims.contains(&p) {
      continue;
    }
    if visited.get(&p) {
      continue;
    }
    if map.get(&p) == '#' {
      continue;
    }
    neigh.push(p);
  }

  neigh
}

fn expand3(map: &TerrainMap<char>, _visited: &TerrainMap<bool>, point: &Point, prev: &Point) -> Vec<Point> {
  let mut neigh = Vec::new();

  for d in all::<Direction>() {
    let p = point.neighbour(d);
    if !map.dims.contains(&p) {
      continue;
    }
    if map.get(&p) == '#' {
      continue;
    }
    if p == *prev {
      continue;
    }
    neigh.push(p);
  }

  neigh
}

fn expand4(map: &TerrainMap<char>, point: &Point) -> Vec<Point> {
  let mut neigh = Vec::new();

  for d in all::<Direction>() {
    let p = point.neighbour(d);
    if !map.dims.contains(&p) {
      continue;
    }
    if map.get(&p) == '#' {
      continue;
    }
    neigh.push(p);
  }

  neigh
}

fn dfs(
  map: &TerrainMap<char>,
  discovered: &mut TerrainMap<bool>,
  cur: Point,
  end: Point,
  cur_len: usize,
  longest: &mut usize,
) {
  //println!("exploring {cur}");
  let new_len = cur_len + 1;
  for e in expand(map, discovered, &cur) {
    //println!("expanded: {e}");
    if e == end {
      if *longest < new_len {
        *longest = new_len;
        println!("found new longest: {new_len}");
        continue;
      }
    } else {
      discovered.set(&e, true);
      dfs(map, discovered, e, end, new_len, longest);
      discovered.set(&e, false);
    }
  }
}

fn dfs2(
  map: &TerrainMap<char>,
  discovered: &mut TerrainMap<bool>,
  cur: Point,
  end: Point,
  cur_len: usize,
  longest: &mut usize,
) {
  //println!("exploring {cur}");
  let new_len = cur_len + 1;
  for e in expand2(map, discovered, &cur) {
    //println!("expanded: {e}");
    if e == end {
      if *longest < new_len {
        *longest = new_len;
        println!("found new longest: {new_len}");
        continue;
      } else {
        println!("shorter: {new_len}. longest still {longest}");
      }
    } else {
      discovered.set(&e, true);
      dfs2(map, discovered, e, end, new_len, longest);
      discovered.set(&e, false);
    }
  }
}

fn dfs3(
  discovered: &mut TerrainMap<bool>,
  successors: &HashMap<Point, (usize, Point, Vec<Point>)>,
  cur: Point,
  end: Point,
  cur_len: usize,
  longest: &mut usize,
) {
  //println!("exploring {cur} {cur_len}");

  if let Some((seg_len, junction, nodes)) = successors.get(&cur) {
    if discovered.get(junction) {
      //println!("at {cur} but already visited junction: {junction}");
    } else {
      let new_len = cur_len + seg_len;
      for e in nodes {
        if discovered.get(e) {
          continue;
        }
        //println!("expanded: {e} at {cur} len {new_len}");
        if *e == end {
          if *longest < new_len {
            *longest = new_len;
            println!("found new longest: {new_len}");
            continue;
          } else {
            //println!("shorter: {new_len}. longest still {longest}");
          }
        } else {
          discovered.set(e, true);
          discovered.set(junction, true);
          dfs3(discovered, successors, *e, end, new_len, longest);
          discovered.set(junction, false);
          discovered.set(e, false);
        }
      }
    }
  } else {
    println!("dead end: {cur}");
  }
}

fn find_start_end(map: &TerrainMap<char>) -> (Point, Point) {
  let mut start = Point::default();
  for i in 0..map.dims.width {
    start.x = i as isize;
    if map.get(&start) == '.' {
      break;
    }
  }
  let mut end = Point {
    x: 0,
    y: map.dims.height as isize - 1,
  };
  for i in 0..map.dims.width {
    end.x = i as isize;
    if map.get(&end) == '.' {
      break;
    }
  }

  (start, end)
}

fn analyse_data(map: &TerrainMap<char>, part1: bool) -> usize {
  let mut discovered = TerrainMap::<bool>::new(map.dims);

  let (start, end) = find_start_end(map);

  let mut longest = 0;
  if part1 {
    dfs(map, &mut discovered, start, end, 0, &mut longest);
  } else {
    dfs2(map, &mut discovered, start, end, 0, &mut longest);
  }
  longest
}

fn analyse_data2(map: &TerrainMap<char>) -> usize {
  //lots of straight lines in maze, so process map into graph of nodes where path splits

  let mut successors = HashMap::<Point, (usize, Point, Vec<Point>)>::default();
  let mut visited = TerrainMap::<bool>::new(map.dims);

  let mut queue = Vec::new();
  let mut junctions = HashSet::<Point>::default();
  let (start, end) = find_start_end(map);
  queue.push((start, start));

  while let Some((p, p2)) = queue.pop() {
    let mut cur = p;
    let mut prev2 = p2;
    loop {
      println!("visiting {cur}");
      visited.set(&cur, true);
      let mut neigh = expand3(map, &visited, &cur, &prev2);
      if neigh.len() > 1 {
        junctions.insert(cur);
        println!("found junction at {cur}");
        for n in &neigh {
          if visited.get(n) {
            continue;
          }
          queue.push((*n, cur));
        }
        break;
      }

      if neigh.is_empty() {
        break;
      }

      prev2 = cur;
      cur = neigh.pop().unwrap();
    }
  }

  for j in &junctions {
    //add start/length in each direction to successors
    //starts from point after junction, ends on next junction

    let neigh = expand4(map, j);
    for n in neigh {
      let mut visited = TerrainMap::<bool>::new(map.dims);
      visited.set(j, true);
      let mut dist = 0;
      let mut cur = n;
      loop {
        dist += 1;
        visited.set(&cur, true);
        let mut n2 = expand2(map, &visited, &cur);
        match n2.len().cmp(&1) {
          Ordering::Greater => {
            println!("adding segment from {j} to {cur} of length {dist}");
            successors.insert(n, (dist, cur, n2));
            break;
          }
          Ordering::Equal => {
            cur = n2.pop().unwrap();
          }
          Ordering::Less => {
            if cur == end {
              println!("adding end segment from {j} to {cur} of length {dist}");
              successors.insert(n, (dist, cur, vec![cur]));
              break;
            } else if cur == start {
              println!("adding start segment from {j} to {cur} of length {dist}");
              successors.insert(cur, (dist, *j, expand4(map, j)));
              break;
            } else {
              break;
            }
          }
        }
      }
    }
  }

  println!("Map width={}, height={}", map.dims.width, map.dims.height);
  println!("Junctions in graph: {}", junctions.len());
  println!("Nodes in simplified graph: {}", successors.len());
  let mut longest = 0;
  let mut visited = TerrainMap::<bool>::new(map.dims);
  dfs3(&mut visited, &successors, start, end, 0, &mut longest);
  longest
}

fn load_data(filename: &str) -> TerrainMap<char> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut map = TerrainMap::<char>::new(determine_map_dims(&contents));

  let mut p = Point::default();
  for line in contents.lines() {
    for c in line.chars() {
      map.set(&p, c);

      p.x += 1;
    }

    p.x = 0;
    p.y += 1;
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

  let data = load_data("input23.txt");
  let score1 = analyse_data(&data, true);
  println!("score1: {score1}");
  let score2 = analyse_data2(&data);
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput.txt");
    let score1 = analyse_data(&data, true);
    assert_eq!(score1, 94);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput.txt");
    let score2 = analyse_data(&data, false);
    assert_eq!(score2, 154);
  }

  #[test]
  fn test_load3() {
    let data = load_data("testinput.txt");
    let score2 = analyse_data2(&data);
    assert_eq!(score2, 154);
  }
}
