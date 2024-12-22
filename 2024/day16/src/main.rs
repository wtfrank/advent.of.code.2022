#![feature(test)]
extern crate test;

use clap::Parser;
#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;
//use itertools::sorted;
//use std::collections::VecDeque;

#[allow(unused_imports)]
use advent::{Dims, Direction, Point, TerrainMap};

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

use priority_queue::PriorityQueue;
use std::cmp::Reverse;

#[allow(unused_imports)]
use enum_iterator::all;

/// Day 16 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

#[derive(Default, Clone, Copy, PartialEq)]
enum MapEntity {
  #[default]
  Empty,
  Wall,
}

type PointDir = (Point, Direction);

const TURN_COST: usize = 1000;
fn analyse_input1(puzzle_input: &str) -> usize {
  let mut height = 0;
  for l in puzzle_input.lines() {
    if l.is_empty() {
      break;
    }
    height += 1;
  }
  let dims = Dims {
    height,
    width: puzzle_input.lines().next().unwrap().len(),
    ..Default::default()
  };

  println!("Height {}, Width {}", dims.height, dims.width);

  let mut tm = TerrainMap::<MapEntity>::new(dims);

  let mut p = Point::default();
  let mut start: Option<Point> = None;
  let mut end: Option<Point> = None;

  for line in puzzle_input.lines() {
    for c in line.chars() {
      let e = match c {
        '#' => MapEntity::Wall,
        '.' => MapEntity::Empty,
        'S' => {
          start = Some(p);
          MapEntity::Empty
        }
        'E' => {
          end = Some(p);
          MapEntity::Empty
        }

        _ => panic!("unexpected char"),
      };
      tm.set(&p, e);
      p.x += 1;
    }
    p.y += 1;
    p.x = 0;
  }

  let start = start.expect("robot start loc not found");
  let end = end.expect("robot end loc not found");
  let start_dir = Direction::East;

  let mut nodes = Vec::<Point>::new();
  let mut nodes2 = HashSet::<PointDir>::default(); // every direction you enter a point from, or leave it in the direction of
  nodes.push(start);
  nodes.push(end);
  nodes2.insert((start, start_dir));

  let neigh = neighbour_dirs(start, &tm);
  for d in neigh {
    if d != Direction::East {
      nodes2.insert((start, d));
      nodes2.insert((start, d.reverse()));
    }
  }

  // process maze into graph
  let mut p = Point { x: 0, y: 0 };
  for y in 0..dims.height {
    p.y = y as isize;
    for x in 0..dims.width {
      p.x = x as isize;
      if tm.get(&p) == MapEntity::Wall {
        continue;
      }
      let neigh = neighbour_dirs(p, &tm);
      if neigh.len() > 2 && p != start && p != end {
        nodes.push(p);
      }

      if p == end {
        // can only enter the end but not leave it
        for d in neigh {
          nodes2.insert((p, d.reverse()));
        }
      } else if neigh.len() > 2 {
        for d in neigh {
          nodes2.insert((p, d));
          nodes2.insert((p, d.reverse()));
        }
      }
      //}
    }
  }

  println!("{} nodes in the graph. Start: {start}. End: {end}.", nodes.len());

  let mut procd = HashMap::<Point, Vec<(Direction, Point, Direction, usize)>>::default();
  let mut procd2 = HashMap::<(Point, Direction), (Point, Direction, usize)>::default();

  /*
  for d in all::<Direction>() {
    if d == start_dir {
      continue;
    }

    let n = start.neighbour(d);
    if !tm.dims.contains(&n) {
      continue;
    }
    if tm.get(&n) == MapEntity::Wall {
      continue;
    }

    let cost = if d == start_dir.reverse() {
      2 * TURN_COST
    } else {
      TURN_COST
    };

    let vec = procd.entry(start).or_insert(Vec::new());
    let fw = (start_dir, start, d, cost);
    vec.push(fw);

    procd2.insert((start, start_dir), (start, d, cost));
  }*/

  for node in nodes.iter() {
    for d in all::<Direction>() {
      let n = node.neighbour(d);
      if !tm.dims.contains(&n) {
        continue;
      }
      if tm.get(&n) == MapEntity::Wall {
        continue;
      }
      let mut cost = 1;
      let mut cur_dir = d;
      let mut cur = n;
      loop {
        let exit_dirs = neighbour_dirs(cur, &tm);
        let qty = exit_dirs.len();
        if n == end || n == start || qty > 2 {
          // live end
          println!("new segment {node} {d:?} to {cur} {cur_dir:?} cost {cost}");
          let vec = procd.entry(*node).or_insert(Vec::new());
          let fw = (d, cur, cur_dir, cost);
          if !vec.contains(&fw) {
            vec.push(fw);
          } else {
            // println!("avoided adding dup fw");
          }
          procd2.insert((*node, d), (cur, cur_dir, cost));
          let bw = (cur_dir.reverse(), *node, d.reverse(), cost);
          let vec = procd.entry(cur).or_insert(Vec::new());
          if !vec.contains(&bw) {
            vec.push(bw);
          } else {
            // println!("avoided adding dup bw");
          }
          procd2.insert((cur, cur_dir.reverse()), (*node, d.reverse(), cost));
          break;
        } else if qty < 2 {
          println!("dead end at {cur}");
          // dead end
          break;
        }

        // there is one exit, move along it
        let mut exit_found = false;
        for e in exit_dirs {
          if e != cur_dir.reverse() {
            if e != cur_dir {
              cost += TURN_COST;
              cur_dir = e;
            }
            exit_found = true;
            break;
          }
        }
        assert!(exit_found);

        cur = cur.neighbour(cur_dir);
        cost += 1;
      }
    }
  }

  // assert!(procd2.contains_key(&(start, start_dir)));

  println!("{:?}", procd.get(&start));
  println!("{:?}", procd.get(&end));

  /*
  for d in all::<Direction>() {
    let n = end.neighbour(d);
    if tm.get(&n) == MapEntity::Wall {
      continue;
    }
    let r = d.reverse();
    println!("checking end {r:?}");
    assert!(procd2.contains_key(&(end, r)));
  }*/

  // now DFS search along this graph
  // ran for 8 hours. I did put duplicate nodes into
  // the graph which conceivably created loops.
  // but still ran for more than a few seconds when I removed them
  /*
  let mut visited = HashSet::<Point>::default();
  visited.insert(start);
  let start_dir = Direction::East;
  let start_cost = 0;
  let (cost, possible) = find_cheapest_path(start, start_dir, end, start_cost, &procd, &mut visited);
  assert!(possible);
  */

  // Dijkstra
  let mut dist = HashMap::<PointDir, usize>::default();
  let mut prev = HashMap::<PointDir, Option<PointDir>>::default();
  let mut pq = PriorityQueue::<PointDir, Reverse<usize>>::default();

  for k in nodes2.iter() {
    dist.insert(*k, usize::MAX);
    prev.insert(*k, None);
    pq.push(*k, Reverse(usize::MAX));
  }

  let sp = (start, Direction::East);
  dist.insert(sp, 0);
  let old = pq.change_priority(&sp, Reverse(0));
  assert!(old.is_some());

  while let Some(((pos, dir), priority)) = pq.pop() {
    println!("Considering {pos} {dir:?} {}", priority.0);
    if priority == Reverse(usize::MAX) {
      println!("Somehow popped something unreachable {pos} {dir:?} off the queue before reaching dest");
    }

    if pos == end {
      return priority.0;
    }

    let vc = procd.get(&pos).unwrap();

    for (entry_dir, exit, exit_dir, exit_cost) in vc.iter() {
      if dir.reverse() == *entry_dir {
        continue;
      }

      let ed = (*exit, *exit_dir);

      assert_eq!(priority.0, *dist.get(&(pos, dir)).unwrap());
      let cost = priority.0 + exit_cost + if *entry_dir == dir { 0 } else { TURN_COST };
      if !dist.contains_key(&ed) {
        println!("{ed:?} not in dist");
        for d in all::<Direction>() {
          if d == ed.1 {
            continue;
          }
          if !dist.contains_key(&(*exit, d)) {
            println!("also {exit}, {d:?}");
          }
        }
      }
      if cost < *dist.get(&ed).unwrap() {
        dist.insert(ed, cost);
        prev.insert(ed, Some((pos, dir)));
        pq.change_priority(&ed, Reverse(cost));
      }
    }
  }

  panic!("Didn't reach goal");
}

/*
fn find_cheapest_path(
  start: Point,
  dir: Direction,
  end: Point,
  cost: usize,
  nodes: &HashMap<Point, Vec<(Direction, Point, Direction, usize)>>,
  visited: &mut HashSet<Point>,
) -> (usize, bool) {
  let mut route_exists = false;
  let mut best_cost = usize::MAX;
  for (in_dir, out, out_dir, seg_cost) in nodes.get(&start).unwrap() {
    if dir == in_dir.reverse() || visited.contains(out) {
      continue;
    }
    // visited check covers going back the way we came although it is 10% quicker to do a reverse direction test
    let mut cost_so_far = cost + seg_cost;
    if *in_dir != dir {
      // can only be 90 degree turn or we'd be going back on ourselves
      cost_so_far += TURN_COST;
    }

    if *out == end {
      route_exists = true;
      if cost_so_far < best_cost {
        best_cost = cost_so_far;
      }
    } else {
      visited.insert(*out);
      let (cost2, route_exists2) = find_cheapest_path(*out, *out_dir, end, cost_so_far, nodes, visited);
      visited.remove(out);

      if route_exists2 {
        route_exists = true;
        if cost2 < best_cost {
          best_cost = cost2;
        }
      }
    }
  }

  // println!("{start},{dir:?}-{end} exists: {route_exists}");

  (best_cost, route_exists)
}
*/

fn find_path_routes(
  start: Point,
  dir: Direction,
  end: Point,
  cost: usize,
  nodes: &HashMap<Point, Vec<(Direction, Point, Direction, usize)>>,
  visited: &mut Vec<Point>,
  max_cost: usize,
) -> (usize, bool) {
  let mut route_exists = false;
  let mut best_cost = usize::MAX;
  for (in_dir, out, out_dir, seg_cost) in nodes.get(&start).unwrap() {
    if dir == in_dir.reverse() {
      // || visited.contains(out) {
      continue;
    }
    // visited check covers going back the way we came although it is 10% quicker to do a reverse direction test
    let mut cost_so_far = cost + seg_cost;
    if *in_dir != dir {
      // can only be 90 degree turn or we'd be going back on ourselves
      cost_so_far += TURN_COST;
    }

    if cost_so_far > max_cost {
      continue;
    }

    if *out == end {
      route_exists = true;
      if cost_so_far < best_cost {
        best_cost = cost_so_far;
      }
    } else {
      visited.push(*out);
      let (cost2, route_exists2) = find_path_routes(*out, *out_dir, end, cost_so_far, nodes, visited, max_cost);
      visited.pop();

      if route_exists2 {
        route_exists = true;
        if cost2 < best_cost {
          best_cost = cost2;
        }
      }
    }
  }

  // println!("{start},{dir:?}-{end} exists: {route_exists}");

  (best_cost, route_exists)
}

fn neighbour_dirs(p: Point, tm: &TerrainMap<MapEntity>) -> Vec<Direction> {
  let mut dirs = Vec::new();
  for d in all::<Direction>() {
    let n = p.neighbour(d);
    if tm.dims.contains(&n) && tm.get(&n) != MapEntity::Wall {
      dirs.push(d);
    }
  }
  dirs
}

#[allow(dead_code)]
fn draw_map(map: &TerrainMap<MapEntity>, robot: &Point) {
  for y in 0..map.dims.height {
    let mut chars = Vec::<char>::new();
    for x in 0..map.dims.width {
      if robot.x == x as isize && robot.y == y as isize {
        chars.push('@');
      } else {
        chars.push(match map.getc(x as isize, y as isize) {
          MapEntity::Empty => '.',
          MapEntity::Wall => '#',
        });
      }
    }
    let s: String = chars.into_iter().collect();
    println!("{s}");
  }
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut height = 0;
  for l in puzzle_input.lines() {
    if l.is_empty() {
      break;
    }
    height += 1;
  }
  let dims = Dims {
    height,
    width: puzzle_input.lines().next().unwrap().len(),
    ..Default::default()
  };

  println!("Height {}, Width {}", dims.height, dims.width);

  let mut tm = TerrainMap::<MapEntity>::new(dims);

  let mut p = Point::default();
  let mut start: Option<Point> = None;
  let mut end: Option<Point> = None;

  for line in puzzle_input.lines() {
    for c in line.chars() {
      let e = match c {
        '#' => MapEntity::Wall,
        '.' => MapEntity::Empty,
        'S' => {
          start = Some(p);
          MapEntity::Empty
        }
        'E' => {
          end = Some(p);
          MapEntity::Empty
        }

        _ => panic!("unexpected char"),
      };
      tm.set(&p, e);
      p.x += 1;
    }
    p.y += 1;
    p.x = 0;
  }

  let start = start.expect("robot start loc not found");
  let end = end.expect("robot end loc not found");
  let start_dir = Direction::East;

  let mut nodes = Vec::<Point>::new();
  let mut nodes2 = HashSet::<PointDir>::default(); // every direction you enter a point from, or leave it in the direction of
  nodes.push(start);
  nodes.push(end);
  nodes2.insert((start, start_dir));

  // process maze into graph
  let mut p = Point { x: 0, y: 0 };
  for y in 0..dims.height {
    p.y = y as isize;
    for x in 0..dims.width {
      p.x = x as isize;
      if tm.get(&p) == MapEntity::Wall {
        continue;
      }
      let neigh = neighbour_dirs(p, &tm);
      if neigh.len() > 2 && p != start && p != end {
        nodes.push(p);
      }

      /*
      if neigh.len() > 2 {
        if p == end {
          // can only enter the end but not leave it
          for d in neigh {
            nodes2.insert((p, d.reverse()));
          }
        } else {
          for d in neigh {
            nodes2.insert((p, d));
            nodes2.insert((p, d.reverse()));
          }
        }
      }*/
      if neigh.len() > 2 || p == start || p == end {
        for d in neigh {
          nodes2.insert((p, d));
          nodes2.insert((p, d.reverse()));
        }
      }
    }
  }

  println!("{} nodes in the graph. Start: {start}. End: {end}.", nodes.len());

  let mut procd = HashMap::<Point, Vec<(Direction, Point, Direction, usize)>>::default();
  let mut procd2 = HashMap::<(Point, Direction), (Point, Direction, usize)>::default();

  /*
  for d in all::<Direction>() {
    if d == start_dir {
      continue;
    }

    let n = start.neighbour(d);
    if !tm.dims.contains(&n) {
      continue;
    }
    if tm.get(&n) == MapEntity::Wall {
      continue;
    }

    let cost = if d == start_dir.reverse() {
      2 * TURN_COST
    } else {
      TURN_COST
    };

    let vec = procd.entry(start).or_insert(Vec::new());
    let fw = (start_dir, start, d, cost);
    vec.push(fw);

    procd2.insert((start, start_dir), (start, d, cost));
  }*/

  for node in nodes.iter() {
    for d in all::<Direction>() {
      let n = node.neighbour(d);
      if !tm.dims.contains(&n) {
        continue;
      }
      if tm.get(&n) == MapEntity::Wall {
        continue;
      }
      let mut cost = 1;
      let mut cur_dir = d;
      let mut cur = n;
      loop {
        let exit_dirs = neighbour_dirs(cur, &tm);
        let qty = exit_dirs.len();
        if n == end || n == start || qty > 2 {
          // live end
          println!("new segment {node} {d:?} to {cur} {cur_dir:?} cost {cost}");
          let vec = procd.entry(*node).or_insert(Vec::new());
          let fw = (d, cur, cur_dir, cost);
          if !vec.contains(&fw) {
            vec.push(fw);
          } else {
            // println!("avoided adding dup fw");
          }
          procd2.insert((*node, d), (cur, cur_dir, cost));
          let bw = (cur_dir.reverse(), *node, d.reverse(), cost);
          let vec = procd.entry(cur).or_insert(Vec::new());
          if !vec.contains(&bw) {
            vec.push(bw);
          } else {
            // println!("avoided adding dup bw");
          }
          procd2.insert((cur, cur_dir.reverse()), (*node, d.reverse(), cost));
          break;
        } else if qty < 2 {
          println!("dead end at {cur}");
          // dead end
          break;
        }

        // there is one exit, move along it
        let mut exit_found = false;
        for e in exit_dirs {
          if e != cur_dir.reverse() {
            if e != cur_dir {
              cost += TURN_COST;
              cur_dir = e;
            }
            exit_found = true;
            break;
          }
        }
        assert!(exit_found);

        cur = cur.neighbour(cur_dir);
        cost += 1;
      }
    }
  }

  // assert!(procd2.contains_key(&(start, start_dir)));

  println!("{:?}", procd.get(&start));
  println!("{:?}", procd.get(&end));

  /*
  for d in all::<Direction>() {
    let n = end.neighbour(d);
    if tm.get(&n) == MapEntity::Wall {
      continue;
    }
    let r = d.reverse();
    println!("checking end {r:?}");
    assert!(procd2.contains_key(&(end, r)));
  }*/

  // now DFS search along this graph
  // ran for 8 hours. I did put duplicate nodes into
  // the graph which conceivably created loops.
  // but still ran for more than a few seconds when I removed them

  // Dijkstra
  let mut dist = HashMap::<PointDir, usize>::default();
  let mut prev = HashMap::<PointDir, Option<PointDir>>::default();
  let mut pq = PriorityQueue::<PointDir, Reverse<usize>>::default();

  for k in nodes2.iter() {
    dist.insert(*k, usize::MAX);
    prev.insert(*k, None);
    pq.push(*k, Reverse(usize::MAX));
  }

  let sp = (start, Direction::East);
  dist.insert(sp, 0);
  let old = pq.change_priority(&sp, Reverse(0));
  assert!(old.is_some());

  let mut shortest_cost: Option<usize> = None;
  while let Some(((pos, dir), priority)) = pq.pop() {
    println!("Considering {pos} {dir:?} {}", priority.0);
    if priority == Reverse(usize::MAX) {
      println!("popped something unreachable {pos} {dir:?} off the queue");
      break;
    }

    if pos == end && shortest_cost.is_none() {
      println!("shortest path: {}", priority.0);
      shortest_cost = Some(priority.0);
    }

    let vc = procd.get(&pos).unwrap();

    for (entry_dir, exit, exit_dir, exit_cost) in vc.iter() {
      if dir.reverse() == *entry_dir {
        continue;
      }

      let ed = (*exit, *exit_dir);

      assert_eq!(priority.0, *dist.get(&(pos, dir)).unwrap());
      let cost = priority.0 + exit_cost + if *entry_dir == dir { 0 } else { TURN_COST };
      if !dist.contains_key(&ed) {
        println!("{ed:?} not in dist");
        for d in all::<Direction>() {
          if d == ed.1 {
            continue;
          }
          if !dist.contains_key(&(*exit, d)) {
            println!("also {exit}, {d:?}");
          }
        }
      }

      if cost < *dist.get(&ed).unwrap() {
        dist.insert(ed, cost);
        prev.insert(ed, Some((pos, dir)));
        pq.change_priority(&ed, Reverse(cost));
      }
    }
  }

  let shortest_cost = shortest_cost.unwrap();

  let mut ok_nodes = HashSet::<PointDir>::default();
  let mut ok_points = HashSet::<Point>::default();
  for (k, v) in dist.iter() {
    if *v > shortest_cost {
      if *v < usize::MAX {
        println!("not on shortest path: {k:?} {v}");
      }
    } else {
      ok_nodes.insert(*k);
      ok_points.insert(k.0);
    }
  }

  println!(
    "{}/{} nodes {}/{} points might be on a path: {ok_points:?}",
    ok_nodes.len(),
    nodes2.len(),
    ok_points.len(),
    nodes.len()
  );

  let mut visited = Vec::<Point>::default();
  visited.push(start);
  let start_cost = 0;
  let (dfs_cost, possible) = find_path_routes(start, start_dir, end, start_cost, &procd, &mut visited, shortest_cost);
  assert!(possible);
  println!("dijkstra lowest cost {shortest_cost}, dfs {dfs_cost}");
  assert_eq!(shortest_cost, dfs_cost);

  panic!("Didn't reach goal");
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input16.txt");
  let answer1 = analyse_input1(&data);
  println!("answer: {answer1}");
  let answer2 = analyse_input2(&data);
  println!("answer2: {answer2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 7036);
  }

  #[test]
  fn test_load1b() {
    let data = load_data("testinput2.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 11048);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 45);
  }

  #[test]
  fn test_load2b() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 64);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};
  #[bench]
  fn bench_part1a(b: &mut Bencher) {
    let data = load_data("testinput1.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part1b(b: &mut Bencher) {
    let data = load_data("testinput2.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  /*
    #[bench]
    fn bench_part1(b: &mut Bencher) {
      let data = load_data("input16.txt");
      b.iter(|| black_box(analyse_input1(&data)));
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
      let data = load_data("input16.txt");
      b.iter(|| black_box(analyse_input2(&data)));
    }
  */
}
