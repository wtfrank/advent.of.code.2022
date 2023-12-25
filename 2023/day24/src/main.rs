use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::io::Write; //for flush

//use std::fmt;
//use std::str::FromStr;
use std::fmt::Write;

//use advent::{determine_map_dims, Direction, Point, TerrainMap};
//use advent::{Interval, Point3};

//use enum_iterator::all;
//use enum_iterator::{all,Sequence};

//use priority_queue::PriorityQueue;
//use std::cmp::Reverse;
//use std::cmp::{max,Reverse,Ordering};
//use std::cmp::Ordering;
//use std::collections::HashMap;
//use std::collections::HashSet;
//use std::collections::VecDeque;

//use std::iter::zip;

//use std::collections::HashSet;
//use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

//use std::collections::HashMap;

//use advent::{prime_factors, lcm};
//use advent::Range;

use pyo3::{prelude::*, types::PyModule};

/// Day 24 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn analyse_data2(hailstones: &[(Position, Velocity)]) -> usize {
  // a big system of simultaneous equations
  // for each hailstone (path p + r.v) the path of the thing we throw (pt + rt.vt) will have some r=rt where the positions are equal
  //
  // there are 6 unknowns - p/v of the projectile. but for each hailstone there is also the time it meets.
  // Does this mean that we solve it after merely 6 hailstones? actually 2 cos of coords?

  // p1x + r1.v1x = ptx + r1.vtx
  // ..
  // p1z + r1.v1z = ptz + r1.vtz
  //
  // p2x + r2.v2x = ptx + r2.vtx
  //
  // p3x + r3.v3x = ptx + r3.vtx
  //
  // p1x - ptx = r1(v1x - vtx)

  for (idx, hs) in hailstones.iter().enumerate() {
    println!("{} + r_{idx}*{} = ptx + r_{idx}*vtx", hs.0 .0, hs.1 .0);
    println!("{} + r_{idx}*{} = pty + r_{idx}*vty", hs.0 .1, hs.1 .1);
    println!("{} + r_{idx}*{} = ptz + r_{idx}*vtz", hs.0 .2, hs.1 .2);
  }

  let mut py = String::new();
  writeln!(&mut py, "#!/usr/bin/env python3").unwrap();
  writeln!(&mut py, "import sympy as sym\n").unwrap();
  writeln!(&mut py, "def solve():").unwrap();
  writeln!(
    &mut py,
    "  ptx, pty, ptz, vtx, vty, vtz = sym.symbols('ptx,pty,ptz,vtx,vty,vtz')"
  )
  .unwrap();

  let mut eq_names = Vec::new();
  let mut param_names = Vec::new();
  let mut eq_count = 0;
  for (idx, hs) in hailstones.iter().enumerate() {
    if eq_count >= 9 {
      break;
    }

    param_names.push(format!("r_{idx}"));
    writeln!(&mut py, "  r_{idx} = sym.symbols('r_{idx}')").unwrap();
    eq_names.push(format!("eq{eq_count}"));
    writeln!(
      &mut py,
      "  eq{eq_count} = sym.Eq({} + r_{idx}*{}, ptx + r_{idx}*vtx)",
      hs.0 .0, hs.1 .0
    )
    .unwrap();

    eq_count += 1;
    eq_names.push(format!("eq{eq_count}"));
    writeln!(
      &mut py,
      "  eq{eq_count} = sym.Eq({} + r_{idx}*{}, pty + r_{idx}*vty)",
      hs.0 .1, hs.1 .1
    )
    .unwrap();
    eq_count += 1;
    eq_names.push(format!("eq{eq_count}"));
    writeln!(
      &mut py,
      "  eq{eq_count} = sym.Eq({} + r_{idx}*{}, ptz + r_{idx}*vtz)",
      hs.0 .2, hs.1 .2
    )
    .unwrap();
    eq_count += 1;
  }

  let eq_str = eq_names.join(",");
  let params_str = param_names.join(",");

  writeln!(
    &mut py,
    "  result = sym.solve([{eq_str}],(ptx, pty, ptz, vtx, vty, vtz, {params_str}))"
  )
  .unwrap();
  writeln!(&mut py, "  return result").unwrap();

  let code_loc = String::from("/tmp/gen_sym_eq.py");
  println!("python code written to {code_loc}");
  std::fs::write(code_loc, &py).unwrap();

  execute_py(&py)
}

type SolnParams = (isize, isize, isize, isize, isize, isize, isize, isize, isize);

fn execute_py(py: &str) -> usize {
  let mut result = 0;
  Python::with_gil(|p| {
    let simeq = PyModule::from_code(p, py, "simeq.py", "simeq").unwrap();
    //let res: (isize, isize, isize, isize, isize, isize, isize, isize, isize) =
    let res: Vec<SolnParams> = simeq.getattr("solve").unwrap().call0().unwrap().extract().unwrap();
    println!("result: {res:?}");
    let r = res[0];
    result = (r.0 + r.1 + r.2) as usize;
  });
  result
}

fn analyse_data(hailstones: &Vec<(Position, Velocity)>, min: usize, max: usize) -> usize {
  let mut count = 0;
  for i in 0..hailstones.len() {
    let hs1 = &hailstones[i];
    for (j, hs2) in hailstones.iter().enumerate().skip(i + 1) {
      if hs2.1 .0 / hs1.1 .0 == hs2.1 .1 / hs1.1 .1 {
        println!("parallel {i} {j}");
        //continue;
      }

      //look for paths crossing
      //(x,y) = (mx1, my1)*t + (bx1, by1)
      //equation of line is hs1.0 + r.hs1.1 and hs2.0 + s.hs2.1
      //set equal: hs1.0 + r.hs1.1 = hs2.0 + s.hs2.1
      //
      // or explicitly
      // 1) hs1.0.0 + r hs1.1.0 = hs2.0.0 + s hs2.1.0
      // 2) hs1.0.1 + r hs1.1.1 = hs2.0.1 + s hs2.1.1
      //
      // r hs1.1 - s hs2.1 = hs2.0 - hs 1.0
      //
      // 1) r hs1.1.0 - s hs2.1.0 = hs2.0.0 - hs1.0.0
      // 2) r hs1.1.1 - s hs2.1.1 = hs2.0.1 - hs1.0.1
      //
      // | hs1.1.0, -hs2.1.0 | * r = hs2.0.0 - hs1.0.0
      // | hs1.1.1, -hs2.1.1 |   s   hs2.0.1 - hs1.0.1
      //
      //
      //
      //matrix form:
      //mx1-mx2 * r = bx2-bx1
      //my1-my2   s   by2-by1
      //
      //that eqn is of form Ax=b, with x = s,t
      //x = A^-1 * b
      //
      //matrix inverse:
      //a,b ^-1 = 1/(ad-bc) * d,-b
      //c,d                   -c,a
      //

      //check determinant

      let det: f64 = ((hs1.1 .0 * -hs2.1 .1) - (hs2.1 .0 * -hs1.1 .1)) as f64;

      if det == 0.0 {
        println!("det = 0");
        continue;
      }

      //r = 1/det *  (my2 * bx2-bx1) + (mx2 * by2-by1)

      //r = 1/det * (d*bx + -b *by)
      //            (-hs2.1.1 * (hs2.0.0 - hs1.0.0) + hs2.1.0 * (hs2.0.1-hs1.0.1)
      //s = 1/det * -c*bx + a *by

      let r = (-hs2.1 .1 * (hs2.0 .0 - hs1.0 .0) + hs2.1 .0 * (hs2.0 .1 - hs1.0 .1)) as f64 / det;
      let s = (-hs1.1 .1 * (hs2.0 .0 - hs1.0 .0) + hs1.1 .0 * (hs2.0 .1 - hs1.0 .1)) as f64 / det;
      let rx = r * hs1.1 .0 as f64 + hs1.0 .0 as f64;
      let ry = r * hs1.1 .1 as f64 + hs1.0 .1 as f64;
      println!("intersect at {rx}, {ry} (r = {r}, s={s})");
      if r > 0.0 && s > 0.0 && rx >= min as f64 && ry >= min as f64 && rx <= max as f64 && ry <= max as f64 {
        count += 1;
      }
    }
  }

  count
}

struct Position(isize, isize, isize);
struct Velocity(isize, isize, isize);

fn load_data(filename: &str) -> Vec<(Position, Velocity)> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut hailstones = Vec::new();

  for line in contents.lines() {
    let r = sscanf::sscanf!(line, "{isize}, {isize}, {isize} @ {isize}, {isize}, {isize}").unwrap();
    hailstones.push((Position(r.0, r.1, r.2), Velocity(r.3, r.4, r.5)));
  }

  hailstones
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
  pyo3::prepare_freethreaded_python();

  let data = load_data("input24.txt");
  let score1 = analyse_data(&data, 200000000000000, 400000000000000);
  println!("score1: {score1}");
  let score2 = analyse_data2(&data);
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let data = load_data("testinput.txt");
    let score1 = analyse_data(&data, 7, 27);
    assert_eq!(score1, 2);
  }

  #[test]
  fn test_load2() {
    pyo3::prepare_freethreaded_python();
    let data = load_data("testinput.txt");
    let score1 = analyse_data2(&data);
    assert_eq!(score1, 47);
  }
}
