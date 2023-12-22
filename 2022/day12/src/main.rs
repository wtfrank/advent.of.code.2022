use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

use advent::{Dims, Point, TerrainMap};

fn trace_back(parents: &TerrainMap<Point>, start: Point, end: Point) -> usize {
  let mut cur = start;
  let mut dist: usize = 0;

  //println!("{}: {}", dist, cur);
  while cur != end {
    cur = parents.get(&cur);
    dist += 1;
    //println!("{}: {}", dist, cur);
  }

  dist
}

/// breadth first search
fn shortest_path(map: &TerrainMap<usize>, from: Point, to: Point) -> usize {
  let mut queue = VecDeque::<Point>::new();
  let mut visited = TerrainMap::<bool>::new(map.dims);
  let mut prev = TerrainMap::<Point>::new(map.dims);

  visited.set(&from, true);
  queue.push_back(from);
  while !queue.is_empty() {
    let v = queue.pop_front().unwrap();
    if v == to {
      return trace_back(&prev, to, from);
    }

    //for d in [(-1,-1), (0,-1), (1,-1), (-1,0), (1, 0), (-1,1), (0,1), (1,1)] {
    for d in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
      let new = Point {
        x: v.x + d.0,
        y: v.y + d.1,
      };
      if new.x < 0 || new.y < 0 {
        continue;
      }
      if new.x as usize >= map.dims.width || new.y as usize >= map.dims.height {
        continue;
      }
      if visited.get(&new) {
        continue;
      }
      if map.get(&new) > map.get(&v) + 1 {
        continue;
      }
      visited.set(&new, true);
      prev.set(&new, v);
      queue.push_back(new);
    }
  }

  0
}

/// now search from highest to any node with height 0
fn shortest_path_to_goal(map: &TerrainMap<usize>, from: Point) -> usize {
  let mut queue = VecDeque::<Point>::new();
  let mut visited = TerrainMap::<bool>::new(map.dims);
  let mut prev = TerrainMap::<Point>::new(map.dims);

  visited.set(&from, true);
  queue.push_back(from);
  while !queue.is_empty() {
    let v = queue.pop_front().unwrap();
    if map.get(&v) == 0 {
      return trace_back(&prev, v, from);
    }

    for d in [(0, -1), (-1, 0), (1, 0), (0, 1)] {
      let new = Point {
        x: v.x + d.0,
        y: v.y + d.1,
      };
      if new.x < 0 || new.y < 0 {
        continue;
      }
      if new.x as usize >= map.dims.width || new.y as usize >= map.dims.height {
        continue;
      }
      if visited.get(&new) {
        continue;
      }
      if map.get(&new) < map.get(&v) - 1 {
        continue;
      }
      visited.set(&new, true);
      prev.set(&new, v);
      queue.push_back(new);
    }
  }

  0
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

fn load_terrain(filename: &str) -> (TerrainMap<usize>, Point, Point) {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut tm = TerrainMap::<usize>::new(determine_map_dims(&contents));

  let mut pos = Point { x: 0, y: 0 };
  let mut start = Point { x: 0, y: 0 };
  let mut end = Point { x: 0, y: 0 };
  for l in contents.lines() {
    for mut c in l.chars() {
      if c == 'S' {
        start = pos;
        c = 'a';
      } else if c == 'E' {
        end = pos;
        c = 'z';
      }

      let elevation = c.to_digit(36).unwrap() - 10;
      tm.set(&pos, elevation as usize);

      pos.x += 1;
    }

    pos.x = 0;
    pos.y += 1;
  }

  (tm, start, end)
}

fn main() -> std::io::Result<()> {
  let (tm, start, end) = load_terrain("input12.txt");

  println!("shortest: {}", shortest_path(&tm, start, end));
  println!("shortest to goal: {}", shortest_path_to_goal(&tm, end));

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_map() {
    let (tm, start, end) = load_terrain("testinput.txt");
    assert!(start.x == 0);
    assert!(start.y == 0);
    assert!(end.x == 5);
    assert!(end.y == 2);

    assert!(tm.get(&start) == 0);
    assert!(tm.get(&end) == 25);
    assert!(tm.get(&Point { x: 2, y: 0 }) == 1);
  }

  #[test]
  fn test_path() {
    let (tm, start, end) = load_terrain("testinput.txt");
    let dist = shortest_path(&tm, start, end);
    assert_eq!(dist, 31);
  }
  #[test]
  fn test_path_goal() {
    let (tm, _start, end) = load_terrain("testinput.txt");
    let dist = shortest_path_to_goal(&tm, end);
    assert_eq!(dist, 29);
  }
  #[test]
  fn test_inspections() {}
  #[test]
  fn test_inspections_lotsofworry() {}
}
