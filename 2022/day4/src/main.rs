use std::fmt;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Range {
  start: u32,
  end: u32,
}

impl fmt::Display for Range {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Write strictly the first element into the supplied output
    // stream: `f`. Returns `fmt::Result` which indicates whether the
    // operation succeeded or failed. Note that `write!` uses syntax which
    // is very similar to `println!`.
    write!(f, "[{}-{}]", self.start, self.end)
  }
}

impl Range {
  fn contains(&self, r: &Range) -> bool {
    r.start >= self.start && r.start <= self.end && r.end >= self.start && r.end <= self.end
  }
  fn overlaps(&self, r: &Range) -> bool {
    (r.start >= self.start && r.start <= self.end) || (r.end >= self.start && r.end <= self.end)
  }
}

fn parse_range(range: &str) -> Range {
  let a: Vec<&str> = range.split('-').collect();
  assert!(a.len() == 2);
  Range {
    start: a[0].parse::<u32>().expect("invalid input"),
    end: a[1].parse::<u32>().expect("numeric input"),
  }
}

fn parse_assignments(assignment: &str) -> (Range, Range) {
  let assignments: Vec<&str> = assignment.split(',').collect();
  assert!(assignments.len() == 2);
  (parse_range(assignments[0]), parse_range(assignments[1]))
}

fn main() -> std::io::Result<()> {
  let mut file = File::open("input.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let mut score: u32 = 0;
  let mut score2: u32 = 0;
  for line in contents.lines() {
    let (a1, a2) = parse_assignments(line);

    //  if (a1.start >= a2.start && a1.end <= a2.end) ||
    //     (a1.start <= a2.start && a1.end >= a2.end) {
    if a1.contains(&a2) || a2.contains(&a1) {
      score += 1;
      println!("ranges contained: {a1}, {a2}");
    }
    if a1.overlaps(&a2) || a2.overlaps(&a1) {
      score2 += 1;
      println!("ranges overlapped: {a1}, {a2}");
    }
  }
  println!("score: {score}");
  println!("score2: {score2}");

  Ok(())
}
