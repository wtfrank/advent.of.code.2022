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

#[allow(unused_imports)]
use priority_queue::PriorityQueue;
#[allow(unused_imports)]
use std::cmp::Reverse;

#[allow(unused_imports)]
use enum_iterator::all;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Day 17 of Advent of Code 2024
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

#[derive(Default, Debug, Clone, Copy, PartialEq, FromPrimitive)]
enum OpCode {
  #[default]
  Adv = 0,
  Bxl = 1,
  Bst = 2,
  Jnz = 3,
  Bxc = 4,
  Out = 5,
  Bdv = 6,
  Cdv = 7,
}

fn load_prog(puzzle_input: &str) -> (String, Vec<usize>, usize, usize, usize) {
  let mut lines = puzzle_input.lines();
  let rega = lines
    .next()
    .unwrap()
    .split("Register A: ")
    .nth(1)
    .unwrap()
    .parse::<usize>()
    .unwrap();
  let regb = lines
    .next()
    .unwrap()
    .split("Register B: ")
    .nth(1)
    .unwrap()
    .parse::<usize>()
    .unwrap();
  let regc = lines
    .next()
    .unwrap()
    .split("Register C: ")
    .nth(1)
    .unwrap()
    .parse::<usize>()
    .unwrap();

  let prog_str = lines.nth(1).unwrap().split("Program: ").nth(1).unwrap().to_string();

  let prog: Vec<usize> = prog_str
    .split(",")
    .map(|s| s.parse::<usize>().unwrap())
    //    .map(|p| <OpCode as FromPrimitive>::from_usize(p).unwrap())
    .collect();

  /*
  println!("{rega} {regb} {regc} {prog:?}");

  let mut it = prog.iter().peekable();
  while it.peek().is_some() {
    let op = <OpCode as FromPrimitive>::from_usize(*it.next().unwrap()).unwrap();
    let arg = it.next().unwrap();

    print!("{op:?} {arg}, ");
  }
  println!();*/

  (prog_str, prog, rega, regb, regc)
}

fn execute_prog(prog: &[usize], rega: usize, regb: usize, regc: usize) -> String {
  let mut rega = rega;
  let mut regb = regb;
  let mut regc = regc;

  let mut output = "".to_string();
  let mut ip = 0;

  loop {
    if ip >= prog.len() {
      break;
    }

    let op = <OpCode as FromPrimitive>::from_usize(prog[ip]).unwrap();
    let arg = prog[ip + 1];

    match op {
      OpCode::Adv => {
        let num = rega;
        let dem = 2_usize.pow(combo(arg, rega, regb, regc) as u32);
        rega = num / dem;
        ip += 2;
      }
      OpCode::Bxl => {
        regb ^= arg;
        ip += 2;
      }
      OpCode::Bst => {
        regb = combo(arg, rega, regb, regc) % 8;
        ip += 2;
      }
      OpCode::Jnz => {
        if rega == 0 {
          ip += 2;
        } else {
          ip = arg;
        }
      }
      OpCode::Bxc => {
        regb ^= regc;
        ip += 2;
      }
      OpCode::Out => {
        let val = combo(arg, rega, regb, regc) % 8;
        output += &format!("{val},");
        ip += 2;
      }
      OpCode::Bdv => {
        let num = rega;
        let dem = 2_usize.pow(combo(arg, rega, regb, regc) as u32);
        regb = num / dem;
        ip += 2;
      }
      OpCode::Cdv => {
        let num = rega;
        let dem = 2_usize.pow(combo(arg, rega, regb, regc) as u32);
        regc = num / dem;
        ip += 2;
      }
    }
  }

  output.pop();
  output
}

fn analyse_input1(puzzle_input: &str) -> String {
  let (_, prog, rega, regb, regc) = load_prog(puzzle_input);
  execute_prog(&prog, rega, regb, regc)
}

fn combo(arg: usize, rega: usize, regb: usize, regc: usize) -> usize {
  match arg {
    0..=3 => arg,
    4 => rega,
    5 => regb,
    6 => regc,
    _ => panic!("invalid programme"),
  }
}

#[allow(dead_code)]
fn analyse_input_brute_force(puzzle_input: &str) -> usize {
  let (prog_str, prog, _rega, regb, regc) = load_prog(puzzle_input);

  let mut i = 0;
  loop {
    let rega = i;
    let out2 = execute_prog(&prog, rega, regb, regc);
    if out2 == prog_str {
      break;
    }

    if i % 1000 == 0 {
      println!("{i}");
    }
    i += 1;
  }
  i
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let (_prog_str, prog, _rega, _regb, _regc) = load_prog(puzzle_input);

  /*
  let mut prev_a = 0;
  for i in (0..prog.len()).rev() {
    let p = prog[i];

    let mut num_cands = 0;
    let mut a_cand = 0;
    for al in 0_usize..8_usize {
      let a = prev_a * 8 + al;

      let b = (a % 8) ^ 6;
      let c = a.checked_shr(((a % 8) ^ 3) as u32).unwrap();
      if p == (b ^ c) % 8 {
        println!("pos {i}: {a}");
        num_cands += 1;
        a_cand = a;
      } else {
        println!("pos {i}: {al} not valid.");
      }
    }
    if num_cands == 1 {
      prev_a = a_cand;
    } else {
      panic!("pos {i} had {num_cands} for a");
    }
  }*/

  let (valid, a) = check_valid(prog.len(), 0, &prog);
  assert!(valid);
  a
}

fn check_valid(prog_pos: usize, prev_a: usize, prog: &[usize]) -> (bool, usize) {
  if prog_pos == 0 {
    return (true, prev_a);
  }

  let p = prog[prog_pos - 1];
  for al in 0_usize..8_usize {
    let a = prev_a * 8 + al;

    let b = (a % 8) ^ 6;
    let c = a.checked_shr(((a % 8) ^ 3) as u32).unwrap();
    if p == (b ^ c) % 8 {
      // println!("pos {prog_pos}: {a}");
      let (valid, orig_a) = check_valid(prog_pos - 1, a, prog);
      if valid {
        return (valid, orig_a);
      }
    } else {
      // println!("pos {i}: {al} not valid.");
    }
  }

  (false, 0)
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input17.txt");
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
    assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 117440);
  }

  #[test]
  fn test_load1a() {
    let data = load_data("testinput3.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, "2,4,1,3,7,5,1,5,0,3,4,2,5,5,3,0");
  }

  // ---------------------------------------------
  use test::{black_box, Bencher};
  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("input17.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("input17.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
