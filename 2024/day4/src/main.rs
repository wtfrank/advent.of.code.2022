#![feature(test)]
extern crate test;

use clap::Parser;
use std::cmp::PartialEq;
use std::fs::File;
use std::io::Read;
//use log::debug;

/// Day r of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

struct WordsearchIter<'a, T: Clone> {
  x: Option<usize>,
  y: Option<usize>,
  dx: isize,
  dy: isize,
  cw: &'a Wordsearch<T>,
}

impl<T: Clone> Iterator for WordsearchIter<'_, T> {
  type Item = T;
  fn next(&mut self) -> Option<Self::Item> {
    if self.x.is_none() || self.x.unwrap() >= self.cw.v[0].len() {
      return None;
    }
    if self.y.is_none() || self.y.unwrap() >= self.cw.v.len() {
      return None;
    }

    let x = self.x.unwrap();
    let y = self.y.unwrap();

    let ret = Some(self.cw.v[y][x].clone());

    let (next_x, overflow) = x.overflowing_add_signed(self.dx);
    if overflow {
      self.x = None;
    } else {
      self.x = Some(next_x);
    }

    let (next_y, overflow) = y.overflowing_add_signed(self.dy);
    if overflow {
      self.y = None;
    } else {
      self.y = Some(next_y)
    }

    ret
  }
}

#[derive(Debug)]
struct Wordsearch<T: Clone> {
  v: Vec<Vec<T>>,
}

impl<T: Clone> Wordsearch<T> {
  pub fn new() -> Self {
    Wordsearch {
      v: Vec::<Vec<T>>::new(),
    }
  }
  pub fn load_line(&mut self, line: Vec<T>) {
    if !self.v.is_empty() && line.len() != self.v[0].len() {
      panic!("line length didn't match previous")
    }
    self.v.push(line);
  }
  pub fn width(&self) -> usize {
    if !self.v.is_empty() {
      self.v[0].len()
    } else {
      0
    }
  }
  pub fn height(&self) -> usize {
    self.v.len()
  }
  pub fn at(&self, x: usize, y: usize) -> T {
    self.v[y][x].clone()
  }
  pub fn row_iter_fw(&self, row: usize) -> WordsearchIter<T> {
    WordsearchIter {
      x: if !self.v.is_empty() { Some(0) } else { None },
      y: if row >= self.v.len() { None } else { Some(row) },
      dx: 1,
      dy: 0,
      cw: self,
    }
  }
  pub fn row_iter_bk(&self, row: usize) -> WordsearchIter<T> {
    let (x, overflow) = if !self.v.is_empty() {
      self.v[0].len().overflowing_sub(1)
    } else {
      (0, true)
    };
    WordsearchIter {
      x: if overflow { None } else { Some(x) },
      y: if row >= self.v.len() { None } else { Some(row) },
      dx: -1,
      dy: 0,
      cw: self,
    }
  }
  pub fn col_iter_down(&self, col: usize) -> WordsearchIter<T> {
    WordsearchIter {
      x: if !self.v.is_empty() && self.v[0].len() > col {
        Some(col)
      } else {
        None
      },
      y: if !self.v.is_empty() { Some(0) } else { None },
      dx: 0,
      dy: 1,
      cw: self,
    }
  }
  pub fn col_iter_up(&self, col: usize) -> WordsearchIter<T> {
    let (y, overflow) = self.v.len().overflowing_sub(1);
    WordsearchIter {
      x: if !self.v.is_empty() && self.v[0].len() > col {
        Some(col)
      } else {
        None
      },
      y: if overflow { None } else { Some(y) },
      dx: 0,
      dy: -1,
      cw: self,
    }
  }

  // one of col & row must be zero. These represent the start of the diagonal.
  pub fn diag_iter_tl_br(&self, col: usize, row: usize) -> WordsearchIter<T> {
    let (x, y) = if self.v.is_empty()
      || self.v[0].is_empty()
      || (col != 0 && row != 0)
      || row >= self.v.len()
      || col >= self.v[0].len()
    {
      (None, None)
    } else {
      (Some(col), Some(row))
    };

    WordsearchIter {
      x,
      y,
      dx: 1,
      dy: 1,
      cw: self,
    }
  }

  // one of col & row must be equal to one less than the max length of row or col
  pub fn diag_iter_br_tl(&self, col: usize, row: usize) -> WordsearchIter<T> {
    let (x, y) = if self.v.is_empty()
      || self.v[0].is_empty()
      || (col != self.v[0].len() - 1 && row != self.v.len() - 1)
      || row >= self.v.len()
      || col >= self.v[0].len()
    {
      (None, None)
    } else {
      (Some(col), Some(row))
    };

    WordsearchIter {
      x,
      y,
      dx: -1,
      dy: -1,
      cw: self,
    }
  }

  // col must be equal to one less than the max length or row must be equal to 0
  pub fn diag_iter_tr_bl(&self, col: usize, row: usize) -> WordsearchIter<T> {
    let (x, y) = if self.v.is_empty()
      || self.v[0].is_empty()
      || (col != self.v[0].len() - 1 && row != 0)
      || row >= self.v.len()
      || col >= self.v[0].len()
    {
      (None, None)
    } else {
      (Some(col), Some(row))
    };

    WordsearchIter {
      x,
      y,
      dx: -1,
      dy: 1,
      cw: self,
    }
  }

  // col must be equal to zero or row must be one less than the max length
  pub fn diag_iter_bl_tr(&self, col: usize, row: usize) -> WordsearchIter<T> {
    let (x, y) = if self.v.is_empty()
      || self.v[0].is_empty()
      || (col != 0 && row != self.v.len() - 1)
      || row >= self.v.len()
      || col >= self.v[0].len()
    {
      (None, None)
    } else {
      (Some(col), Some(row))
    };

    WordsearchIter {
      x,
      y,
      dx: 1,
      dy: -1,
      cw: self,
    }
  }
}

fn count_matches<T: Clone + PartialEq>(mut iter: WordsearchIter<T>, search: &[T]) -> usize {
  let mut search_iter = search.iter();
  let mut count = 0;
  loop {
    let c = iter.next();
    if c.is_none() {
      // check if we ended at a match of the search string
      if search_iter.next().is_none() {
        count += 1;
      }
      break;
    }
    let mut s = search_iter.next();
    if s.is_none() {
      count += 1;
      search_iter = search.iter();
      s = search_iter.next();
    }

    if *c.as_ref().unwrap() != *s.unwrap() {
      // reset our match
      search_iter = search.iter();
      // maybe we match with the start of the search string though
      if c.unwrap() == search[0] {
        search_iter.next();
      }
    }
  }
  count
}
fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

fn analyse_input1(puzzle_input: &str) -> usize {
  let mut total = 0;
  let mut cw = Wordsearch::<char>::new();
  for line in puzzle_input.lines() {
    cw.load_line(line.chars().collect());
  }

  let cols = cw.v[0].len();
  let rows = cw.v.len();
  println!("{cols} cols, {rows} rows");

  /* for diagonals we split them into two sections */

  let search: Vec<char> = "XMAS".chars().collect();
  for x in 0..(cw.v[0].len()) {
    total += count_matches(cw.col_iter_down(x), &search);
    total += count_matches(cw.col_iter_up(x), &search);
    total += count_matches(cw.diag_iter_tl_br(x, 0), &search);
    total += count_matches(cw.diag_iter_tr_bl(x, 0), &search);
    total += count_matches(cw.diag_iter_br_tl(x, cw.v.len() - 1), &search);
    total += count_matches(cw.diag_iter_bl_tr(x, cw.v.len() - 1), &search);
  }
  for y in 0..(cw.v.len()) {
    total += count_matches(cw.row_iter_fw(y), &search);
    total += count_matches(cw.row_iter_bk(y), &search);
    if y != 0 {
      total += count_matches(cw.diag_iter_tl_br(0, y), &search);
      total += count_matches(cw.diag_iter_tr_bl(cw.v[0].len() - 1, y), &search);
    }
    if y != cw.v.len() - 1 {
      total += count_matches(cw.diag_iter_bl_tr(0, y), &search);
      total += count_matches(cw.diag_iter_br_tl(cw.v[0].len() - 1, y), &search);
    }
  }
  total
}

fn analyse_input2(puzzle_input: &str) -> usize {
  let mut total = 0;
  let mut cw = Wordsearch::<char>::new();
  for line in puzzle_input.lines() {
    cw.load_line(line.chars().collect());
  }

  for y in 1..cw.height() - 1 {
    for x in 1..cw.width() - 1 {
      let c = cw.at(x, y);
      if c != 'A' {
        continue;
      }
      let tl = cw.at(x - 1, y - 1);
      let br = cw.at(x + 1, y + 1);
      if !((tl == 'M' && br == 'S') || (tl == 'S' && br == 'M')) {
        continue;
      }

      let tr = cw.at(x + 1, y - 1);
      let bl = cw.at(x - 1, y + 1);
      if !((tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M')) {
        continue;
      }

      println!("found X-MAS at {x},{y}");
      total += 1;
    }
  }

  total
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let data = load_data("input4.txt");
  let answer1 = analyse_input1(&data);
  println!("answer: {answer1} (note: 2584 was too high)");
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
    assert_eq!(result, 18);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput1.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, 9);
  }

  #[test]
  fn test_cw_iter_empty() {
    let cw = Wordsearch::<char>::new();
    let mut i = cw.row_iter_fw(0);
    assert_eq!(i.next(), None);
    let mut i = cw.row_iter_bk(0);
    assert_eq!(i.next(), None);
    let mut i = cw.col_iter_down(0);
    assert_eq!(i.next(), None);
    let mut i = cw.col_iter_up(0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tl_br(0, 0);
    assert_eq!(i.next(), None);
    let mut i = cw.diag_iter_br_tl(0, 0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tr_bl(0, 0);
    assert_eq!(i.next(), None);
    let mut i = cw.diag_iter_bl_tr(0, 0);
    assert_eq!(i.next(), None);
  }

  #[test]
  fn test_cw_iter_zero_length_rows() {
    let mut cw = Wordsearch::new();
    cw.load_line("".chars().collect());
    let mut i = cw.row_iter_fw(0);
    assert_eq!(i.next(), None);
    let mut i = cw.row_iter_bk(0);
    assert_eq!(i.next(), None);
    let mut i = cw.col_iter_down(0);
    assert_eq!(i.next(), None);
    let mut i = cw.col_iter_up(0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tl_br(0, 0);
    assert_eq!(i.next(), None);
    let mut i = cw.diag_iter_br_tl(0, 0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tr_bl(0, 0);
    assert_eq!(i.next(), None);
    let mut i = cw.diag_iter_bl_tr(0, 0);
    assert_eq!(i.next(), None);
  }

  #[test]
  fn test_cw_iter_row_fw() {
    let mut cw = Wordsearch::new();
    cw.load_line("AB".chars().collect());
    cw.load_line("CD".chars().collect());
    let mut i = cw.row_iter_fw(0);
    assert_eq!(i.next(), Some('A'));
    assert_eq!(i.next(), Some('B'));
    assert_eq!(i.next(), None);
  }
  #[test]
  fn test_cw_iter_row_bk() {
    let mut cw = Wordsearch::new();
    cw.load_line("AB".chars().collect());
    cw.load_line("CD".chars().collect());
    let mut i = cw.row_iter_bk(1);
    assert_eq!(i.next(), Some('D'));
    assert_eq!(i.next(), Some('C'));
    assert_eq!(i.next(), None);
  }
  #[test]
  fn test_cw_iter_col_down() {
    let mut cw = Wordsearch::new();
    cw.load_line("AB".chars().collect());
    cw.load_line("CD".chars().collect());
    let mut i = cw.col_iter_down(1);
    assert_eq!(i.next(), Some('B'));
    assert_eq!(i.next(), Some('D'));
    assert_eq!(i.next(), None);
  }
  #[test]
  fn test_cw_iter_col_up() {
    let mut cw = Wordsearch::new();
    cw.load_line("AB".chars().collect());
    cw.load_line("CD".chars().collect());
    let mut i = cw.col_iter_up(0);
    assert_eq!(i.next(), Some('C'));
    assert_eq!(i.next(), Some('A'));
    assert_eq!(i.next(), None);
  }

  #[test]
  fn test_cw_iter_diag_tl_br() {
    let mut cw = Wordsearch::new();
    cw.load_line("AB".chars().collect());
    cw.load_line("CD".chars().collect());
    let mut i = cw.diag_iter_tl_br(0, 0);
    assert_eq!(i.next(), Some('A'));
    assert_eq!(i.next(), Some('D'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tl_br(0, 1);
    assert_eq!(i.next(), Some('C'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tl_br(1, 0);
    assert_eq!(i.next(), Some('B'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tl_br(1, 1);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tl_br(2, 0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tl_br(0, 2);
    assert_eq!(i.next(), None);
  }

  #[test]
  fn test_cw_iter_diag_br_tl() {
    let mut cw = Wordsearch::new();
    cw.load_line("AB".chars().collect());
    cw.load_line("CD".chars().collect());
    let mut i = cw.diag_iter_br_tl(1, 1);
    assert_eq!(i.next(), Some('D'));
    assert_eq!(i.next(), Some('A'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_br_tl(0, 1);
    assert_eq!(i.next(), Some('C'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_br_tl(1, 0);
    assert_eq!(i.next(), Some('B'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_br_tl(0, 0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_br_tl(2, 0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_br_tl(0, 2);
    assert_eq!(i.next(), None);
  }

  #[test]
  fn test_cw_iter_diag_tr_bl() {
    let mut cw = Wordsearch::new();
    cw.load_line("AB".chars().collect());
    cw.load_line("CD".chars().collect());
    let mut i = cw.diag_iter_tr_bl(1, 0);
    assert_eq!(i.next(), Some('B'));
    assert_eq!(i.next(), Some('C'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tr_bl(1, 1);
    assert_eq!(i.next(), Some('D'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tr_bl(0, 0);
    assert_eq!(i.next(), Some('A'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tr_bl(0, 1);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tr_bl(2, 0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_tr_bl(0, 2);
    assert_eq!(i.next(), None);
  }

  #[test]
  fn test_cw_iter_diag_bl_tr() {
    let mut cw = Wordsearch::new();
    cw.load_line("AB".chars().collect());
    cw.load_line("CD".chars().collect());
    let mut i = cw.diag_iter_bl_tr(0, 1);
    assert_eq!(i.next(), Some('C'));
    assert_eq!(i.next(), Some('B'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_bl_tr(1, 1);
    assert_eq!(i.next(), Some('D'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_bl_tr(0, 0);
    assert_eq!(i.next(), Some('A'));
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_bl_tr(1, 0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_bl_tr(2, 0);
    assert_eq!(i.next(), None);

    let mut i = cw.diag_iter_bl_tr(0, 2);
    assert_eq!(i.next(), None);
  }

  #[test]
  fn test_count_matches() {
    let mut cw = Wordsearch::new();
    cw.load_line("CDEFAABABABBCDCDEFGDEF".chars().collect());

    assert_eq!(
      count_matches(cw.row_iter_fw(0), &"AASDA".chars().collect::<Vec<char>>()),
      0
    );
    assert_eq!(
      count_matches(cw.row_iter_fw(0), &"ABAB".chars().collect::<Vec<char>>()),
      1
    );
    assert_eq!(
      count_matches(cw.row_iter_fw(0), &"AB".chars().collect::<Vec<char>>()),
      3
    );
    assert_eq!(
      count_matches(cw.row_iter_fw(0), &"DEF".chars().collect::<Vec<char>>()),
      3
    );
    assert_eq!(
      count_matches(cw.row_iter_fw(0), &"CDEF".chars().collect::<Vec<char>>()),
      2
    );
  }

  #[test]
  fn test_count_matches2() {
    let mut file = File::open("testinput1.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let mut cw = Wordsearch::<char>::new();
    for line in contents.lines() {
      cw.load_line(line.chars().collect());
    }

    let cols = cw.v[0].len();
    let rows = cw.v.len();
    println!("{cols} cols, {rows} rows");

    let search: Vec<char> = "XMAS".chars().collect();

    let mut total = 0;
    for x in 0..(cw.v[0].len()) {
      total += count_matches(cw.col_iter_down(x), &search);
    }
    assert_eq!(total, 1);

    let mut total = 0;
    for x in 0..(cw.v[0].len()) {
      total += count_matches(cw.col_iter_up(x), &search);
    }
    assert_eq!(total, 2);

    let mut total = 0;
    for x in 0..(cw.v[0].len()) {
      total += count_matches(cw.diag_iter_tl_br(x, 0), &search);
    }
    assert_eq!(total, 1);

    let mut total = 0;
    for x in 0..(cw.v[0].len()) {
      total += count_matches(cw.diag_iter_tr_bl(x, 0), &search);
    }
    assert_eq!(total, 0);

    let mut total = 0;
    for x in 0..(cw.v[0].len()) {
      total += count_matches(cw.diag_iter_br_tl(x, cw.v.len() - 1), &search);
    }
    assert_eq!(total, 3);

    let mut total = 0;
    for x in 0..(cw.v[0].len()) {
      total += count_matches(cw.diag_iter_bl_tr(x, cw.v.len() - 1), &search);
    }
    assert_eq!(total, 3);

    let mut total = 0;
    for y in 0..(cw.v.len()) {
      total += count_matches(cw.row_iter_fw(y), &search);
    }
    assert_eq!(total, 3);

    let mut total = 0;
    for y in 0..(cw.v.len()) {
      total += count_matches(cw.row_iter_bk(y), &search);
    }
    assert_eq!(total, 2);

    let mut total = 0;
    for y in 1..(cw.v.len()) {
      total += count_matches(cw.diag_iter_tl_br(0, y), &search);
    }
    assert_eq!(total, 0);

    let mut total = 0;
    for y in 0..(cw.v.len() - 1) {
      total += count_matches(cw.diag_iter_bl_tr(0, y), &search);
    }
    assert_eq!(total, 1);

    let mut total = 0;
    for y in 0..(cw.v.len() - 1) {
      let count = count_matches(cw.diag_iter_tr_bl(cw.v[0].len() - 1, y), &search);
      total += count;
      if count > 0 {
        //let row: Vec<char> = cw.diag_iter_tr_bl(cw.v[0].len() - 1, y).collect();
        //println!("count: {count}: {row:?}");
      }
    }
    assert_eq!(total, 1);

    let mut total = 0;
    for y in 1..(cw.v.len()) {
      let count = count_matches(cw.diag_iter_br_tl(cw.v[0].len() - 1, y), &search);
      total += count;
      if count > 0 {
        let row: Vec<char> = cw.diag_iter_br_tl(cw.v[0].len() - 1, y).collect();
        println!("y: {y}, count: {count}. {row:?}");
      }
    }
    assert_eq!(total, 2);
  }

  use test::{black_box, Bencher};

  #[bench]
  fn bench_part1(b: &mut Bencher) {
    let data = load_data("testinput1.txt");
    b.iter(|| black_box(analyse_input1(&data)));
  }

  #[bench]
  fn bench_part2(b: &mut Bencher) {
    let data = load_data("testinput1.txt");
    b.iter(|| black_box(analyse_input2(&data)));
  }
}
