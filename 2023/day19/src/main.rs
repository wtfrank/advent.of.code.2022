use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;

//use advent::{Dims, TerrainMap};

//use enum_iterator::all;
//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::Reverse;
//use std::cmp::{max,Reverse,Ordering};
use std::collections::HashMap;
//use std::collections::HashSet;
//use std::collections::VecDeque;

//use std::iter::zip;

//use std::collections::HashSet;
use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

//use std::collections::HashMap;

//use advent::{prime_factors, lcm};
//use advent::Range;

/// Day 12 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

#[derive(Debug, Copy, Clone)]
struct ClosedInterval {
  start: usize,
  end: usize,
}

#[derive(Debug, Clone)]
struct XmasRange {
  x: ClosedInterval,
  m: ClosedInterval,
  a: ClosedInterval,
  s: ClosedInterval,
}

fn analyse_data2(workflows: &HashMap<String, Vec<WorkflowOp>>) -> usize {
  let mut score = 0;
  let wf: &str = "in";
  let ci = ClosedInterval { start: 1, end: 4000 };
  let r = XmasRange {
    x: ci,
    m: ci,
    a: ci,
    s: ci,
  };
  let mut queue = Vec::new();
  queue.push((r, wf));

  while let Some((mut r, wf)) = queue.pop() {
    if wf == "A" {
      score +=
        (r.x.end - r.x.start + 1) * (r.m.end - r.m.start + 1) * (r.a.end - r.a.start + 1) * (r.s.end - r.s.start + 1);
      println!("new score: {score} from {r:?}");
      continue;
    } else if wf == "R" {
      continue;
    }

    let ops = workflows.get(wf).unwrap();

    for op in ops {
      match op.operation {
        Ordering::Equal => {
          queue.push((r, &op.destination));
          break;
        }
        Ordering::Less => {
          let (start, end) = match op.category {
            'x' => (&mut r.x.start, &mut r.x.end),
            'm' => (&mut r.m.start, &mut r.m.end),
            'a' => (&mut r.a.start, &mut r.a.end),
            's' => (&mut r.s.start, &mut r.s.end),
            _ => panic!("bad category"),
          };
          if *start < op.value && *end < op.value {
            queue.push((r, &op.destination));
            break;
          } else if *start < op.value && *end >= op.value {
            //split op
            //let s2 = *start;
            let e2 = *end;

            *end = op.value - 1;
            let new_r = r.clone();
            println!("Less: {new_r:?}");
            queue.push((new_r, &op.destination));

            let (start, end) = match op.category {
              'x' => (&mut r.x.start, &mut r.x.end),
              'm' => (&mut r.m.start, &mut r.m.end),
              'a' => (&mut r.a.start, &mut r.a.end),
              's' => (&mut r.s.start, &mut r.s.end),
              _ => panic!("bad category"),
            };

            *start = op.value;
            *end = e2;
          } else {
            assert!(*start >= op.value);
            //try next op
          }
        }
        Ordering::Greater => {
          let (start, end) = match op.category {
            'x' => (&mut r.x.start, &mut r.x.end),
            'm' => (&mut r.m.start, &mut r.m.end),
            'a' => (&mut r.a.start, &mut r.a.end),
            's' => (&mut r.s.start, &mut r.s.end),
            _ => panic!("bad category"),
          };
          if *start > op.value && *end > op.value {
            queue.push((r, &op.destination));
            break;
          } else if *start < op.value && *end >= op.value {
            //split op
            let s2 = *start;
            //let e2 = *end;

            *start = op.value + 1;
            let new_r = r.clone();
            println!("Greater: {new_r:?}");
            queue.push((new_r, &op.destination));

            let (start, end) = match op.category {
              'x' => (&mut r.x.start, &mut r.x.end),
              'm' => (&mut r.m.start, &mut r.m.end),
              'a' => (&mut r.a.start, &mut r.a.end),
              's' => (&mut r.s.start, &mut r.s.end),
              _ => panic!("bad category"),
            };

            *start = s2;
            *end = op.value;
          } else {
            assert!(*end <= op.value);
            //try next op
          }
        }
      }
    }
  }

  score
}

fn analyse_data(parts: &[Part], workflows: &HashMap<String, Vec<WorkflowOp>>) -> usize {
  let mut score = 0;

  for part in parts {
    let mut wf: &str = "in";
    loop {
      if wf == "A" {
        score += part.0 + part.1 + part.2 + part.3;
        println!("new score: {score}");
        break;
      } else if wf == "R" {
        break;
      }
      let ops = workflows.get(wf).unwrap();
      for op in ops {
        match op.operation {
          Ordering::Equal => {
            wf = &op.destination;
            break;
          }
          Ordering::Less => {
            let val = match op.category {
              'x' => part.0,
              'm' => part.1,
              'a' => part.2,
              's' => part.3,
              _ => panic!("unexpected category"),
            };
            if val < op.value {
              wf = &op.destination;
              break;
            }
          }
          Ordering::Greater => {
            let val = match op.category {
              'x' => part.0,
              'm' => part.1,
              'a' => part.2,
              's' => part.3,
              _ => panic!("unexpected category"),
            };
            if val > op.value {
              wf = &op.destination;
              break;
            }
          }
        }
      }
    }
  }

  score
}

#[derive(Debug)]
struct WorkflowOp {
  category: char,
  operation: Ordering, //= means always goes to dest
  value: usize,
  destination: String,
}

type Part = (usize, usize, usize, usize);

fn load_data(filename: &str) -> (Vec<Part>, HashMap<String, Vec<WorkflowOp>>) {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut parts = Vec::new();
  let mut workflows = HashMap::<String, Vec<WorkflowOp>>::default();

  let mut in_parts = false;
  for line in contents.lines() {
    if line.is_empty() {
      in_parts = true;
      continue;
    }

    if in_parts {
      let r = sscanf::sscanf!(line, "{{x={usize},m={usize},a={usize},s={usize}}}").unwrap();
      parts.push((r.0, r.1, r.2, r.3));
    } else {
      let r = sscanf::sscanf!(line, "{String}{{{String}}}").unwrap();
      let name = r.0;
      let rules = r.1.split(',');
      let mut last_found = false;
      let mut v = Vec::new();
      for rule in rules {
        if last_found {
          panic!("rules carried on after last");
        }
        let s = sscanf::scanf!(rule, "{char}{char:/[<>]+/}{usize}:{String}");
        match s {
          Err(_) => {
            last_found = true;
            v.push(WorkflowOp {
              category: ' ',
              operation: Ordering::Equal,
              value: 0,
              destination: rule.to_string(),
            });
          }
          Ok(s1) => {
            let op = if s1.1 == '<' { Ordering::Less } else { Ordering::Greater };
            v.push(WorkflowOp {
              category: s1.0,
              operation: op,
              value: s1.2,
              destination: s1.3.to_string(),
            });
          }
        }
      }
      workflows.insert(name.to_string(), v);
    }
  }

  (parts, workflows)
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

  //let (score1, score2) = analyse_data(&mut data);
  let data = load_data("input19.txt");
  let score1 = analyse_data(&data.0, &data.1);
  println!("score1: {score1}");
  let score2 = analyse_data2(&data.1);
  println!("score1: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput.txt");
    let score1 = analyse_data(&data.0, &data.1);
    assert_eq!(score1, 19114);
  }
  #[test]
  fn test_load2() {
    let data = load_data("testinput.txt");
    let score1 = analyse_data2(&data.1);
    assert_eq!(score1, 167409079868000);
  }
}
