
use std::fs::File;
use std::io::Read;
use clap::Parser;

use std::iter::zip;

//use std::collections::HashSet;
//use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

/// Day 3 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value_t=false)]
   benchmark: bool,
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_line() {
  }
  #[test]
  fn test_load1() {
    let (pairs, (ct, cd)) = load_data( "testinput.txt" );
    let score = analyse1(&pairs);
    assert_eq!(score, 288);
    assert_eq!(ct, 71530);
    assert_eq!(cd, 940200);

    let v = vec![(ct,cd)];
    let score2 = analyse1(&v);
    assert_eq!(score2, 71503 );
  }
}

fn analyse1( pairs: &Vec<(usize, usize)> ) -> usize {
  let mut score = 1;
  for (t,d) in pairs {
    println!("{t} {d}");

    let mut count = 0;

    for i in 0..*t {
      let travelled = i * (t-i);
      if travelled > *d { count += 1; }
    }
    score *= count;
  }
  score
}

fn load_data( filename: &str) -> (Vec<(usize,usize)>, (usize, usize))
{
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut it = contents.lines();
  let line1 = it.next().unwrap();
  let line2 = it.next().unwrap();

  let l1 = sscanf::sscanf_unescaped!(line1, "Time:{String}").unwrap();
  let l2 = sscanf::sscanf_unescaped!(line2, "Distance:{String}").unwrap();
  let mut times = Vec::new();
  let mut distances = Vec::new();

  for t in l1.split(' ') {
    if t.is_empty() { continue; }
    times.push( t.parse::<usize>().unwrap() );
  }
  for d in l2.split(' ') {
    if d.is_empty() { continue; }
    distances.push( d.parse::<usize>().unwrap() );
  }

  assert_eq!(times.len(), distances.len());
  let pairs:Vec<(usize,usize)> = zip(times, distances).collect();

  let combined_time = l1.replace(" ", "");
  let combined_dist = l2.replace(" ", "");

  let ct = combined_time.parse::<usize>().unwrap();
  let cd = combined_dist.parse::<usize>().unwrap();
      


  (pairs, (ct, cd))
}


fn main() {
    env_logger::init();

    let args = Args::parse();
    if args.benchmark {
      return;
    }

    let (pairs, (ct, cd)) = load_data( "input6.txt" );
    let score1 = analyse1(&pairs);
    println!("score1: {score1}");
    let v = vec![(ct,cd)];
    let score2 = analyse1(&v);
    println!("score2: {score2}");

}
