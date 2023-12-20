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
use std::collections::VecDeque;

//use std::iter::zip;

//use std::collections::HashSet;
//use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

//use std::collections::HashMap;

//use advent::{prime_factors, lcm};
//use advent::Range;

/// Day 20 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

fn push_button(modules: &mut HashMap<String, Module>, noisy: bool) -> (usize, usize, usize, bool, bool, bool, bool) {
  let mut queue = VecDeque::<(Pulse, String, String)>::new();

  queue.push_back((Pulse::Low, "broadcaster".to_string(), "button".to_string()));

  let mut lowc = 0;
  let mut highc = 0;
  let mut rxc = 0;
  let mut km = false;
  let mut qs = false;
  let mut kz = false;
  let mut xj = false;

  while let Some((pulse, m_to, m_from)) = queue.pop_front() {
    if noisy {
      println!("{pulse:?}, to {m_to}, from {m_from}");
    }

    match pulse {
      Pulse::Low => lowc += 1,
      Pulse::High => highc += 1,
    };
    if m_to == "rx" && pulse == Pulse::Low {
      rxc += 1;
    } else if m_to == "km" && pulse == Pulse::Low {
      km = true;
    } else if m_to == "qs" && pulse == Pulse::Low {
      qs = true;
    } else if m_to == "kz" && pulse == Pulse::Low {
      kz = true;
    } else if m_to == "xj" && pulse == Pulse::Low {
      xj = true;
    }

    let mw = modules.get_mut(&m_to);
    if mw.is_none() {
      if noisy {
        println!("non-existent module {m_to} found, assuming non-propagating output module");
      }
      continue;
    }
    let module = mw.unwrap();
    match module.kind {
      Kind::Broadcast => {
        for o in &module.outputs {
          queue.push_back((pulse, o.clone(), m_to.clone()));
        }
      }
      Kind::FlipFlop => {
        if pulse == Pulse::Low {
          let new_pulse = match module.ffstate {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
          };
          module.ffstate = new_pulse;
          for o in &module.outputs {
            queue.push_back((new_pulse, o.clone(), m_to.clone()));
          }
        }
      }
      Kind::Conjunction => {
        module.inputs.insert(m_from, pulse);
        if noisy {
          println!("conjunction {m_to} inputs:");
          for (n, p) in &module.inputs {
            println!("  inp: {p:?} from {n}");
          }
        }
        let new_pulse = if module.inputs.iter().all(|(_, p)| *p == Pulse::High) {
          Pulse::Low
        } else {
          Pulse::High
        };
        for o in &module.outputs {
          queue.push_back((new_pulse, o.clone(), m_to.clone()));
        }
      }
    }
  }

  (lowc, highc, rxc, km, qs, kz, xj)
}

fn analyse_data(modules: &mut HashMap<String, Module>) -> (usize, usize) {
  let mut lows = 0;
  let mut highs = 0;
  let mut rx_at = 0;
  let mut score1 = 0;

  for i in 0.. {
    let (l, h, rxc, km, qs, kz, xj) = push_button(modules, false);
    lows += l;
    highs += h;
    if rxc > 0 && rx_at == 0 {
      rx_at = i + 1;
      break;
    }
    if i == 999 {
      score1 = lows * highs;
    }
    if i % 10_000 == 0 {
      println!("i: {i}");
    }

    if km {
      println!("km at {}", i + 1);
    }
    if qs {
      println!("qs at {}", i + 1);
    }
    if kz {
      println!("kz at {}", i + 1);
    }

    if xj {
      println!("xj at {}", i + 1);
    }
  }

  println!("{lows} lows, {highs} highs");

  (score1, rx_at)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Pulse {
  Low,
  High,
}

#[derive(Debug, Eq, PartialEq)]
enum Kind {
  Broadcast,
  FlipFlop,
  Conjunction,
}

#[derive(Debug)]
struct Module {
  kind: Kind,
  outputs: Vec<String>,
  inputs: HashMap<String, Pulse>,
  ffstate: Pulse,
}

fn load_data(filename: &str) -> HashMap<String, Module> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut modules = HashMap::<String, Module>::default();

  let mut inputs = HashMap::<String, Vec<String>>::default();

  for line in contents.lines() {
    let r = sscanf::sscanf!(line, "{String} -> {String}").unwrap();

    let mut name = r.0;
    let mut it = name.chars();
    let kind = match it.next().unwrap() {
      '%' => Kind::FlipFlop,
      '&' => Kind::Conjunction,
      _ => Kind::Broadcast,
    };

    if kind != Kind::Broadcast {
      name = it.collect::<String>();
    }

    inputs.insert(name.clone(), Vec::new());
    let outputs = r.1.split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();
    modules.insert(
      name,
      Module {
        kind,
        outputs,
        inputs: HashMap::default(),
        ffstate: Pulse::Low,
      },
    );
  }

  //ensure conjunctions have all inputs set up
  for (n, m) in inputs.iter_mut() {
    for (n2, m2) in modules.iter() {
      if m2.outputs.iter().any(|o| *o == *n) {
        m.push(n2.clone());
      }
    }
  }

  for (n, m) in modules.iter_mut() {
    if m.kind != Kind::Conjunction {
      continue;
    }
    for i in inputs.get(n).unwrap() {
      m.inputs.insert(i.clone(), Pulse::Low);
    }
  }

  println!("total modules in config: {}", modules.len());

  modules
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
  let mut data = load_data("input20.txt");
  let (score1, _) = analyse_data(&mut data);
  println!("score1: {score1}");

  let mut data = load_data("input20.txt");
  let (_, score2) = analyse_data(&mut data);

  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load0() {
    let mut data = load_data("testinput.txt");
    let (l, h) = push_button(&mut data);
    assert_eq!(l, 8);
    assert_eq!(h, 4);
  }
  #[test]
  fn test_load1() {
    let mut data = load_data("testinput.txt");
    let (score1, _) = analyse_data(&mut data);
    assert_eq!(score1, 32000000);
  }
  #[test]
  fn test_load2() {
    let mut data = load_data("testinput2.txt");
    let (score1, _) = analyse_data(&mut data);
    assert_eq!(score1, 11687500);
  }
}
