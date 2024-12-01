use clap::Parser;
use std::fs::File;
use std::io::Read;
//use log::debug;

/// Day 1 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn difference(a: usize, b: usize) -> usize {
  let (res, overflowed) = a.overflowing_sub(b);

  if overflowed {
    usize::MAX - res + 1
  } else {
    res
  }
}

fn list_dist(filename: &str) -> usize {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut distance: usize = 0;
  let mut left_list = Vec::<usize>::new();
  let mut right_list = Vec::<usize>::new();

  for line in contents.lines() {
    let mut tokens = line.split_whitespace();
    let left = tokens.next().unwrap().parse::<usize>().unwrap();
    let right = tokens.next().unwrap().parse::<usize>().unwrap();
    assert_eq!(None, tokens.next());
    left_list.push(left);
    right_list.push(right);
  }

  left_list.sort();
  right_list.sort();

  for i in 0..left_list.len() {
    let left = left_list[i];
    let right = right_list[i];
    distance += difference(left, right);
  }

  distance
}

fn list_dist2(filename: &str) -> usize {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut distance: usize = 0;
  let mut left_list = Vec::<usize>::new();
  let mut right_list = Vec::<usize>::new();

  for line in contents.lines() {
    let mut tokens = line.split_whitespace();
    let left = tokens.next().unwrap().parse::<usize>().unwrap();
    let right = tokens.next().unwrap().parse::<usize>().unwrap();
    assert_eq!(None, tokens.next());
    left_list.push(left);
    right_list.push(right);
  }

  left_list.sort();
  right_list.sort();

  for i in 0..left_list.len() {
    let left = left_list[i];
    let mut count = 0;
    for j in 0..right_list.len() {
      match left.cmp(&right_list[j]) {
        std::cmp::Ordering::Less => continue,
        std::cmp::Ordering::Equal => count += 1,
        std::cmp::Ordering::Greater => continue,
      }
    }

    distance += left * count;
    // println!("val {left} occured {count} times");
  }

  distance
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let answer1 = list_dist("input1.txt");
  println!("answer: {answer1}");
  let answer2 = list_dist2("input1.txt");
  println!("answer2: {answer2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let calib = list_dist("testinput1.txt");
    assert_eq!(calib, 11);
  }

  #[test]
  fn test_load2() {
    let calib = list_dist2("testinput1.txt");
    assert_eq!(calib, 31);
  }
}
