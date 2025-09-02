#[allow(unused_imports)]
use regex::Regex;
#[allow(unused_imports)]
use std::cmp::PartialEq;

use std::fs::File;
use std::io::Read;

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

pub fn load_data(filename: &str) -> String {
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

fn load_map(puzzle_input: &str) -> (TerrainMap<MapEntity>, Point, Point) {
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

  (tm, start, end)
}

pub fn analyse_input1(puzzle_input: &str) -> usize {
  let (tm, start, end) = load_map(puzzle_input);
  let start_dir = Direction::East;

  let (_nodes, nodes2, procd, _exit_entry) = preprocess_map(start, start_dir, end, &tm);
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

  let start_pd = (start, Direction::East);
  let (_d_prev, d_dist, _d_end, d_cost) = dijkstra(
    nodes2.iter(),
    start_pd,
    |p| p.0 == end,
    |(pos, dir)| {
      let vc = procd.get(&pos).unwrap();

      vc.iter()
        .filter(|eeee| dir.reverse() != eeee.0)
        .map(|eeee| ((eeee.1, eeee.2), eeee.3 + if eeee.0 == dir { 0 } else { TURN_COST }))
        .collect::<Vec<(PointDir, usize)>>()
    },
  );

  if d_cost.is_none() {
    panic!("dijkstra issue");
  }
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

  let mut shortest_cost = None;
  while let Some(((pos, dir), priority)) = pq.pop() {
    println!("Considering {pos} {dir:?} {}", priority.0);
    if priority == Reverse(usize::MAX) {
      if shortest_cost.is_none() {
        panic!("Somehow popped something unreachable {pos} {dir:?} off the queue before reaching dest");
      }
      break;
    }

    if pos == end && shortest_cost.is_none() {
      //return priority.0;
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

  for d in dist.keys() {
    assert_eq!(d_dist.get(d), dist.get(d));
  }

  shortest_cost.unwrap()
}

#[allow(clippy::type_complexity)]
fn preprocess_map(
  start: Point,
  start_dir: Direction,
  end: Point,
  tm: &TerrainMap<MapEntity>,
) -> (
  Vec<Point>,
  HashSet<PointDir>,                                         // direction you can enter a point
  HashMap<Point, Vec<(Direction, Point, Direction, usize)>>, //in, (in_dir, out, out_dir). in_dir is the direction you were travelling as you left in. out_dir is the direction you were travelling as you entered out.
  HashMap<(Point, Direction), (Point, Direction)>, // if you leave this pos travelling in this dir, you end up entering that pos travelling in that dir
) {
  let mut nodes = Vec::<Point>::new();
  let mut nodes2 = HashSet::<PointDir>::default(); // every direction you could enter a point from
  let mut exit_entry = HashMap::<PointDir, PointDir>::default();
  nodes.push(start);
  nodes.push(end);
  nodes2.insert((start, start_dir));

  let neigh = neighbour_dirs(start, tm);
  for d in neigh {
    nodes2.insert((start, d.reverse()));
  }

  let neigh = neighbour_dirs(end, tm);
  for d in neigh {
    nodes2.insert((end, d.reverse()));
  }

  // process maze into graph
  let mut p = Point { x: 0, y: 0 };
  for y in 0..tm.dims.height {
    p.y = y as isize;
    for x in 0..tm.dims.width {
      p.x = x as isize;
      if tm.get(&p) == MapEntity::Wall {
        continue;
      }
      let neigh = neighbour_dirs(p, tm);
      if neigh.len() > 2 && p != start && p != end {
        nodes.push(p);
      }
      /*
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
      */
      if neigh.len() > 2 || p == start || p == end {
        for d in neigh {
          // nodes2.insert((p, d));
          nodes2.insert((p, d.reverse()));
        }
      }
    }
  }

  println!("{} nodes in the graph. Start: {start}. End: {end}.", nodes.len());

  let mut procd = HashMap::<Point, Vec<(Direction, Point, Direction, usize)>>::default();

  for node in nodes.iter() {
    let log = false; // *node == end || *node == start || *node == Point { x: 1, y: 5 } || *node == Point { x: 15, y: 7 };
    if log {
      println!("Checking node {node:?} exits");
    }
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
        let exit_dirs = neighbour_dirs(cur, tm);
        let qty = exit_dirs.len();
        if cur == end || cur == start || qty > 2 {
          // live end
          // println!("new segment {node} {d:?} to {cur} {cur_dir:?} cost {cost}");
          let vec = procd.entry(*node).or_insert(Vec::new());
          let fw = (d, cur, cur_dir, cost);
          if !vec.contains(&fw) {
            vec.push(fw);
          } else {
            // println!("avoided adding dup fw");
          }
          let bw = (cur_dir.reverse(), *node, d.reverse(), cost);
          let vec2 = procd.entry(cur).or_insert(Vec::new());
          if !vec2.contains(&bw) {
            vec2.push(bw);
          } else {
            // println!("avoided adding dup bw");
          }

          // if you leave cur in direction cur_dir,reverse, you get to node in direction d.reverse
          exit_entry.insert((cur, cur_dir.reverse()), (*node, d.reverse()));
          if log {
            println!("exit_entry: {cur} {:?} | {node} {:?}", cur_dir.reverse(), d.reverse());
          }
          // println!("new segment: fw {node:?} {fw:?} | bw {cur:?} {bw:?}");
          break;
        } else if qty < 2 {
          // println!("dead end at {cur}");
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

  println!("start {:?}", procd.get(&start));
  println!("end {:?}", procd.get(&end));

  (nodes, nodes2, procd, exit_entry)
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

/*
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

*/

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

fn draw_map_nodes(
  start: Point,
  end: Point,
  tm: &TerrainMap<MapEntity>,
  nodes: &[Point],
  ok_nodes: &HashSet<Point>,
  path_nodes: &HashSet<Point>,
) {
  let mut bad_nodes = HashSet::<Point>::default();
  for n in nodes {
    if !ok_nodes.contains(n) {
      bad_nodes.insert(*n);
    }
  }

  println!(
    " {}",
    (0..tm.dims.height)
      .map(|a| a % 10)
      .map(|b| char::from_digit(b as u32, 10).unwrap())
      .collect::<String>()
  );

  for y in 0..tm.dims.height {
    let mut chars = String::new();
    for x in 0..tm.dims.width {
      let p = Point {
        x: x as isize,
        y: y as isize,
      };
      if p == start {
        chars.push('S');
      } else if p == end {
        chars.push('E');
      } else if path_nodes.contains(&p) {
        chars.push_str("\x1b[92m.\x1b[0m");
      } else if bad_nodes.contains(&p) {
        chars.push_str("\x1b[91mX\x1b[0m");
      } else if ok_nodes.contains(&p) {
        chars.push('O');
      } else {
        chars.push(match tm.getc(x as isize, y as isize) {
          MapEntity::Empty => ' ',
          MapEntity::Wall => '#',
        });
      }
    }
    println!("{}{chars}", y % 10);
  }
}

// fn expand_neigbours<'a>(node: PointDir) -> impl Iterator<Item = &'a PointDir> {}

pub fn analyse_input2(puzzle_input: &str) -> usize {
  let (tm, start, end) = load_map(puzzle_input);
  let start_dir = Direction::East;

  let (nodes, nodes2, procd, exit_entry) = preprocess_map(start, start_dir, end, &tm);

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

  // Dijkstra - forwards

  let mut prev_reachable = usize::MAX;
  let mut reachable = nodes.len();

  let mut possible_nodes: HashSet<PointDir> = nodes2.iter().copied().collect();
  let possible_points: HashSet<Point> = possible_nodes.iter().map(|(p, _d)| p).copied().collect();

  draw_map_nodes(start, end, &tm, &nodes, &possible_points, &HashSet::default());

  while reachable < prev_reachable {
    println!("--- forwards dijkstra");
    //TODO:
    println!("procd 1,5: {:?}", procd.get(&Point { x: 1, y: 5 }));
    for d in all::<Direction>() {
      println!(
        "1,5 {d:?} in nodes? {}",
        possible_nodes.contains(&(Point { x: 1, y: 5 }, d))
      );
    }
    println!("procd 3,5: {:?}", procd.get(&Point { x: 3, y: 5 }));
    for d in all::<Direction>() {
      println!(
        "3,5 {d:?} in nodes? {}",
        possible_nodes.contains(&(Point { x: 3, y: 5 }, d))
      );
    }
    println!("procd 3,7: {:?}", procd.get(&Point { x: 3, y: 7 }));
    for d in all::<Direction>() {
      println!(
        "3,7 {d:?} in nodes? {}",
        possible_nodes.contains(&(Point { x: 3, y: 7 }, d))
      );
    }
    let start_pd = (start, Direction::East);
    let (prev, dist, d_end, d_cost) = dijkstra(
      possible_nodes.iter(),
      start_pd,
      |p| p.0 == end,
      |(pos, dir)| {
        let vc = procd.get(&pos).unwrap();

        let neighbours = vc
          .iter()
          //.filter(|eeee| dir.reverse() != eeee.0)
          .filter(|eeee| {
            let next_pd = exit_entry.get(&(eeee.1, eeee.2));
            next_pd.is_some() && possible_nodes.contains(next_pd.unwrap())
          })
          .map(|eeee| ((eeee.1, eeee.2), eeee.3 + if eeee.0 == dir { 0 } else { TURN_COST }))
          .collect::<Vec<(PointDir, usize)>>();
        println!("neighbours of {pos},{dir:?}: {:?}", neighbours);
        neighbours
      },
    );

    let shortest_cost = d_cost.unwrap();
    let end_pointdir = d_end.unwrap();

    let (path, pathpd) = derive_route(start, end_pointdir, &prev);
    let path_nodes = HashSet::<Point>::from_iter(path.iter().copied());

    let mut ok_nodes = HashSet::<PointDir>::default();
    let mut ok_points = HashSet::<Point>::default();

    // hack because the initial node might be rotated wrongly
    ok_nodes.insert(pathpd[0]);
    ok_nodes.insert((start, start_dir));
    for (k, v) in dist.iter() {
      let min_end_dist =
        end.x.abs_diff(k.0.x) + end.y.abs_diff(k.0.y) + if k.0.x != end.x && k.0.y != end.y { TURN_COST } else { 0 };
      if *v > shortest_cost - min_end_dist {
        // v might be usize::MAX
        if *v < usize::MAX {
          // println!("not on shortest path: {k:?} {v} {min_end_dist}");
        }
      } else {
        // ok_nodes.insert(*k);
        ok_points.insert(k.0);
        // add all directions at every valid point
        for (d, _, _, _) in procd.get(&k.0).unwrap().iter() {
          ok_nodes.insert((k.0, *d));
        }
      }
    }

    draw_map_nodes(start, end, &tm, &nodes, &ok_points, &path_nodes);

    println!(
      "{}/{} nodes {}/{} points might be on a path",
      ok_nodes.len(),
      nodes2.len(),
      ok_points.len(),
      nodes.len()
    );

    assert!(ok_points.contains(&start));
    assert!(ok_nodes.contains(&(start, start_dir)));

    for d in all::<Direction>() {
      println!("3,5 {d:?} in nodes? {}", ok_nodes.contains(&(Point { x: 3, y: 5 }, d)));
    }

    println!("--- backwards dijkstra");
    /*
    let mut n = expanded_ok_nodes.iter().copied().collect::<Vec<PointDir>>();
    n.sort_by(|a, b| {
      let x = a.0.x.cmp(&b.0.x);
      if x == std::cmp::Ordering::Equal {
        a.0.y.cmp(&b.0.y)
      } else {
        x
      }
    });
    println!("Nodes: {:?}", n);*/
    //Dijkstra - backwards
    let start2 = end;
    let end2 = start;
    let start_pd = (start2, Direction::South);
    let (prev2, dist2, d_end2, d_cost2) = dijkstra(
      ok_nodes.iter(),
      start_pd,
      |p| p.0 == end2,
      |(pos, dir)| {
        let vc = procd.get(&pos).unwrap();
        let neighbours = vc
          .iter()
          //.filter(|eeee| dir.reverse() != eeee.0)
          // .filter(|eeee| ok_nodes.contains(&(eeee.1, eeee.2.reverse())))
          .filter(|eeee| {
            let next_pd = exit_entry.get(&(eeee.1, eeee.2));
            next_pd.is_some() && ok_nodes.contains(next_pd.unwrap())
          })
          .map(|eeee| {
            (
              (eeee.1, eeee.2.reverse()),
              eeee.3 + if eeee.0 == dir { 0 } else { TURN_COST },
            )
          })
          .collect::<Vec<(PointDir, usize)>>();
        // println!("{pos:?} neighbours: {neighbours:?}");
        neighbours
      },
    );

    let shortest_cost2 = d_cost2.expect("end has not been reached");
    let end_pointdir2 = d_end2.unwrap();

    let (path2, path2pd) = derive_route(end, end_pointdir2, &prev2);
    let path_nodes2 = HashSet::<Point>::from_iter(path2.iter().copied());

    let mut ok_nodes2 = HashSet::<PointDir>::default();
    let mut ok_points2 = HashSet::<Point>::default();
    ok_nodes2.insert(path2pd[0]);
    ok_nodes2.insert((start, start_dir));
    for (k, v) in dist2.iter() {
      let end = end2;
      let min_end_dist =
        end.x.abs_diff(k.0.x) + end.y.abs_diff(k.0.y) + if k.0.x != end.x && k.0.y != end.y { TURN_COST } else { 0 };
      if *v > shortest_cost2 - min_end_dist {
        // v might be usize::MAX
        if *v < usize::MAX {
          // println!("not on shortest path: {k:?} {v} {min_end_dist}");
        }
      } else {
        // ok_nodes2.insert(*k);
        ok_points2.insert(k.0);

        for (d, _, _, _) in procd.get(&k.0).unwrap().iter() {
          ok_nodes2.insert((k.0, *d));
        }
      }
    }

    draw_map_nodes(start2, end2, &tm, &nodes, &ok_points2, &path_nodes2);

    println!(
      "{}/{} nodes {}/{} points might be on a path",
      ok_nodes2.len(),
      nodes2.len(),
      ok_points2.len(),
      nodes.len()
    );

    assert!(ok_points2.contains(&start));
    assert!(ok_nodes2.contains(&(start, start_dir)));

    let new_reachable = ok_points2.len();
    println!("prev reachable: {prev_reachable}, cur reachable: {reachable}, new reachable: {new_reachable}");
    prev_reachable = reachable;
    reachable = new_reachable;

    println!("ok_nodes2: {:?}", ok_nodes2);

    for (k, v) in procd.iter() {
      for (d, _, _, _) in v.iter() {
        if !ok_nodes2.contains(&(*k, *d)) {
          println!("{k:?},{d:?} disallowed");
        }
      }
    }

    possible_nodes = ok_nodes2;
    /*
    possible_nodes.clear();
    for n in ok_nodes2.drain() {
      possible_nodes.insert(n);
    }*/
    // possible_points = ok_points2;
  }

  // go through each node not on path and see if start -> N -> end is the same cost as start -> end

  /*
    let mut visited = Vec::<Point>::default();
    visited.push(start);
    let start_cost = 0;
    let (dfs_cost, possible) = find_path_routes(start, start_dir, end, start_cost, &procd, &mut visited, shortest_cost);
    assert!(possible);
    println!("dijkstra lowest cost {shortest_cost}, dfs {dfs_cost}");
    assert_eq!(shortest_cost, dfs_cost);
  */
  panic!("Didn't reach goal");
}

/* this variant of dijkstra annotates the distance to all nodes, goal or otherwise */
#[allow(clippy::type_complexity)]
fn dijkstra<'a, 'b, T, F, G>(
  nodes: impl Iterator<Item = &'a T>,
  start: T,
  goal_fn: F,
  neighbours_fn: G,
) -> (HashMap<T, Option<T>>, HashMap<T, usize>, Option<T>, Option<usize>)
where
  T: 'a + Copy + Eq + std::hash::Hash + std::fmt::Debug,
  F: Fn(T) -> bool,
  G: Fn(T) -> Vec<(T, usize)>,
{
  let mut dist = HashMap::<T, usize>::default();
  let mut prev = HashMap::<T, Option<T>>::default();
  let mut pq = PriorityQueue::<T, Reverse<usize>>::default();

  let mut node_count = 0;
  for k in nodes {
    dist.insert(*k, usize::MAX);
    prev.insert(*k, None);
    pq.push(*k, Reverse(usize::MAX));
    node_count += 1;
  }
  println!("dijkstra: {node_count} nodes");

  dist.insert(start, 0);
  let old = pq.change_priority(&start, Reverse(0));
  if old.is_none() {
    panic!("no node in queue for {start:?}");
  }

  let mut shortest_cost: Option<usize> = None;
  let mut end_node: Option<T> = None;
  while let Some((node, priority)) = pq.pop() {
    if priority == Reverse(usize::MAX) {
      break;
    }
    println!("expand {node:?} {} {:?}", priority.0, prev.get(&node).unwrap());

    if shortest_cost.is_none() && goal_fn(node) {
      println!("shortest path: {}", priority.0);
      shortest_cost = Some(priority.0);
      end_node = Some(node);
    }

    for (neighbour, cost) in neighbours_fn(node) {
      let total_cost = priority.0 + cost;
      if total_cost
        < *dist
          .get(&neighbour)
          .unwrap_or_else(|| panic!("no dist entry for {neighbour:?}"))
      {
        dist.insert(neighbour, total_cost);
        prev.insert(neighbour, Some(node));
        pq.change_priority(&neighbour, Reverse(total_cost));
      }
    }
  }

  (prev, dist, end_node, shortest_cost)
}

fn derive_route(
  start: Point,
  end: PointDir,
  prev: &HashMap<PointDir, Option<PointDir>>,
) -> (Vec<Point>, Vec<PointDir>) {
  let mut route = Vec::<Point>::new();
  let mut routepd = Vec::<PointDir>::new();

  let mut penult = end;
  let mut cur = end;
  while cur.0 != start {
    route.push(cur.0);
    routepd.push(cur);
    let p = (*prev.get(&cur).unwrap()).unwrap();
    penult = cur;
    cur = p;
  }

  println!("derive_route: final was {cur:?}, prev was {penult:?}");

  route.push(cur.0);
  routepd.push((cur.0, penult.1)); // hack because start node is not necessarily an exit node
  route.reverse();
  routepd.reverse();
  (route, routepd)
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

  #[test]
  fn test_preproc1() {
    let data = load_data("testinput2.txt");
    let (tm, start, end) = load_map(&data);
    let start_dir = Direction::East;
    let (nodes, nodes2, procd, exit_entry) = preprocess_map(start, start_dir, end, &tm);

    let point1_5 = Point { x: 1, y: 5 };
    let point3_5 = Point { x: 3, y: 5 };
    let point3_7 = Point { x: 3, y: 7 };
    let point5_15 = Point { x: 5, y: 15 };

    assert!(nodes.contains(&start));
    assert!(nodes.contains(&end));
    assert!(nodes.contains(&point1_5));
    assert!(nodes.contains(&point3_5));
    assert!(procd.contains_key(&point1_5));
    assert!(procd.contains_key(&point3_5));
    assert!(nodes2.contains(&(point1_5, Direction::North)));
    assert!(nodes2.contains(&(point1_5, Direction::South)));
    assert!(nodes2.contains(&(point1_5, Direction::West)));
    assert!(nodes2.contains(&(point3_5, Direction::North)));
    assert!(nodes2.contains(&(point3_5, Direction::South)));
    assert!(nodes2.contains(&(point3_5, Direction::East)));

    assert!(nodes2.contains(&(start, Direction::South)));
    assert!(nodes2.contains(&(end, Direction::East)));
    assert!(nodes2.contains(&(end, Direction::North)));

    let start_neigh = procd.get(&start).unwrap();
    assert!(start_neigh.iter().any(|f| f.0 == Direction::North));
    let end_neigh = procd.get(&end).unwrap();
    assert!(end_neigh.iter().any(|f| f.0 == Direction::West));
    assert!(end_neigh.iter().any(|f| f.0 == Direction::South));

    let p15_neigh = procd.get(&point1_5).unwrap();
    assert_eq!(p15_neigh.len(), 3);
    println!("p15_neigh: {p15_neigh:?}");
    assert!(p15_neigh
      .iter()
      .any(|f| f.0 == Direction::East && f.1 == point3_5 && f.2 == Direction::East));
    assert!(p15_neigh
      .iter()
      .any(|f| f.0 == Direction::North && f.1 == point3_5 && f.2 == Direction::South));
    assert!(p15_neigh.iter().any(|f| f.0 == Direction::South));
    assert!(!p15_neigh.iter().any(|f| f.0 == Direction::West));
    let p35_neigh = procd.get(&point3_5).unwrap();
    assert_eq!(p35_neigh.len(), 3);

    assert!(p35_neigh
      .iter()
      .any(|f| f.0 == Direction::South && f.1 == point3_7 && f.2 == Direction::South));
    assert!(p35_neigh.iter().any(|f| f.0 == Direction::North));
    assert!(p35_neigh.iter().any(|f| f.0 == Direction::West));
    assert!(!p35_neigh.iter().any(|f| f.0 == Direction::East));

    assert!(exit_entry.contains_key(&(point3_5, Direction::West)));
    assert!(exit_entry.contains_key(&(point3_5, Direction::North)));
    assert!(exit_entry.contains_key(&(point3_5, Direction::South)));
    assert!(!exit_entry.contains_key(&(point3_5, Direction::East)));

    assert_eq!(
      exit_entry.get(&(point3_5, Direction::South)).unwrap(),
      &(point3_7, Direction::South)
    );
    assert_eq!(
      exit_entry.get(&(point3_7, Direction::South)).unwrap(),
      &(point5_15, Direction::East)
    );
  }
  #[test]
  fn test_preproc2() {
    let data = load_data("testinput2.txt");
    let (tm, start, end) = load_map(&data);
    let start_dir = Direction::East;
    let (nodes, nodes2, _procd, exit_entry) = preprocess_map(start, start_dir, end, &tm);

    let possible_points: HashSet<Point> = nodes2.iter().map(|(p, _d)| p).copied().collect();

    draw_map_nodes(start, end, &tm, &nodes, &possible_points, &HashSet::default());

    let start_next = Point { x: 1, y: 5 };
    let end_next = Point { x: 15, y: 7 };

    assert!(exit_entry.contains_key(&(start, Direction::North)));
    assert!(exit_entry.contains_key(&(start_next, Direction::South)));
    assert!(exit_entry.contains_key(&(start_next, Direction::North)));
    assert!(exit_entry.contains_key(&(start_next, Direction::East)));
    assert!(exit_entry.contains_key(&(end, Direction::South)));
    assert!(exit_entry.contains_key(&(end, Direction::West)));
    assert!(exit_entry.contains_key(&(end_next, Direction::North)));
    assert!(exit_entry.contains_key(&(end_next, Direction::West)));
    assert!(exit_entry.contains_key(&(end_next, Direction::South)));

    for (k, v) in exit_entry.iter() {
      println!("checking {k:?} {v:?} has {:?}{:?}", v.0, v.1.reverse());
      assert!(exit_entry.contains_key(&(v.0, v.1.reverse())));
    }
  }
}
