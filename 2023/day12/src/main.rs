use clap::Parser;
use std::fmt;
use std::fs::File;
use std::io::Read;

use std::str::FromStr;

//use advent::{determine_map_dims, Point, TerrainMap};

//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::{max,Reverse,Ordering};
//use std::collections::HashSet;
use std::collections::HashMap;

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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Spring {
  Unknown,
  Operational,
  Damaged,
}

impl fmt::Display for Spring {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Spring::Unknown => "Un",
        Spring::Operational => "Op",
        Spring::Damaged => "Da",
      }
    )
  }
}

fn multi_pop(v: &mut Vec<Spring>, count: usize) {
  let mut count = count;
  while count > 0 {
    count -= 1;
    v.pop();
  }
}

fn do_analyse_fast<'a>(
  cond: &'a [Spring],
  seq: &'a [usize],
  in_seq: bool,
  seq_remain: usize,
  memo: &mut HashMap<(&'a [Spring], &'a [usize]), usize>,
) -> usize {
  let mut score = 0;

  let mut in_seq = in_seq;
  let mut seq_remain = seq_remain;

  let mut pos: isize = -1;
  let mut seq_pos = 0;
  loop {
    pos += 1;
    if pos as usize >= cond.len() {
      break;
    }
    let c = &cond[pos as usize];

    match c {
      Spring::Operational => {
        if in_seq && seq_remain > 0 {
          //println!("seq not possible");
          return score;
        }
        in_seq = false;
      }
      Spring::Damaged => {
        match in_seq {
          true => {
            if seq_remain == 0 {
              //println!("seq not possible");
              return score;
            } else {
              seq_remain -= 1;
            }
          }
          false => {
            in_seq = true;
            if seq_pos >= seq.len() {
              //println!("seq not possible");
              return score;
            }
            seq_remain = seq[seq_pos] - 1;
            seq_pos += 1;
          }
        }
      }
      Spring::Unknown => {
        let key = (&cond[pos as usize..], &seq[seq_pos..]);
        if !in_seq && memo.contains_key(&key) {
          score += memo.get(&key).unwrap();
        } else {
          let mut part_score = 0;
          //assume damaged
          if in_seq && seq_remain > 0 {
            part_score += do_analyse_fast(
              &cond[pos as usize + 1..],
              &seq[seq_pos..],
              in_seq,
              seq_remain - 1,
              memo,
            );
          } else if !in_seq && seq_pos < seq.len() {
            part_score += do_analyse_fast(
              &cond[pos as usize + 1..],
              &seq[seq_pos + 1..],
              true,
              seq[seq_pos] - 1,
              memo,
            );
          }

          //assume working
          if in_seq && seq_remain == 0 {
            part_score +=
              do_analyse_fast(&cond[pos as usize + 1..], &seq[seq_pos..], false, 0, memo);
          }
          if !in_seq {
            part_score +=
              do_analyse_fast(&cond[pos as usize + 1..], &seq[seq_pos..], false, 0, memo);
          }

          memo.insert(key, part_score);
          score += part_score;
        }

        //once we've reached an unknown, then our chidlren are responsible for calculations so
        //don't score here
        in_seq = true;
        seq_remain = 10000;
        break;
      }
    }
  }
  if seq_pos >= seq.len() && (!in_seq || seq_remain == 0) {
    score += 1;
    //println!("score bump to {score} pos {pos} cur_seq len {}", cur_seq.len());
  }

  if score > 0 {
    //println!("returning {score}");
  }
  score
}

fn do_analyse(
  cond: &[Spring],
  seq: &[usize],
  orig_len: usize,
  in_seq: bool,
  seq_remain: usize,
  cur_seq: &mut Vec<Spring>,
) -> usize {
  assert_eq!(cur_seq.len() + cond.len(), orig_len);
  let start_cs_len = cur_seq.len();
  //println!("cur_seq {cur_seq:?} cond {cond:?} seq {} in_seq {in_seq} seq_remain {seq_remain}", seq.len());
  let mut score = 0;

  let mut in_seq = in_seq;
  let mut seq_remain = seq_remain;

  let mut pos: isize = -1;
  let mut seq_pos = 0;
  let mut pushes = 0;
  loop {
    pos += 1;
    if pos as usize >= cond.len() {
      break;
    }
    let c = &cond[pos as usize];
    //println!("c {c}, pos: {pos}/{}, in_seq: {in_seq}, seq_remain: {seq_remain}, cur_seq: {cur_seq:?}, cond: {:?}", cond.len(), &cond[ pos as usize .. ]);
    assert_eq!(cond.len() - pos as usize + cur_seq.len(), orig_len);

    match c {
      Spring::Operational => {
        if in_seq && seq_remain > 0 {
          multi_pop(cur_seq, pushes);
          assert_eq!(cur_seq.len(), start_cs_len);
          //println!("seq not possible");
          return score;
        }
        in_seq = false;
        cur_seq.push(*c);
        pushes += 1;
      }
      Spring::Damaged => {
        match in_seq {
          true => {
            if seq_remain == 0 {
              multi_pop(cur_seq, pushes);
              assert_eq!(cur_seq.len(), start_cs_len);
              //println!("seq not possible");
              return score;
            } else {
              seq_remain -= 1;
            }
          }
          false => {
            in_seq = true;
            if seq_pos >= seq.len() {
              multi_pop(cur_seq, pushes);
              assert_eq!(cur_seq.len(), start_cs_len);
              //println!("seq not possible");
              return score;
            }
            seq_remain = seq[seq_pos] - 1;
            seq_pos += 1;
          }
        }
        cur_seq.push(*c);
        pushes += 1;
      }
      Spring::Unknown => {
        //assume damaged
        if in_seq && seq_remain > 0 {
          cur_seq.push(Spring::Damaged);
          let csl = cur_seq.len();
          score += do_analyse(
            &cond[pos as usize + 1..],
            &seq[seq_pos..],
            orig_len,
            in_seq,
            seq_remain - 1,
            cur_seq,
          );
          assert_eq!(cur_seq.len(), csl);
          cur_seq.pop();
        } else if !in_seq && (seq_pos) < seq.len() {
          cur_seq.push(Spring::Damaged);
          let csl = cur_seq.len();
          score += do_analyse(
            &cond[pos as usize + 1..],
            &seq[seq_pos + 1..],
            orig_len,
            true,
            seq[seq_pos] - 1,
            cur_seq,
          );
          assert_eq!(cur_seq.len(), csl);
          cur_seq.pop();
        }

        //assume working
        if in_seq && seq_remain == 0 {
          cur_seq.push(Spring::Operational);
          let csl = cur_seq.len();
          score += do_analyse(
            &cond[pos as usize + 1..],
            &seq[seq_pos..],
            orig_len,
            false,
            0,
            cur_seq,
          );
          assert_eq!(cur_seq.len(), csl);
          cur_seq.pop();
        }
        if !in_seq {
          cur_seq.push(Spring::Operational);
          let csl = cur_seq.len();
          score += do_analyse(
            &cond[pos as usize + 1..],
            &seq[seq_pos..],
            orig_len,
            false,
            0,
            cur_seq,
          );
          assert_eq!(cur_seq.len(), csl);
          cur_seq.pop();
        }

        //once we've reached an unknown, then our chidlren are responsible for calculations
        in_seq = true;
        seq_remain = 100;
        break;
      }
    }
  }
  if seq_pos >= seq.len() && (!in_seq || seq_remain == 0) {
    score += 1;
    //println!("score bump to {score} pos {pos} cur_seq len {}", cur_seq.len());
  }

  multi_pop(cur_seq, pushes);
  assert_eq!(cur_seq.len(), start_cs_len);

  if score > 0 {
    //println!("returning {score}");
  }
  score
}

fn analyse_fast(cond: &[Spring], seq: &[usize]) -> usize {
  let mut score = 0;

  let mut memo = HashMap::<(&[Spring], &[usize]), usize>::new();

  score += do_analyse_fast(cond, seq, false, 0, &mut memo);

  score
}

fn analyse(cond: &[Spring], seq: &[usize]) -> usize {
  let mut score = 0;

  let mut cur_seq = Vec::new();

  score += do_analyse(cond, seq, cond.len(), false, 0, &mut cur_seq);

  score
}

fn load_data(filename: &str) -> Vec<(Vec<Spring>, Vec<usize>)> {
  let mut data = Vec::new();
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  for line in contents.lines() {
    let mut condition = Vec::new();
    let mut seq = Vec::new();

    let mut s = line.split(' ');
    let s_cond = s.next().unwrap();
    let s_seq = s.next().unwrap();

    for c in s_cond.chars() {
      match c {
        '.' => {
          condition.push(Spring::Operational);
        }
        '#' => {
          condition.push(Spring::Damaged);
        }
        '?' => {
          condition.push(Spring::Unknown);
        }
        _ => {
          panic!("unexpected spring state {c}");
        }
      }
    }

    let s2 = s_seq.split(',');
    for num in s2 {
      let Ok(count) = usize::from_str(num) else {
        panic!("Couldn#t parse number: {num}");
      };
      seq.push(count);
    }

    data.push((condition, seq));
  }

  //sequences.push( line.split(' ').map( |a| a.parse::<isize>().unwrap() ).collect() );
  //let r = sscanf::sscanf!(line, "{String} = ({String}, {String})").unwrap();

  data
}

fn expand_data(data: Vec<(Vec<Spring>, Vec<usize>)>) -> Vec<(Vec<Spring>, Vec<usize>)> {
  let mut data2 = Vec::new();

  for (cond, seq) in &data {
    let mut new_cond = Vec::new();
    for c in cond {
      new_cond.push(*c);
    }
    for _ in 0..4 {
      new_cond.push(Spring::Unknown);
      for c in cond {
        new_cond.push(*c);
      }
    }

    let mut new_seq = Vec::new();
    for _ in 0..5 {
      for s in seq {
        new_seq.push(*s);
      }
    }

    data2.push((new_cond, new_seq));
  }

  data2
}

fn analyse_data3(data: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
  let data2 = expand_data(data);
  analyse_data_fast(data2)
}

#[cfg(test)]
fn analyse_data2(data: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
  let data2 = expand_data(data);
  analyse_data(data2)
}

fn analyse_data_fast(data: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
  let mut score = 0;
  for (condition, seq) in &data {
    let row_score = analyse_fast(condition, seq);
    println!("row score: {row_score}");
    score += row_score;
  }

  score
}

fn analyse_data(data: Vec<(Vec<Spring>, Vec<usize>)>) -> usize {
  let mut score = 0;
  for (condition, seq) in &data {
    let row_score = analyse(condition, seq);
    println!("row score: {row_score}");
    score += row_score;
  }

  score
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input12.txt");
  let score1 = analyse_data(data.clone());
  println!("score1: {score1}");
  let score2 = analyse_data3(data);
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput.txt");
    let score = analyse_data(data);
    assert_eq!(score, 21);
  }
  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let score = analyse_data(data);
    assert_eq!(score, 4);
  }
  #[test]
  fn test_load3() {
    let data = load_data("testinput3.txt");
    let score = analyse_data(data);
    assert_eq!(score, 10);
  }
  #[test]
  fn test_load4() {
    let data = load_data("testinput4.txt");
    let score = analyse_data(data);
    assert_eq!(score, 1);
  }

  #[test]
  fn test_load5() {
    let data = load_data("testinput5.txt");
    let score = analyse_data(data);
    assert_eq!(score, 2);
  }

  #[test]
  fn test_load6() {
    let data = load_data("testinput6.txt");
    let score = analyse_data(data);
    assert_eq!(score, 15);
  }

  #[test]
  fn test_load7() {
    let data = load_data("testinput7.txt");
    let score = analyse_data(data);
    assert_eq!(score, 3);
  }

  #[test]
  fn test_load8() {
    let data = load_data("testinput8.txt");
    let score = analyse_data(data);
    assert_eq!(score, 2);
  }

  #[test]
  fn test_part9() {
    let data = load_data("testinput.txt");
    let score = analyse_data2(data);
    assert_eq!(score, 525152);
  }

  #[test]
  fn test_part10() {
    let data = load_data("testinput.txt");
    let score = analyse_data3(data);
    assert_eq!(score, 525152);
  }
}
