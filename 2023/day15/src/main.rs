use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

//use advent::{determine_map_dims, Point, TerrainMap};

//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::{max,Reverse,Ordering};
//use std::collections::HashSet;
//use std::collections::HashMap;
use std::collections::VecDeque;

//use std::iter::zip;

//use std::collections::HashSet;
//use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

//use std::collections::HashMap;

//use advent::{prime_factors, lcm};

/// Day 12 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}


#[allow(dead_code)]
fn analyse_data(_data: &mut [char]) -> (usize, usize) {

  (0,0)
}

fn hash_step(s: &str) -> usize {
  let mut cv:usize = 0;
  for b in s.as_bytes() {
    cv += *b as usize;
    cv *= 17;
    cv %= 256;
  }

  cv
}

fn load_data(filename: &str) -> (usize,usize) {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let contents = contents.replace('\n', "");
  let contents = contents.replace('\r', "");

  let score1 = contents.split(',').map(hash_step).reduce(|acc, h| acc + h).unwrap();
  let mut score2 = 0;

  let mut boxes = Vec::<VecDeque<(String, usize)>>::new();
  for _ in 0..256 {
    boxes.push(VecDeque::new());
  }

  for group in contents.split(',') {
    let idx:usize = match group.find('=') {
      Some(a) => a,
      None => group.find('-').unwrap(),
    };
    let (label, action) = group.split_at(idx);
    assert_eq!(label.len(), idx);
    let b = hash_step(label);
    let v = boxes.get_mut(b).unwrap();
    if action == "-" {
      let mut idx = usize::MAX;
      for (i,(lab,_)) in v.iter().enumerate() {
        if label == lab {
          idx = i;
          break;
        }
      }
      if idx != usize::MAX {
        v.remove(idx);
        println!("removed {label} box {b}");
      }
    }
    else {
      let mut it = action.chars();
      assert_eq!(it.next().unwrap(), '=');
      let power = (it.next().unwrap() as u32 - '0' as u32) as usize;
      let mut found = false;
      for (lab,pow) in v.iter_mut() {
        if label == lab {
          found = true;
          *pow = power;
          println!("replaced {label} power {power} in box {b}");
          break;
        }
      }
      if !found {
        v.push_back( (label.to_string(), power) );
        println!("added {label} power {power} to back of box {b}");
      }
    }
  }

  for (bn, bx) in boxes.iter().enumerate() {
    for (ln, (_, foc)) in bx.iter().enumerate() {
      println!("box {bn}, slot {ln}, focal length {foc}");
      score2 += (bn +1) * (ln+1) * foc;
    }
  }




  (score1, score2)
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

  //let mut data = load_data("input15.txt");
  //let (score1, score2) = analyse_data(&mut data);
  let (score1, score2) = load_data("input15.txt");
  println!("score1: {score1}");
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    //let mut data = load_data("testinput.txt");
    //let (score1, _score2) = analyse_data(&mut data);
    let (score1, score2) = load_data("testinput.txt");
    assert_eq!(score1, 1320);
    assert_eq!(score2, 145);
  }
}
