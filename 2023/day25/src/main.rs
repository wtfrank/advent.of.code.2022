use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

//use advent::{determine_map_dims, Direction, Point, TerrainMap};
//use advent::{Interval, Point3};

//use enum_iterator::all;
//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::Reverse;
//use std::cmp::{max,Reverse,Ordering};
//use std::cmp::Ordering;
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

fn remove_conn(a: &str, b: &str, connections: &mut [bool], nodes: &HashMap<String, usize>) {
  let a = nodes.get(a).unwrap();
  let b = nodes.get(b).unwrap();

  let width = nodes.len() + 1;

  assert!(connections[a * width + b]);
  assert!(connections[b * width + a]);

  connections[a * width + b] = false;
  connections[b * width + a] = false;
}

fn find_connected(
  node: &str,
  connections: &[bool],
  nodes: &HashMap<String, usize>,
  indexes: &HashMap<usize, String>,
) -> usize {
  let mut visited = HashSet::new();
  let mut queue = Vec::<usize>::new();
  let n = nodes.get(node).unwrap();

  let width = nodes.len() + 1;
  println!("width: {width}");

  queue.push(*n);

  let mut count = 0;
  while let Some(n) = queue.pop() {
    if visited.contains(&n) {
      continue;
    }
    println!("visited {}", indexes.get(&n).unwrap());
    visited.insert(n);
    count += 1;

    for i in 0..width {
      if connections[i * width + n] && !visited.contains(&i) {
        queue.push(i);
      }
    }
  }

  count
}

fn analyse_data(connections: &mut [bool], nodes: &HashMap<String, usize>, indexes: &HashMap<usize, String>) -> usize {
  remove_conn("hqq", "xxq", connections, nodes);
  remove_conn("xzz", "kgl", connections, nodes);
  remove_conn("vkd", "qfb", connections, nodes);

  let count1 = find_connected("hqq", connections, nodes, indexes);
  let count2 = find_connected("xxq", connections, nodes, indexes);

  println!("group1: {count1}, groups2: {count2}");
  count1 * count2
}

fn load_data(filename: &str) -> (Vec<bool>, HashMap<String, usize>, HashMap<usize, String>) {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut vertices = Vec::new();

  let mut known_edges = HashMap::<String, usize>::default();
  let mut known_edges2 = HashMap::<usize, String>::default();
  let mut edge_index = Vec::new();

  for line in contents.lines() {
    let r = sscanf::sscanf!(line, "{String}: {String}").unwrap();

    let s = r.1.split(' ');

    let v = s.map(|q| q.to_string()).collect::<Vec<String>>();

    vertices.push((r.0.to_string(), v));
  }

  for (v, u) in &vertices {
    if !known_edges.contains_key(v) {
      edge_index.push(v.clone());
      known_edges.insert(v.clone(), edge_index.len() - 1);
      known_edges2.insert(edge_index.len() - 1, v.clone());
    }
    for us in u {
      if !known_edges.contains_key(us) {
        edge_index.push(us.clone());
        known_edges.insert(us.clone(), edge_index.len() - 1);
        known_edges2.insert(edge_index.len() - 1, us.clone());
      }
    }
  }

  let width = edge_index.len() + 1;
  let mut connections = Vec::new();
  println!("total vertices: {}", edge_index.len() + 1);
  connections.resize(width * width, false);

  for (v, us) in vertices {
    let v_id = known_edges.get(&v).unwrap();
    for u in us {
      let u_id = known_edges.get(&u).unwrap();

      let idx = v_id * width + u_id;
      connections[idx] = true;
      let idx = u_id * width + v_id;
      connections[idx] = true;
    }
  }

  (connections, known_edges, known_edges2)
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

  let (mut data, data2, data3) = load_data("input25.txt");
  let score1 = analyse_data(&mut data, &data2, &data3);
  println!("score1: {score1}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let (mut data, data2, data3) = load_data("testinput.txt");
    let score1 = analyse_data(&mut data, &data2, &data3);
    assert_eq!(score1, 2);
  }
}
