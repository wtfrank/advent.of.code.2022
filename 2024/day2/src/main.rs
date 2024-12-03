use clap::Parser;
use std::fs::File;
use std::io::Read;
//use log::debug;

/// Day 2 of Advent of Code 2024
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

fn valid_report(report: &[usize]) -> bool {
  let mut dir = std::cmp::Ordering::Equal;

  for i in 1..report.len() {
    let prev = report[i - 1];
    let cur = report[i];

    let diff = difference(prev, cur);
    // diff < 1 || diff > 3 {
    if !(1..=3).contains(&diff) {
      // println!("not valid due to diff {diff}");
      return false;
    }

    let this_dir = prev.cmp(&cur);

    // println!("prev {prev}, cur {cur}");
    match dir {
      std::cmp::Ordering::Equal => {
        dir = this_dir;
      }
      _ => {
        if this_dir != dir {
          // println!("not valid due to dir change");
          return false;
        }
      }
    };
  }

  // println!("valid");
  true
}

fn valid_report_damped(report: &[usize]) -> bool {
  if valid_report(report) {
    return true;
  }

  for i in 0..report.len() {
    let mut damped = report.to_vec();
    damped.remove(i);
    if valid_report(&damped) {
      return true;
    }
  }
  false
}

fn analyse_input1(filename: &str) -> usize {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut reports = Vec::<Vec<usize>>::new();

  for line in contents.lines() {
    let tokens = line.split_whitespace();
    let report: Vec<usize> = tokens.into_iter().map(|t| t.parse::<usize>().unwrap()).collect();
    reports.push(report);
  }

  reports.iter().fold(0, |acc, x| {
    acc
      + match valid_report(x) {
        true => 1,
        false => 0,
      }
  })
}

fn analyse_input2(filename: &str) -> usize {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut reports = Vec::<Vec<usize>>::new();

  for line in contents.lines() {
    let tokens = line.split_whitespace();
    let report: Vec<usize> = tokens.into_iter().map(|t| t.parse::<usize>().unwrap()).collect();
    reports.push(report);
  }

  reports.iter().fold(0, |acc, x| {
    acc
      + match valid_report_damped(x) {
        true => 1,
        false => 0,
      }
  })
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let answer1 = analyse_input1("input2.txt");
  println!("answer: {answer1}");
  let answer2 = analyse_input2("input2.txt");
  println!("answer2: {answer2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let calib = analyse_input1("testinput1.txt");
    assert_eq!(calib, 2);
  }

  #[test]
  fn test_load2() {
    let calib = analyse_input2("testinput1.txt");
    assert_eq!(calib, 4);
  }
}
