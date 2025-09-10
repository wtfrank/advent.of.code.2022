use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::prelude::*;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

/* The input for this task is similar to yaml
 * so we make some small alterations which allow
 * the input to be processed by a yaml parser
 */
fn remap_to_yaml(input: &str) -> String {
  let mut output = String::new();
  for l in input.lines() {
    if l.starts_with("    If") {
      output += &l[2..l.len()];
      output += "\n";
    } else {
      output += l;
      output += "\n";
    }
  }
  output
}

fn deserialize_starting_items<'de, D>(deserializer: D) -> Result<VecDeque<usize>, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;

  let mut v = VecDeque::<usize>::new();
  for t in s.split(|c: char| c.is_whitespace() || c == ',') {
    if t.is_empty() {
      continue;
    }
    let u = t.parse::<usize>().map_err(D::Error::custom)?;
    v.push_back(u);
  }
  Ok(v)
}

fn parse_monkevar(input: &str) -> Result<MonkeVar, ()> {
  if input == "old" {
    return Ok(MonkeVar::Old);
  }

  let u: usize = input.parse().map_err(|_| ())?;
  Ok(MonkeVar::Num(u))
}

fn parse_monkeop(input: &str) -> Result<MonkeOperator, ()> {
  match input {
    "*" => Ok(MonkeOperator::Multiply),
    "+" => Ok(MonkeOperator::Add),
    _ => Err(()),
  }
}

fn deserialize_operation<'de, D>(deserializer: D) -> Result<(MonkeVar, MonkeOperator, MonkeVar), D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;

  let mut tokens = s.split_whitespace();

  if tokens.next().unwrap() != "new" {
    return Err(D::Error::custom("bad operation format"));
  }
  if tokens.next().unwrap() != "=" {
    return Err(D::Error::custom("bad operation format"));
  }

  let var1 = parse_monkevar(tokens.next().unwrap()).map_err(|_| D::Error::custom("bad operation format"))?;
  let op = parse_monkeop(tokens.next().unwrap()).map_err(|_| D::Error::custom("bad operation format"))?;
  let var2 = parse_monkevar(tokens.next().unwrap()).map_err(|_| D::Error::custom("bad operation format"))?;

  Ok((var1, op, var2))
}

fn deserialize_test<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;

  let mut tokens = s.split_whitespace();

  if tokens.next().unwrap() != "divisible" {
    return Err(D::Error::custom("bad test format"));
  }

  if tokens.next().unwrap() != "by" {
    return Err(D::Error::custom("bad test format"));
  }

  tokens.next().unwrap().parse::<usize>().map_err(D::Error::custom)
}

fn deserialize_test_result<'de, D>(deserializer: D) -> Result<usize, D::Error>
where
  D: Deserializer<'de>,
{
  let s: &str = Deserialize::deserialize(deserializer)?;

  let mut tokens = s.split_whitespace();

  if tokens.next().unwrap() != "throw" {
    return Err(D::Error::custom("bad result format"));
  }

  if tokens.next().unwrap() != "to" {
    return Err(D::Error::custom("bad result format"));
  }

  if tokens.next().unwrap() != "monkey" {
    return Err(D::Error::custom("bad result format"));
  }

  tokens.next().unwrap().parse::<usize>().map_err(D::Error::custom)
}

impl std::fmt::Display for Monke {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    let mut starting_items = String::new();
    for si in self.has.iter() {
      if !starting_items.is_empty() {
        starting_items += ", ";
      }
      starting_items += &si.to_string();
    }

    write!(
      f,
      "has: {}. {}{}{}. Test:{} ? {} : {}",
      starting_items,
      self.operation.0,
      self.operation.1,
      self.operation.2,
      self.test_divisor,
      self.success_monke,
      self.failure_monke
    )
  }
}

fn load_monke(f: &str) -> Vec<Monke> {
  let mut file = File::open(f).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let contents = remap_to_yaml(&contents);

  let mut parsed: BTreeMap<String, Monke> = serde_yaml::from_str(&contents).unwrap();
  for (k, v) in &parsed {
    println!("{k} {v}");
  }

  let mut monkes = Vec::<Monke>::new();
  let mut i = 0;
  loop {
    let name = std::format!("Monkey {i}");
    match parsed.remove(&name) {
      Some(m) => {
        monkes.push(m);
      }
      None => {
        break;
      }
    }
    i += 1;
  }

  monkes
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
enum MonkeVar {
  Old,
  Num(usize),
}

impl std::fmt::Display for MonkeVar {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      MonkeVar::Old => write!(f, "old"),
      MonkeVar::Num(u) => write!(f, "{}", u),
    }
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
enum MonkeOperator {
  Multiply,
  Add,
}

impl std::fmt::Display for MonkeOperator {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      MonkeOperator::Multiply => write!(f, " * "),
      MonkeOperator::Add => write!(f, " + "),
    }
  }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Monke {
  #[serde(rename = "Starting items", deserialize_with = "deserialize_starting_items")]
  has: VecDeque<usize>,
  #[serde(rename = "Operation", deserialize_with = "deserialize_operation")]
  operation: (MonkeVar, MonkeOperator, MonkeVar),
  #[serde(rename = "Test", deserialize_with = "deserialize_test")]
  test_divisor: usize,
  #[serde(rename = "If true", deserialize_with = "deserialize_test_result")]
  success_monke: usize,
  #[serde(rename = "If false", deserialize_with = "deserialize_test_result")]
  failure_monke: usize,
  #[serde(default)]
  inspections: usize,
}

impl Monke {
  fn receive(&mut self, item: usize) {
    self.has.push_back(item);
  }
}

fn process_operation(initial: usize, operation: (MonkeVar, MonkeOperator, MonkeVar)) -> usize {
  let lhs: usize = match operation.0 {
    MonkeVar::Old => initial,
    MonkeVar::Num(n) => n,
  };
  let rhs: usize = match operation.2 {
    MonkeVar::Old => initial,
    MonkeVar::Num(n) => n,
  };

  match operation.1 {
    MonkeOperator::Multiply => lhs * rhs,
    MonkeOperator::Add => lhs + rhs,
  }
}

fn process_round(monkes: &mut [Monke], reduce_worry: bool) {
  let mut divisor_multiple = 1;
  for i in 0..monkes.len() {
    let m: &Monke = monkes.get(i).unwrap();
    divisor_multiple *= m.test_divisor;
  }

  for i in 0..monkes.len() {
    let m: &mut Monke = monkes.get_mut(i).unwrap();
    // get everything we need from m before we transfer
    // to other monkes as we can't mutably borrow two items
    // from a vecdeque simultaneously.
    let mut items: VecDeque<usize> = m.has.drain(0..).collect();
    m.inspections += items.len();
    let sm = m.success_monke;
    let fm = m.failure_monke;
    let divisor = m.test_divisor;
    let operation = m.operation;
    while !items.is_empty() {
      let mut w: usize = items.pop_front().unwrap();
      // monke inspects
      w = process_operation(w, operation);

      // my worry reduces
      if reduce_worry {
        w = (w as f64 / 3.0) as usize;
      }
      //limit by multiple to ensure numbers don't grow to stupid levels
      w %= divisor_multiple;
      // throw object
      if w.is_multiple_of(divisor) {
        monkes[sm].receive(w);
      } else {
        monkes[fm].receive(w);
      }
    }
  }
}

fn calc_monke_business(monkes: &Vec<Monke>) -> (usize, usize) {
  assert!(monkes.len() >= 2);
  let mut heap = BinaryHeap::<usize>::new();
  for m in monkes {
    heap.push(m.inspections);
  }
  (heap.pop().unwrap(), heap.pop().unwrap())
}

fn main() -> std::io::Result<()> {
  let mut monke_states = load_monke("input11.txt");
  for _ in 0..20 {
    process_round(&mut monke_states, true);
  }
  let mb = calc_monke_business(&monke_states);
  println!("monke business: {}", mb.0 * mb.1);

  let mut monke_states = load_monke("input11.txt");
  for _ in 0..10_000 {
    process_round(&mut monke_states, false);
  }
  let mb = calc_monke_business(&monke_states);
  println!("monke business: {} {}:{}", mb.0, mb.1, mb.0 * mb.1);

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_remap() {
    let input = "Monkey 0:\n\
  Test: divisible by 23\n\
    If true: throw to monkey 2\n";
    let expected = "Monkey 0:\n\
  Test: divisible by 23\n\
  If true: throw to monkey 2\n";
    let output = remap_to_yaml(input);
    assert!(output == expected);
  }

  #[test]
  fn test_parse() {
    let monke_states = load_monke("testinput.txt");
    assert!(monke_states.len() == 4);
    let m0 = &monke_states[0];
    assert!(m0.has.len() == 2);
    assert!(m0.test_divisor == 23);
    assert!(m0.success_monke == 2);
    assert!(m0.failure_monke == 3);
  }
  #[test]
  fn test_round() {
    let mut monke_states = load_monke("testinput.txt");
    process_round(&mut monke_states, true);
    let m0 = &monke_states[0];
    assert!(m0.has.len() == 4);
    assert!(m0.has[0] == 20);
    assert!(m0.has[1] == 23);
    assert!(m0.has[2] == 27);
    assert!(m0.has[3] == 26);
    let m1 = &monke_states[1];
    assert!(m1.has.len() == 6);
    assert!(m1.has[0] == 2080);
    assert!(m1.has[1] == 25);
    assert!(m1.has[2] == 167);
    assert!(m1.has[3] == 207);
    assert!(m1.has[4] == 401);
    assert!(m1.has[5] == 1046);
    assert!(monke_states[2].has.is_empty());
    assert!(monke_states[3].has.is_empty());
  }
  #[test]
  fn test_inspections() {
    let mut monke_states = load_monke("testinput.txt");
    for _ in 0..20 {
      process_round(&mut monke_states, true);
    }
    let monke_business = calc_monke_business(&monke_states);
    assert!(monke_business.0 * monke_business.1 == 10605);
  }
  #[test]
  fn test_inspections_lotsofworry() {
    let mut monke_states = load_monke("testinput.txt");
    for _ in 0..10_000 {
      process_round(&mut monke_states, false);
    }
    let mb = calc_monke_business(&monke_states);
    assert!(mb.0 == 52166);
    assert!(mb.1 == 52013);
  }
}
