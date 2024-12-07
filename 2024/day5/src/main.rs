#![feature(test)]
extern crate test;

use clap::Parser;
#[allow(unused_imports)]
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

/// Day 5 of Advent of Code 2024
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

fn analyse_input1(puzzle_input: &str) -> usize {
  let mut total = 0;

  let mut comes_after = HashMap::<usize, Vec<usize>>::default();
  let mut updates = Vec::<Vec<usize>>::new();

  let mut loading_rules = true;
  for line in puzzle_input.lines() {
    if line.is_empty() {
      loading_rules = false;
      continue;
    }
    if loading_rules {
      let mut s = line.split("|");
      let before = s.next().unwrap().parse::<usize>().unwrap();
      let after = s.next().unwrap().parse::<usize>().unwrap();
      comes_after.entry(after).or_insert(Vec::new()).push(before);
    } else {
      updates.push(line.split(",").map(|l| l.parse::<usize>().unwrap()).collect());
    }
  }

  for update in updates {
    let mut present = HashSet::<usize>::default();
    for u in update.iter() {
      present.insert(*u);
    }

    let mut valid = true;
    let mut before = HashSet::<usize>::default();
    'outer: for u in update.iter() {
      if comes_after.contains_key(u) {
        let vs = comes_after.get(u).unwrap();
        for v in vs {
          if present.contains(v) && !before.contains(v) {
            println!("invalid: needed {v} before {u}");
            valid = false;
            break 'outer;
          }
        }
      }
      before.insert(*u);
    }
    if valid {
      total += update[(update.len() - 1) / 2];
      println!("Valid");
    } else {
      println!("Invalid");
    }
  }
  total
}

fn reorder(present: &mut HashSet<usize>, comes_after: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
  let mut ordered = Vec::<usize>::new();

  // put any with no dependencies at the start of the list.
  // then generate new set of rules, that excludes those already in the list
  // then put any that now have no dependencies in the list
  // etc

  let prev_comes_after = comes_after;

  loop {
    // generate new rules
    let mut comes_after = HashMap::default();
    for (after, befores) in prev_comes_after.iter() {
      if !present.contains(after) {
        continue;
      }
      let mut new_befores = Vec::<usize>::new();
      for before in befores.iter() {
        if present.contains(before) {
          new_befores.push(*before);
        }
      }
      if !new_befores.is_empty() {
        comes_after.insert(after, new_befores);
      }
    }

    let mut deferred_removals = Vec::<usize>::new();
    for p in present.iter() {
      if !comes_after.contains_key(p) {
        ordered.push(*p);
        deferred_removals.push(*p);
      }
    }

    for d in deferred_removals {
      present.remove(&d);
    }

    if present.is_empty() {
      break;
    }
  }

  ordered
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;

  let mut comes_after = HashMap::<usize, Vec<usize>>::default();
  let mut updates = Vec::<Vec<usize>>::new();

  let mut loading_rules = true;
  for line in puzzle_input.lines() {
    if line.is_empty() {
      loading_rules = false;
      continue;
    }
    if loading_rules {
      let mut s = line.split("|");
      let before = s.next().unwrap().parse::<usize>().unwrap();
      let after = s.next().unwrap().parse::<usize>().unwrap();
      comes_after.entry(after).or_insert(Vec::new()).push(before);
    } else {
      updates.push(line.split(",").map(|l| l.parse::<usize>().unwrap()).collect());
    }
  }

  for update in updates {
    let mut present = HashSet::<usize>::default();
    for u in update.iter() {
      present.insert(*u);
    }

    let mut valid = true;
    let mut before = HashSet::<usize>::default();
    'outer: for u in update.iter() {
      if comes_after.contains_key(u) {
        let vs = comes_after.get(u).unwrap();
        for v in vs {
          if present.contains(v) && !before.contains(v) {
            println!("invalid: needed {v} before {u}");
            valid = false;
            break 'outer;
          }
        }
      }
      before.insert(*u);
    }
    if !valid {
      let fixed_update = reorder(&mut present, &comes_after);
      total += fixed_update[(fixed_update.len() - 1) / 2];
    }
  }

  total
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input5.txt");
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
    assert_eq!(result, 143);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 123);
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input5.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input5.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
