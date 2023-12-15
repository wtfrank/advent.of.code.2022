use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::iter::zip;

//use std::collections::HashSet;
//use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

use std::collections::HashMap;

use advent::{lcm, prime_factors};

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
    let (directions, nodes) = load_data("testinput.txt");
    let score = analyse_data(&directions, &nodes);

    assert_eq!(score, 2);
  }

  #[test]
  fn test_load2() {
    let (directions, nodes) = load_data("testinput2.txt");
    let score = analyse_data(&directions, &nodes);

    assert_eq!(score, 6);
  }

  #[test]
  fn test_load3() {
    let (directions, nodes) = load_data("testinput3.txt");
    let score = analyse_data2(&directions, &nodes);

    assert_eq!(score, 6);
  }
}

fn analyse_data(dirs: &str, nodes: &HashMap<String, (String, String)>) -> usize {
  let mut score = 0;
  let mut i = 0;
  let mut cur_node = "AAA";
  let dv: Vec<char> = dirs.chars().collect();
  loop {
    if cur_node == "ZZZ" {
      break;
    }

    let dir = dv[i];
    let next: &str = match dir {
      'L' => &nodes.get(cur_node).unwrap().0,
      'R' => &nodes.get(cur_node).unwrap().1,
      _ => {
        panic!("unexpected direction");
      }
    };

    cur_node = next;
    i = (i + 1) % dv.len();
    score += 1;
  }
  score
}

fn end_char(s: &str) -> char {
  let lcv: Vec<char> = s.chars().collect();
  let lc = lcv.last().unwrap();
  *lc
}

/* brute force solution that's inadequate */
#[allow(dead_code)]
fn analyse_data2(dirs: &str, nodes: &HashMap<String, (String, String)>) -> usize {
  let mut score = 0;
  let mut i = 0;

  let mut node_num = HashMap::<String, usize>::default();
  for (idx, n) in nodes.keys().enumerate() {
    node_num.insert(n.clone(), idx);
  }

  let mut nv = Vec::<(usize, usize)>::new();
  nv.resize(nodes.len(), (0, 0));
  let mut ends_z: Vec<bool> = Vec::<bool>::new();
  ends_z.resize(nodes.len(), false);

  for (k, v) in nodes.iter() {
    let nn = node_num.get(k).unwrap();
    let l = &v.0;
    let r = &v.1;
    let ln = node_num.get(l).unwrap();
    let rn = node_num.get(r).unwrap();

    ends_z[*nn] = end_char(k) == 'Z';

    nv[*nn] = (*ln, *rn);
  }
  for (a, b) in &nv {
    if *a == 0 && *b == 0 {
      panic!("uninitialised pos");
    }
  }

  let mut cur_node: Vec<usize> = Vec::new();

  for n in nodes.keys() {
    let lc = end_char(n);
    if lc == 'A' {
      cur_node.push(*node_num.get(n).unwrap());
    }
  }
  println!("number of ghosts: {}", cur_node.len());

  let dv: Vec<char> = dirs.chars().collect();
  loop {
    let dir = dv[i];

    let mut all_z = true;
    let mut z_count = 0;
    for c in &cur_node {
      if !ends_z[*c] {
        all_z = false;
      }
      if ends_z[*c] {
        z_count += 1;
      }
    }
    if z_count > 0 {
      println!("{score} {z_count}");
    }

    if all_z {
      break;
    }

    for cn in &mut cur_node {
      let next = match dir {
        'L' => nv.get(*cn).unwrap().0,
        'R' => nv.get(*cn).unwrap().1,
        _ => {
          panic!("unexpected direction");
        }
      };

      *cn = next;
    }
    i = (i + 1) % dv.len();
    score += 1;
    if score % 1_000_000 == 0 {
      println!("progress: {score}");
    }
  }
  score
}

fn find_cycle(start: usize, dv: &[char], nv: &[(usize, usize)], ends_z: &[bool]) -> usize {
  let mut score = 0;
  let mut i = 0;
  let mut cur_node = start;
  loop {
    println!("{start} iter {i} {cur_node}");

    if ends_z[cur_node] {
      println!("ghost {start} on finish {cur_node} at {score}");
      if i == 0 {
        println!("ghost {start} cycle length: {score}");
        break;
      }
    }

    let dir = dv[i];
    let next = match dir {
      'L' => nv.get(cur_node).unwrap().0,
      'R' => nv.get(cur_node).unwrap().1,
      _ => {
        panic!("unexpected direction");
      }
    };

    cur_node = next;
    i = (i + 1) % dv.len();
    score += 1;

    //don't check at start of function so we miss the start conditions matching!
    if cur_node == start && i == 0 {
      println!("ghost_start: {start} cycle length: {score}");
      break;
    }
  }

  score
}

fn analyse_data3(dirs: &str, nodes: &HashMap<String, (String, String)>) -> usize {
  let dv: Vec<char> = dirs.chars().collect();
  let mut node_num = HashMap::<String, usize>::default();
  let mut num_node = HashMap::<usize, String>::default();
  for (idx, n) in nodes.keys().enumerate() {
    node_num.insert(n.clone(), idx);
    num_node.insert(idx, n.clone());
  }

  let mut nv = Vec::<(usize, usize)>::new();
  nv.resize(nodes.len(), (0, 0));
  let mut ends_z: Vec<bool> = Vec::<bool>::new();
  ends_z.resize(nodes.len(), false);

  for (k, v) in nodes.iter() {
    let nn = node_num.get(k).unwrap();
    let l = &v.0;
    let r = &v.1;
    let ln = node_num.get(l).unwrap();
    let rn = node_num.get(r).unwrap();

    ends_z[*nn] = end_char(k) == 'Z';

    nv[*nn] = (*ln, *rn);
  }
  for (a, b) in &nv {
    if *a == 0 && *b == 0 {
      panic!("uninitialised pos");
    }
  }

  let mut cur_node: Vec<usize> = Vec::new();

  for n in nodes.keys() {
    let lc = end_char(n);
    if lc == 'A' {
      cur_node.push(*node_num.get(n).unwrap());
    }
  }
  println!("number of ghosts: {}", cur_node.len());

  let cycles: Vec<usize> = cur_node.iter().map(|g| find_cycle(*g, &dv, &nv, &ends_z)).collect();
  //for g_start in &cur_node {
  //  find_cycle(*g_start, &dv, &nv, &ends_z );
  //}

  println!("path length: {}", dv.len());
  for cycle in &cycles {
    println!("cycle {cycle} prime factors: {:?}", prime_factors(*cycle));
  }

  lcm(&cycles)
}

fn load_data(filename: &str) -> (String, HashMap<String, (String, String)>) {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut it = contents.lines();
  let directions = it.next().unwrap();
  let blank = it.next().unwrap();
  assert_eq!(blank.len(), 0);

  let mut nodes = HashMap::<String, (String, String)>::default();

  for line in it {
    let r = sscanf::sscanf!(line, "{String} = ({String}, {String})").unwrap();
    nodes.insert(r.0, (r.1, r.2));
  }
  (directions.to_string(), nodes)
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let (dirs, nodes) = load_data("input8.txt");
  let score1 = analyse_data(&dirs, &nodes);
  println!("score1: {score1}");
  let score2 = analyse_data3(&dirs, &nodes);
  println!("score2: {score2}");
}
