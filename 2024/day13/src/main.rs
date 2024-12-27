#![feature(test)]
extern crate test;

use aoc_2024_day13::*;

use clap::Parser;
#[allow(unused_imports)]
use std::cmp::PartialEq;
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
use enum_iterator::all;

/// Day 13 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input13.txt");
  let answer1 = analyse_input1(&data);
  println!("answer: {answer1} (36991 too low))");
  let answer2 = analyse_input2(&data);
  println!("answer2: {answer2}");
}
