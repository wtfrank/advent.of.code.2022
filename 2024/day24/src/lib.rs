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

#[allow(unused_imports)]
use regex::Regex;

pub fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

#[allow(clippy::type_complexity)]
fn parse_input(puzzle_input: &str) -> (HashSet<&str>, HashMap<&str, usize>, Vec<(&str, &str, &str, &str)>) {
  let mut z_wires = HashSet::<&str>::default();
  let mut inputs = HashMap::<&str, usize>::default();
  let mut gates = Vec::<(&str, &str, &str, &str)>::new();

  let ra = Regex::new(r"^(.+) ([A-Z]+) (.+) -> (.+)$").unwrap();
  let mut parsing_inputs = true;
  for line in puzzle_input.lines() {
    if parsing_inputs {
      if line.is_empty() {
        parsing_inputs = false;
        continue;
      }

      let mut l = line.split(": ");
      let input = l.next().unwrap();
      let value = l.next().unwrap().parse::<usize>().unwrap();
      assert!(l.next().is_none());
      assert!(value == 1 || value == 0);

      inputs.insert(input, value);
      if input.starts_with('z') {
        z_wires.insert(input);
      }
    } else {
      let c = ra.captures(line).expect("input format not as expected");
      if c.len() != 5 {
        panic!("bad match");
      }

      let inp1: &str = c.get(1).unwrap().as_str();
      let gate: &str = c.get(2).unwrap().as_str();
      let inp2: &str = c.get(3).unwrap().as_str();
      let outp: &str = c.get(4).unwrap().as_str();
      gates.push((inp1, gate, inp2, outp));

      if outp.starts_with('z') {
        z_wires.insert(outp);
      }
    }
  }
  (z_wires, inputs, gates)
}

pub fn analyse_input1(puzzle_input: &str) -> usize {
  let (z_wires, mut inputs, mut gates) = parse_input(puzzle_input);

  while !gates.is_empty() {
    for i in (0..gates.len()).rev() {
      let gate = gates[i];
      if !inputs.contains_key(gate.0) || !inputs.contains_key(gate.2) {
        continue;
      }
      let inpval1 = inputs.get(gate.0).unwrap();
      let inpval2 = inputs.get(gate.2).unwrap();
      let op = gate.1;
      let outp = gate.3;
      let outpval = match op {
        "OR" => {
          if *inpval1 == 1 || *inpval2 == 1 {
            1
          } else {
            0
          }
        }
        "AND" => {
          if *inpval1 == 1 && *inpval2 == 1 {
            1
          } else {
            0
          }
        }
        "XOR" => {
          if *inpval1 != *inpval2 {
            1
          } else {
            0
          }
        }
        _ => panic!("unexpected op {op}"),
      };
      inputs.insert(outp, outpval);
      gates.remove(i);
    }
  }

  let mut total = 0;
  for z in itertools::sorted(z_wires.iter()).rev() {
    total += inputs.get(z).unwrap();
    total <<= 1;
  }
  total >>= 1;

  total
}

pub fn analyse_input2(_puzzle_input: &str) -> usize {
  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1a() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 4);
  }

  #[test]
  fn test_load1b() {
    let data = load_data("testinput2.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 2024);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 23);
  }
}
