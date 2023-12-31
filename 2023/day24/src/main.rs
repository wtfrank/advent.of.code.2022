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

extern crate nalgebra as na;

use pyo3::{prelude::*, types::PyModule};

/// Day 24 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

/* solves simultaneous equations like analyse_data2, but via matrix operations in rust, instead of via generating python code for symobolic algebra with sympy */
#[allow(dead_code)]
fn analyse_data2b(hailstones: &[(Position, Velocity)]) -> usize {
  //9 unknown variables
  //
  // ptx, pty, ptz, vtx/y/z, r1/2/3
  //
  // p1x + r1.v1x = ptx + r1.vtx
  //
  // have to rewrite the above equation in linear form
  // the problem is that r1.vtz is two unknown variables multiplied by each other i.e. not linear form
  //
  // treat the two unknowns multiplied non-linearly, as a single variable. This means more unknowns.
  //
  //
  // each eqn rearranged to form
  // ptx + r1.vtx - r1.v1x = p1x
  //
  // so we have Ax=b
  // where x is this seq of unknowns: ptx, pty, ptz, r1, r2, r3, r1.vtx, r1.vty, r1.vtz, r2.vtx, r2.vty, r2.vtz, r3.vtx, r3.vty, r3.vtz
  // vtx/vty/vtz don't appear in the seq, but we can derive them from the compound terms
  //
  //
  let mut m = na::SMatrix::<f64, 15, 15>::repeat(0_f64);
  let mut b = na::SMatrix::<f64, 15, 1>::repeat(0_f64);

  let mut eqn = 0;
  for (idx, hs) in hailstones.iter().enumerate().take(3) {
    //ptx + rN.vtx - rN.vNx = pNx
    m[(eqn, 0)] = 1_f64;
    m[(eqn, eqn + 6)] = 1_f64;
    m[(eqn, 3 + idx)] = -hs.1 .0 as f64;
    b[(eqn, 0)] = hs.0 .0 as f64;
    eqn += 1;
    //pty + rN.vty - rN.vNy = pNy
    m[(eqn, 1)] = 1_f64;
    m[(eqn, eqn + 6)] = 1_f64;
    m[(eqn, 3 + idx)] = -hs.1 .1 as f64;
    b[(eqn, 0)] = hs.0 .1 as f64;
    eqn += 1;
    //ptz + rN.vtz - rN.vNz = pNz
    m[(eqn, 2)] = 1_f64;
    m[(eqn, eqn + 6)] = 1_f64;
    m[(eqn, 3 + idx)] = -hs.1 .2 as f64;
    b[(eqn, 0)] = hs.0 .2 as f64;

    eqn += 1;
  }

  println!("A matrix: {m}");
  println!("b vector: {b}");

  let det = m.determinant();
  println!("det: {det}");
  /*
  let decomp = m.lu();
  println!("decomp: {decomp}");
  */

  //we discover that as a square matrix there are zero-filled rows such that the determinanant is zero,
  //and as a non-square matrix, we can't solve it via the technique.
  //Fundamental issue is that this is a system of polynomial equations not a system of linear equations

  0
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

/* a version of analyse_data that uses nalgebra Matrix structures, instead of my manually
 * written matrix calculation
 */
#[allow(dead_code)]
fn analyse_data1a(hailstones: &Vec<(Position, Velocity)>, min: usize, max: usize) -> usize {
  let mut count = 0;
  for i in 0..hailstones.len() {
    let hs1 = &hailstones[i];
    for (_j, hs2) in hailstones.iter().enumerate().skip(i + 1) {
      /* we don't care what time the 2d paths intercept, just whether they intercept at all.
       * So we set up a pair of simultaneous linear equations.
       * each hailstone has P and V.
       *
       * 1) p1x + r.v1x = p2x + s.v2x
       * 2) p1y + r.v1y = p2y + s.v2y
       *
       * rearrange 1)
       * r.v1x - s.v2x = p2x - p1x
       * rearrange 2)
       * r.v1y - s.v2y = p2y - p1y
       *
       * Ax = b
       */

      let mut a = na::Matrix2::<isize>::repeat(0);
      let mut b = na::Vector2::<isize>::repeat(0);

      a[(0, 0)] = hs1.1 .0;
      a[(0, 1)] = -hs2.1 .0;
      a[(1, 0)] = hs1.1 .1;
      a[(1, 1)] = -hs2.1 .1;

      b[0] = hs2.0 .0 - hs1.0 .0;
      b[1] = hs2.0 .1 - hs1.0 .1;

      //let det = a.determinant();
      let det = a[(0, 0)] * a[(1, 1)] - a[(0, 1)] * a[(1, 0)];
      println!("determinant: {det}");
      if det == 0 {
        println!("parallel");
        continue;
      }

      //A-1b = x

      // let inv = a.adjoint(); //adjoint is conjugate-transpose, not adjudgate/classical adjoint.
      let mut adj = na::Matrix2::<f64>::repeat(0_f64);
      adj[(0, 0)] = a[(1, 1)] as f64;
      adj[(0, 1)] = -a[(0, 1)] as f64;
      adj[(1, 0)] = -a[(1, 0)] as f64;
      adj[(1, 1)] = a[(0, 0)] as f64;

      println!("adj: {adj}");

      //let x = adj * na::Vector2::<f64>::from_fn(|i, j| b[(i, j)] as f64) / det as f64;
      let x = adj * b.cast::<f64>() / det as f64;
      let r = x[0];
      let s = x[1];
      println!("{r} {s}");
      if r < 0_f64 || s < 0_f64 {
        continue;
      }

      let x = hs1.0 .0 as f64 + r * hs1.1 .0 as f64;
      let y = hs1.0 .1 as f64 + r * hs1.1 .1 as f64;
      if x >= min as f64 && y >= min as f64 && x <= max as f64 && y <= max as f64 {
        count += 1;
      }
    }
  }

  count
}

/* 1a but uses other types to get the most elegant calculation */
#[allow(dead_code)]
fn analyse_data1b(hailstones: &Vec<(Position, Velocity)>, min: usize, max: usize) -> usize {
  let mut count = 0;
  for i in 0..hailstones.len() {
    let hs1 = &hailstones[i];
    for (_j, hs2) in hailstones.iter().enumerate().skip(i + 1) {
      let mut a = na::Matrix2::<f64>::repeat(0_f64);
      let mut b = na::Vector2::<f64>::repeat(0_f64);

      a[(0, 0)] = hs1.1 .0 as f64;
      a[(0, 1)] = -hs2.1 .0 as f64;
      a[(1, 0)] = hs1.1 .1 as f64;
      a[(1, 1)] = -hs2.1 .1 as f64;

      b[0] = hs2.0 .0 as f64 - hs1.0 .0 as f64;
      b[1] = hs2.0 .1 as f64 - hs1.0 .1 as f64;

      let det = a.determinant();
      println!("determinant: {det}");
      if det == 0_f64 {
        println!("parallel");
        continue;
      }

      //A-1b = x

      let mut adj = na::Matrix2::<f64>::repeat(0_f64);
      adj[(0, 0)] = a[(1, 1)];
      adj[(0, 1)] = -a[(0, 1)];
      adj[(1, 0)] = -a[(1, 0)];
      adj[(1, 1)] = a[(0, 0)];

      println!("adj: {adj}");

      let x = adj * b / det;
      let r = x[0];
      let s = x[1];
      println!("{r} {s}");
      if r < 0_f64 || s < 0_f64 {
        continue;
      }

      let x = hs1.0 .0 as f64 + r * hs1.1 .0 as f64;
      let y = hs1.0 .1 as f64 + r * hs1.1 .1 as f64;
      if x >= min as f64 && y >= min as f64 && x <= max as f64 && y <= max as f64 {
        count += 1;
      }
    }
  }

  count
}

#[allow(dead_code)]
fn analyse_data1c(hailstones: &Vec<(Position, Velocity)>, min: usize, max: usize) -> usize {
  let mut count = 0;
  for i in 0..hailstones.len() {
    let hs1 = &hailstones[i];
    for (_j, hs2) in hailstones.iter().enumerate().skip(i + 1) {
      let mut a = na::Matrix2::<f64>::repeat(0_f64);
      let mut b = na::Vector2::<f64>::repeat(0_f64);

      a[(0, 0)] = hs1.1 .0 as f64;
      a[(0, 1)] = -hs2.1 .0 as f64;
      a[(1, 0)] = hs1.1 .1 as f64;
      a[(1, 1)] = -hs2.1 .1 as f64;

      b[0] = hs2.0 .0 as f64 - hs1.0 .0 as f64;
      b[1] = hs2.0 .1 as f64 - hs1.0 .1 as f64;

      let det = a.determinant();
      println!("determinant: {det}");
      if det == 0_f64 {
        println!("parallel");
        continue;
      }

      //A-1b = x
      let x = a.try_inverse().unwrap() * b;

      let r = x[0];
      let s = x[1];
      println!("{r} {s}");
      if r < 0_f64 || s < 0_f64 {
        continue;
      }

      let x = hs1.0 .0 as f64 + r * hs1.1 .0 as f64;
      let y = hs1.0 .1 as f64 + r * hs1.1 .1 as f64;
      if x >= min as f64 && y >= min as f64 && x <= max as f64 && y <= max as f64 {
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
  fn test_load1a() {
    let data = load_data("testinput.txt");
    let score1 = analyse_data1a(&data, 7, 27);
    assert_eq!(score1, 2);
  }

  #[test]
  fn test_load1b() {
    let data = load_data("testinput.txt");
    let score1 = analyse_data1b(&data, 7, 27);
    assert_eq!(score1, 2);
  }

  #[test]
  fn test_load1c() {
    let data = load_data("testinput.txt");
    let score1 = analyse_data1c(&data, 7, 27);
    assert_eq!(score1, 2);
  }

  #[test]
  fn test_load2() {
    pyo3::prepare_freethreaded_python();
    let data = load_data("testinput.txt");
    let score1 = analyse_data2(&data);
    assert_eq!(score1, 47);
  }

  #[test]
  fn test_load2b() {
    let data = load_data("testinput.txt");
    let _score1 = analyse_data2b(&data);
    //assert_eq!(score1, 47);
  }
}
