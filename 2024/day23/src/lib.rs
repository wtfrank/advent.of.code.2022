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

#[allow(unused_imports)]
use priority_queue::PriorityQueue;
#[allow(unused_imports)]
use std::cmp::Reverse;

#[allow(unused_imports)]
use enum_iterator::{all, Sequence};

#[allow(unused_imports)]
use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;

#[allow(unused_imports)]
use std::sync::OnceLock;

pub fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

type Node = [char; 2];

fn adj_list(puzzle_input: &str) -> HashMap<Node, Vec<Node>> {
  let mut adjacency = HashMap::<Node, Vec<Node>>::default();
  for line in puzzle_input.lines() {
    let mut it = line.split("-");
    let first: Node = it.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap();
    let second: Node = it.next().unwrap().chars().collect::<Vec<char>>().try_into().unwrap();
    assert_eq!(None, it.next());

    adjacency.entry(first).or_insert(Vec::new()).push(second);
    adjacency.entry(second).or_insert(Vec::new()).push(first);
  }
  adjacency
}

pub fn analyse_input1(puzzle_input: &str) -> usize {
  let adjacency = adj_list(puzzle_input);
  let mut groups = HashSet::<[Node; 3]>::default();

  for (comp, seconds) in adjacency.iter().filter(|(k, _)| k[0] == 't') {
    for second in seconds {
      for third in adjacency.get(second).unwrap() {
        if third == comp {
          continue;
        }
        for check in adjacency.get(third).unwrap() {
          if check == comp {
            let mut group = [*comp, *second, *third];
            group.sort();
            groups.insert(group);
          }
        }
      }
    }
  }
  groups.len()
}

pub fn analyse_input2(puzzle_input: &str) -> usize {
  let adjacency = adj_list(puzzle_input);
  println!(
    "#nodes: {}. starting with t:{}",
    adjacency.len(),
    adjacency.keys().filter(|k| k[0] == 't').count()
  );
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 7);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 23);
  }
}
