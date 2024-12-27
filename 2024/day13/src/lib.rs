use std::fs::File;
use std::io::Read;

#[allow(unused_imports)]
use advent::{Dims, Direction, Point, TerrainMap};

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

//#[allow(unused_imports)]
//use priority_queue::PriorityQueue;
#[allow(unused_imports)]
use std::cmp::Reverse;

#[allow(unused_imports)]
use enum_iterator::{all, Sequence};

#[allow(unused_imports)]
use std::sync::OnceLock;

use regex::Regex;

#[allow(unused_imports)]
use nalgebra::{matrix, Dim, Matrix, RawStorageMut, Scalar};
use num::Zero;

pub fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

enum ParseState {
  Start,
  ButtonA,
  ButtonB,
  Prize,
}
pub fn analyse_input1(puzzle_input: &str) -> usize {
  let mut total = 0;

  let ra = Regex::new(r"^Button A: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
  let rb = Regex::new(r"^Button B: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
  let rp = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)$").unwrap();

  let mut p = ParseState::Start;
  let mut a = Point { x: 0, y: 0 };
  let mut b = Point { x: 0, y: 0 };
  let mut prize = Point { x: 0, y: 0 };
  for line in puzzle_input.lines() {
    match p {
      ParseState::Start => {
        let c = ra.captures(line).expect("Button A details");
        if c.len() != 3 {
          panic!("bad match of a)");
        };
        a.x = c.get(1).unwrap().as_str().parse::<isize>().unwrap();
        a.y = c.get(2).unwrap().as_str().parse::<isize>().unwrap();
        p = ParseState::ButtonA;
      }
      ParseState::ButtonA => {
        let c = rb.captures(line).expect("Button B details");
        if c.len() != 3 {
          panic!("bad match of b)");
        };
        b.x = c.get(1).unwrap().as_str().parse::<isize>().unwrap();
        b.y = c.get(2).unwrap().as_str().parse::<isize>().unwrap();
        p = ParseState::ButtonB;
      }
      ParseState::ButtonB => {
        let c = rp.captures(line).expect("Prize details");
        if c.len() != 3 {
          panic!("bad match of prize)");
        };
        prize.x = c.get(1).unwrap().as_str().parse::<isize>().unwrap();
        prize.y = c.get(2).unwrap().as_str().parse::<isize>().unwrap();
        p = ParseState::Prize;
        total += calc_prize_cost_ge(a, b, prize);
      }
      ParseState::Prize => {
        if !line.is_empty() {
          panic!("non blank line");
        };
        p = ParseState::Start;
      }
    };
  }

  total
}

fn calc_prize_cost_ge(a: Point, b: Point, prize: Point) -> usize {
  // j presses of A and k presses of B
  // jAx + kBx = Px
  // jAy + kBy = Py
  // how do I find out if there is a non-negative integer solution of j and k?
  // Simply solve the linear system
  // Gaussian Elimination
  // end up with division that may or may not have a remainder

  /*
  let mut aug = Vec::<Vec<isize>>::new();
  aug.push(vec![a.x, b.x, prize.x]);
  aug.push(vec![a.y, b.y, prize.y]);

  if aug[1][0] != 0 {
    let l = advent::lcm2(&[aug[0][0], aug[1][0]]);
    if l != aug[1][0] {}
  };*/

  let mut aug = matrix![a.x, b.x, prize.x;
                        a.y, b.y, prize.y];

  // println!("Original matrix: {aug}");
  for r in 1..aug.nrows() {
    // 1) if row has fewer leading zeros than parent, swap with parent(s)
    swap_rows(&mut aug, r);

    // 2) if row has more leading zeros than parent, move to next row
    if aug.row(r).iter().take_while(|&x| *x == 0).count() > aug.row(r - 1).iter().take_while(|&x| *x == 0).count() {
      continue;
    }
    // 3a) if row has equal leading zeros to parent, then multiply rows such that parent can be subtracted to eliminate leading column
    // it's correct to multiply each row by the leading value of the other. don't always need to do this. and there could be a risk of exceeding int max on a big matrix
    // could make a copy of parent row and multiply the copy. or could just multiply parent row in place

    let p1 = *aug.row(r - 1).iter().find(|&x| *x != 0).expect("all zeros in row");

    let r1 = *aug.row(r).iter().find(|&x| *x != 0).expect("all zeros in row");

    let prow = aug.row(r - 1) * r1;
    let mut rrow = aug.row_mut(r);
    //println!("original row: {rrow}");
    rrow *= p1;

    rrow -= prow;
    //println!("adjusted row: {rrow}");

    // 3b) subtract parent row such that leading non-zero is eliminated
  }
  //println!("Upper triangular matrix: {aug}");

  // if a prize
  if aug[(1, 2)] % aug[(1, 1)] != 0 {
    println!("impossible");
    return 0;
  }
  let b_presses = aug[(1, 2)] / aug[(1, 1)];

  // 4) substitute and find a
  let p1 = aug[(0, 1)];
  let r1 = aug[(1, 1)];
  let rrow = aug.row(1) * p1;
  let mut prow = aug.row_mut(0);
  prow *= r1;
  prow -= rrow;

  // println!("Matrix after also solving for a: {aug}");
  if aug[(0, 2)] % aug[(0, 0)] != 0 {
    println!("impossible2");
    return 0;
  }
  let a_presses = aug[(0, 2)] / aug[(0, 0)];

  println!("a presses: {a_presses}, b_presses: {b_presses}");
  (a_presses * 3 + b_presses) as usize
}

fn swap_rows<T, R, C, S>(matrix: &mut Matrix<T, R, C, S>, row: usize)
where
  T: Scalar + Zero,
  S: RawStorageMut<T, R, C>,
  R: Dim,
  C: Dim,
{
  let mut row = row;
  while row > 0 {
    let parent = row - 1;
    let plz = matrix.row(parent).iter().take_while(|&x| *x == T::zero()).count();
    let rlz = matrix.row(row).iter().take_while(|&x| *x == T::zero()).count();
    if plz > rlz {
      matrix.swap_rows(parent, row);
      row = parent;
    } else {
      return;
    }
  }
}

pub fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;

  let ra = Regex::new(r"^Button A: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
  let rb = Regex::new(r"^Button B: X\+([0-9]+), Y\+([0-9]+)$").unwrap();
  let rp = Regex::new(r"^Prize: X=([0-9]+), Y=([0-9]+)$").unwrap();

  let mut p = ParseState::Start;
  let mut a = Point { x: 0, y: 0 };
  let mut b = Point { x: 0, y: 0 };
  let mut prize = Point { x: 0, y: 0 };
  for line in puzzle_input.lines() {
    match p {
      ParseState::Start => {
        let c = ra.captures(line).expect("Button A details");
        if c.len() != 3 {
          panic!("bad match of a)");
        };
        a.x = c.get(1).unwrap().as_str().parse::<isize>().unwrap();
        a.y = c.get(2).unwrap().as_str().parse::<isize>().unwrap();
        p = ParseState::ButtonA;
      }
      ParseState::ButtonA => {
        let c = rb.captures(line).expect("Button B details");
        if c.len() != 3 {
          panic!("bad match of b)");
        };
        b.x = c.get(1).unwrap().as_str().parse::<isize>().unwrap();
        b.y = c.get(2).unwrap().as_str().parse::<isize>().unwrap();
        p = ParseState::ButtonB;
      }
      ParseState::ButtonB => {
        let c = rp.captures(line).expect("Prize details");
        if c.len() != 3 {
          panic!("bad match of prize)");
        };
        prize.x = c.get(1).unwrap().as_str().parse::<isize>().unwrap() + 10000000000000;
        prize.y = c.get(2).unwrap().as_str().parse::<isize>().unwrap() + 10000000000000;
        p = ParseState::Prize;
        total += calc_prize_cost_ge(a, b, prize);
      }
      ParseState::Prize => {
        if !line.is_empty() {
          panic!("non blank line");
        };
        p = ParseState::Start;
      }
    };
  }

  total
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 480);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 23);
  }
}
