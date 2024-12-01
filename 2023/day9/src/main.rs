use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::iter::zip;

//use std::collections::HashSet;
//use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

//use std::collections::HashMap;

//use advent::{prime_factors, lcm};

/// Day 9 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn extrapolate_seq(seq: &[isize]) -> isize {
  let mut deltas = Vec::new();
  let mut all_zeroes = true;
  for i in 1..seq.len() {
    let val = seq[i] - seq[i - 1];
    deltas.push(val);
    if val != 0 {
      all_zeroes = false;
    }
  }
  println!("deltas: {deltas:?}");

  let mut final_val = *deltas.last().unwrap();
  if !all_zeroes {
    final_val += extrapolate_seq(&deltas);
  }
  println!("final: {final_val}");
  final_val
}

fn analyse_data(sequences: &[Vec<isize>]) -> isize {
  let sum = sequences.iter().map(|s| *s.last().unwrap() + extrapolate_seq(s)).sum();

  println!("seqs: {sequences:?}");

  sum
}

fn analyse_data2(sequences: &[Vec<isize>]) -> isize {
  let revs: Vec<Vec<isize>> = sequences
    .iter()
    .map(|s| s.iter().rev().copied().collect::<Vec<_>>())
    .collect();
  /*
  let mut revs:Vec<Vec<isize>> = Vec::new();
  for s in sequences {
    let sr:Vec<isize> = s.iter().rev().map(|a|*a).collect::<Vec<_>>();
    revs.push( sr );
  }*/
  let sum = revs
    .iter()
    .map(|s: &Vec<isize>| *s.last().unwrap() + extrapolate_seq(s))
    .sum();

  println!("seqs: {sequences:?}");

  sum
}

fn load_data(filename: &str) -> Vec<Vec<isize>> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut sequences = Vec::<Vec<isize>>::new();
  for line in contents.lines() {
    sequences.push(line.split(' ').map(|a| a.parse::<isize>().unwrap()).collect());
    //let r = sscanf::sscanf!(line, "{String} = ({String}, {String})").unwrap();
  }
  sequences
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let sequences = load_data("input9.txt");
  let score1 = analyse_data(&sequences);
  println!("score1: {score1}");
  let score2 = analyse_data2(&sequences);
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput.txt");
    let score = analyse_data(&data);
    assert_eq!(score, 114);
  }
  #[test]
  fn test_load2() {
    let data = load_data("testinput.txt");
    let score = analyse_data2(&data);
    assert_eq!(score, 2);
  }
}
