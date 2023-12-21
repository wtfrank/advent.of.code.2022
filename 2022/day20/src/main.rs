/*
 one approach:
 array of (original number, current position)
 each movement calculates via modulus where the number ends up.
 Any number with current position between start/end gets shifted +1 or -1.
 To finish, generate a new array by sorting old array.

 This means a pass through entire array for each movement.


 another approach:
   maintain array original number positions, and array of current position
   maintain a pointer from original number to current position object.
   involves shuffling pointers in current position array.

*/

use clap::Parser;
use std::fs::File;
use std::io::Read;

/// Day 20 of Advent of Code 2022
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

#[derive(Debug, PartialEq, Clone)]
struct NumberPosition {
  num: i16,
  pos: i16,
}

#[derive(Debug, PartialEq)]
struct VecD<T>(Vec<T>);

impl<NumberPosition> std::fmt::Display for VecD<NumberPosition> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}

impl<T> VecD<T> {
  pub fn new() -> VecD<T> {
    VecD(Vec::<T>::new())
  }
  pub fn len(&self) -> usize {
    //let Vec::<T>(v) = self;
    self.0.len()
  }
  pub fn push(&mut self, item: T) {
    self.0.push(item);
  }
  pub fn get(&self, idx: usize) -> Option<&T> {
    self.0.get(idx)
  }
  pub fn get_mut(&mut self, idx: usize) -> Option<&mut T> {
    self.0.get_mut(idx)
  }
  pub fn iter(&self) -> std::slice::Iter<'_, T> {
    self.0.iter()
  }
  pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
    self.0.iter_mut()
  }
  pub fn sort_by<F>(&mut self, compare: F)
  where
    F: FnMut(&T, &T) -> std::cmp::Ordering,
  {
    self.0.sort_by(compare)
  }
}

fn mix_numbers(numbers: &mut VecD<NumberPosition>) {
  let total = numbers.len();
  for i in 0..total {
    //println!("round {i}");
    do_mix(numbers, i);
  }
}

fn pos_mod(a: i16, b: i16) -> i16 {
  let c: i16 = a % b;
  if c < 0 {
    c + b
  } else {
    c
  }
}

fn do_mix(numbers: &mut VecD<NumberPosition>, i: usize) {
  let total = numbers.len();
  for _np in numbers.iter() {
    //println!("pos {}: {}", np.position, np.number);
  }

  let np = numbers.get(i).unwrap();
  let start_pos = np.pos;
  let mut end_pos = np.pos + np.num;
  if np.num < 0 {
    end_pos -= 1; //changing direction means the position has to move through itself
  }
  end_pos = pos_mod(end_pos, total as i16);

  println!("pos {start_pos} to pos {end_pos}");

  let range1s = start_pos;
  let range1e = if end_pos >= start_pos { end_pos } else { total as i16 };
  let range2e = if end_pos >= start_pos { -1 } else { end_pos };

  for np2 in numbers.iter_mut() {
    if (np2.pos > range1s && np2.pos <= range1e) || np2.pos <= range2e {
      np2.pos = pos_mod(np2.pos + total as i16 - 1, total as i16);
    }
  }

  let np = numbers.get_mut(i).unwrap();
  np.pos = end_pos;
}

fn calc_score(numbers: &mut VecD<NumberPosition>) -> i16 {
  numbers.sort_by(|a, b| a.pos.cmp(&b.pos));

  let zero_pos: usize = numbers.iter().find(|n| n.num == 0).unwrap().pos as usize;
  //println!("zero_pos: {zero_pos}");

  /*
  for np in numbers.iter() {
    println!("pos {}: {}", np.position, np.number);
  }*/

  let pos1 = (zero_pos + 1000) % numbers.len();
  let pos2 = (zero_pos + 2000) % numbers.len();
  let pos3 = (zero_pos + 3000) % numbers.len();

  println!("pos1 value: {}@{pos1}", numbers.get(pos1).expect("logic error").num);
  println!("pos2 value: {}@{pos2}", numbers.get(pos2).expect("logic error").num);
  println!("pos3 value: {}@{pos3}", numbers.get(pos3).expect("logic error").num);
  numbers.get(pos1).expect("logic error").num
    + numbers.get(pos2).expect("logic error").num
    + numbers.get(pos3).expect("logic error").num
}

fn load_numbers(filename: &str) -> VecD<NumberPosition> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut number_positions = VecD::new();
  for (pos, line) in contents.lines().enumerate() {
    number_positions.push(NumberPosition {
      num: line.parse::<i16>().unwrap(),
      pos: pos as i16,
    });
  }

  number_positions
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let mut numbers = load_numbers("input20.txt");
  let ct = numbers.len();
  println!("loaded: {ct}");
  mix_numbers(&mut numbers);
  let score = calc_score(&mut numbers);
  println!("score: {score}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sample_data() {
    let mut nums = load_numbers("testinput.txt");
    assert_eq!(nums.len(), 7);
    mix_numbers(&mut nums);
    let score = calc_score(&mut nums);
    assert_eq!(score, 3);
  }

  #[test]
  fn test_calc_score() {
    let mut nums = VecD::<NumberPosition>::new();
    nums.push(NumberPosition { num: 1, pos: 0 });
    nums.push(NumberPosition { num: 2, pos: 1 });
    nums.push(NumberPosition { num: -3, pos: 2 });
    nums.push(NumberPosition { num: 4, pos: 3 });
    nums.push(NumberPosition { num: 0, pos: 4 });
    nums.push(NumberPosition { num: 3, pos: 5 });
    nums.push(NumberPosition { num: -2, pos: 6 });
    let score = calc_score(&mut nums);
    assert_eq!(score, 3);
  }

  #[test]
  fn test_mix_1() {
    let mut nums = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: 2, pos: 1 },
      NumberPosition { num: -3, pos: 2 },
      NumberPosition { num: 3, pos: 3 },
      NumberPosition { num: -2, pos: 4 },
      NumberPosition { num: 0, pos: 5 },
      NumberPosition { num: 4, pos: 6 },
    ]);
    do_mix(&mut nums, 0);

    let nums2 = VecD(vec![
      NumberPosition { num: 2, pos: 0 },
      NumberPosition { num: 1, pos: 1 },
      NumberPosition { num: -3, pos: 2 },
      NumberPosition { num: 3, pos: 3 },
      NumberPosition { num: -2, pos: 4 },
      NumberPosition { num: 0, pos: 5 },
      NumberPosition { num: 4, pos: 6 },
    ]);

    nums.sort_by(|a, b| a.pos.cmp(&b.pos));
    assert_eq!(nums, nums2);
  }

  #[test]
  fn test_mix_2() {
    let mut nums = VecD(vec![
      NumberPosition { num: 2, pos: 0 },
      NumberPosition { num: 1, pos: 1 },
      NumberPosition { num: -3, pos: 2 },
      NumberPosition { num: 3, pos: 3 },
      NumberPosition { num: -2, pos: 4 },
      NumberPosition { num: 0, pos: 5 },
      NumberPosition { num: 4, pos: 6 },
    ]);
    do_mix(&mut nums, 0);

    let nums2 = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: -3, pos: 1 },
      NumberPosition { num: 2, pos: 2 },
      NumberPosition { num: 3, pos: 3 },
      NumberPosition { num: -2, pos: 4 },
      NumberPosition { num: 0, pos: 5 },
      NumberPosition { num: 4, pos: 6 },
    ]);

    nums.sort_by(|a, b| a.pos.cmp(&b.pos));
    assert_eq!(nums, nums2);
  }

  #[test]
  fn test_mix_3() {
    let mut nums = VecD::<NumberPosition>::new();
    nums.push(NumberPosition { num: 1, pos: 0 });
    nums.push(NumberPosition { num: -3, pos: 1 });
    nums.push(NumberPosition { num: 2, pos: 2 });
    nums.push(NumberPosition { num: 3, pos: 3 });
    nums.push(NumberPosition { num: -2, pos: 4 });
    nums.push(NumberPosition { num: 0, pos: 5 });
    nums.push(NumberPosition { num: 4, pos: 6 });
    do_mix(&mut nums, 1);

    let mut nums2 = VecD::<NumberPosition>::new();
    nums2.push(NumberPosition { num: 1, pos: 0 });
    nums2.push(NumberPosition { num: 2, pos: 1 });
    nums2.push(NumberPosition { num: 3, pos: 2 });
    nums2.push(NumberPosition { num: -2, pos: 3 });
    nums2.push(NumberPosition { num: -3, pos: 4 });
    nums2.push(NumberPosition { num: 0, pos: 5 });
    nums2.push(NumberPosition { num: 4, pos: 6 });

    nums.sort_by(|a, b| a.pos.cmp(&b.pos));
    assert_eq!(nums, nums2);
  }

  #[test]
  fn test_mix_4() {
    let mut nums = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: 2, pos: 1 },
      NumberPosition { num: 3, pos: 2 },
      NumberPosition { num: -2, pos: 3 },
      NumberPosition { num: -3, pos: 4 },
      NumberPosition { num: 0, pos: 5 },
      NumberPosition { num: 4, pos: 6 },
    ]);
    do_mix(&mut nums, 2);

    let nums2 = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: 2, pos: 1 },
      NumberPosition { num: -2, pos: 2 },
      NumberPosition { num: -3, pos: 3 },
      NumberPosition { num: 0, pos: 4 },
      NumberPosition { num: 3, pos: 5 },
      NumberPosition { num: 4, pos: 6 },
    ]);

    nums.sort_by(|a, b| a.pos.cmp(&b.pos));
    assert_eq!(nums, nums2);
  }

  #[test]
  fn test_mix_5() {
    let mut nums = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: 2, pos: 1 },
      NumberPosition { num: -2, pos: 2 },
      NumberPosition { num: -3, pos: 3 },
      NumberPosition { num: 0, pos: 4 },
      NumberPosition { num: 3, pos: 5 },
      NumberPosition { num: 4, pos: 6 },
    ]);
    do_mix(&mut nums, 2);

    let nums2 = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: 2, pos: 1 },
      NumberPosition { num: -3, pos: 2 },
      NumberPosition { num: 0, pos: 3 },
      NumberPosition { num: 3, pos: 4 },
      NumberPosition { num: 4, pos: 5 },
      NumberPosition { num: -2, pos: 6 },
    ]);

    nums.sort_by(|a, b| a.pos.cmp(&b.pos));
    assert_eq!(nums, nums2);
  }

  #[test]
  fn test_mix_6() {
    let mut nums = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: 2, pos: 1 },
      NumberPosition { num: -3, pos: 2 },
      NumberPosition { num: 0, pos: 3 },
      NumberPosition { num: 3, pos: 4 },
      NumberPosition { num: 4, pos: 5 },
      NumberPosition { num: -2, pos: 6 },
    ]);
    do_mix(&mut nums, 3);

    let nums2 = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: 2, pos: 1 },
      NumberPosition { num: -3, pos: 2 },
      NumberPosition { num: 0, pos: 3 },
      NumberPosition { num: 3, pos: 4 },
      NumberPosition { num: 4, pos: 5 },
      NumberPosition { num: -2, pos: 6 },
    ]);

    nums.sort_by(|a, b| a.pos.cmp(&b.pos));
    assert_eq!(nums, nums2);
  }

  #[test]
  fn test_mix_7() {
    let mut nums = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: 2, pos: 1 },
      NumberPosition { num: -3, pos: 2 },
      NumberPosition { num: 0, pos: 3 },
      NumberPosition { num: 3, pos: 4 },
      NumberPosition { num: 4, pos: 5 },
      NumberPosition { num: -2, pos: 6 },
    ]);
    do_mix(&mut nums, 5);

    let nums2 = VecD(vec![
      NumberPosition { num: 1, pos: 0 },
      NumberPosition { num: 2, pos: 1 },
      NumberPosition { num: -3, pos: 2 },
      NumberPosition { num: 4, pos: 3 },
      NumberPosition { num: 0, pos: 4 },
      NumberPosition { num: 3, pos: 5 },
      NumberPosition { num: -2, pos: 6 },
    ]);

    nums.sort_by(|a, b| a.pos.cmp(&b.pos));
    assert_eq!(nums, nums2);
  }
}
