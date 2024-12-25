use std::fs::File;
use std::io::Read;

#[allow(unused_imports)]
use advent::{Dims, Direction, Point, TerrainMap};

#[allow(unused_imports)]
use std::collections::VecDeque;

#[allow(unused_imports)]
use rustc_hash::{FxHashMap, FxHashSet};
#[allow(dead_code)]
type HashMap<T, U> = FxHashMap<T, U>;
#[allow(dead_code)]
type HashSet<T> = FxHashSet<T>;

#[allow(unused_imports)]
use priority_queue::PriorityQueue;
#[allow(unused_imports)]
use std::cmp::Reverse;

#[allow(unused_imports)]
use enum_iterator::{all, Sequence};

#[allow(unused_imports)]
use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;

#[allow(unused_imports)]
use std::sync::OnceLock;

#[allow(unused_imports)]
use regex::Regex;

#[allow(unused_imports)]
use rand::{
  distributions::{Distribution, Uniform},
  prelude::*,
  Rng,
};

#[allow(unused_imports)]
use rand_chacha::ChaCha8Rng;

pub fn load_data(filename: &str) -> String {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

#[allow(clippy::type_complexity)]
fn parse_input(puzzle_input: &str) -> (Vec<&str>, HashMap<&str, usize>, Vec<(&str, &str, &str, &str)>) {
  let mut z_wires = HashSet::<&str>::default();
  let mut inputs = HashMap::<&str, usize>::default();
  let mut gates = Vec::<(&str, &str, &str, &str)>::new();

  let ra = Regex::new(r"^(.+) ([A-Z]+) (.+) -> (.+)$").unwrap();
  let mut parsing_inputs = true;
  for line in puzzle_input.lines() {
    if parsing_inputs {
      if line.is_empty() {
        parsing_inputs = false;
        continue;
      }

      let mut l = line.split(": ");
      let input = l.next().unwrap();
      let value = l.next().unwrap().parse::<usize>().unwrap();
      assert!(l.next().is_none());
      assert!(value == 1 || value == 0);

      inputs.insert(input, value);
      if input.starts_with('z') {
        z_wires.insert(input);
      }
    } else {
      let c = ra.captures(line).expect("input format not as expected");
      if c.len() != 5 {
        panic!("bad match");
      }

      let inp1: &str = c.get(1).unwrap().as_str();
      let gate: &str = c.get(2).unwrap().as_str();
      let inp2: &str = c.get(3).unwrap().as_str();
      let outp: &str = c.get(4).unwrap().as_str();
      gates.push((inp1, gate, inp2, outp));

      if outp.starts_with('z') {
        z_wires.insert(outp);
      }
    }
  }
  let z_wires: Vec<&str> = itertools::sorted(z_wires.iter()).rev().copied().collect();

  //zwires is an array with the name of the MSB at position 0
  (z_wires, inputs, gates)
}

pub fn analyse_input1(puzzle_input: &str) -> usize {
  let (z_wires, mut inputs, mut gates) = parse_input(puzzle_input);

  while !gates.is_empty() {
    for i in (0..gates.len()).rev() {
      let gate = gates[i];
      if !inputs.contains_key(gate.0) || !inputs.contains_key(gate.2) {
        continue;
      }
      let inpval1 = inputs.get(gate.0).unwrap();
      let inpval2 = inputs.get(gate.2).unwrap();
      let op = gate.1;
      let outp = gate.3;
      let outpval = match op {
        "OR" => {
          if *inpval1 == 1 || *inpval2 == 1 {
            1
          } else {
            0
          }
        }
        "AND" => {
          if *inpval1 == 1 && *inpval2 == 1 {
            1
          } else {
            0
          }
        }
        "XOR" => {
          if *inpval1 != *inpval2 {
            1
          } else {
            0
          }
        }
        _ => panic!("unexpected op {op}"),
      };
      inputs.insert(outp, outpval);
      gates.remove(i);
    }
  }

  let mut total = 0;
  for z in z_wires.iter() {
    total += inputs.get(z).unwrap();
    total <<= 1;
  }
  total >>= 1;

  total
}

fn evaluate(
  input1: usize,
  input2: usize,
  gates: &[(&str, &str, &str, &str)],
  input1_names: &[&str],
  input2_names: &[&str],
  z_wires: &[&str],
) -> usize {
  let mut gates = gates.to_vec();
  let mut inputs = HashMap::<&str, usize>::default();

  for i in 0..input1_names.len() {
    inputs.insert(input1_names[i], (input1 >> i) & 1);
    inputs.insert(input2_names[i], (input2 >> i) & 1);
  }

  /*
    println!("input1 names: {input1_names:?}");
    println!("input2 names: {input2_names:?}");
    println!("output names: {z_wires:?}");

    println!("inputs: {:?}", inputs.keys());
  */
  while !gates.is_empty() {
    for i in (0..gates.len()).rev() {
      let gate = gates[i];
      if !inputs.contains_key(gate.0) || !inputs.contains_key(gate.2) {
        continue;
      }
      let inpval1 = inputs.get(gate.0).unwrap();
      let inpval2 = inputs.get(gate.2).unwrap();
      let op = gate.1;
      let outp = gate.3;
      let outpval = match op {
        "OR" => {
          if *inpval1 == 1 || *inpval2 == 1 {
            1
          } else {
            0
          }
        }
        "AND" => {
          if *inpval1 == 1 && *inpval2 == 1 {
            1
          } else {
            0
          }
        }
        "XOR" => {
          if *inpval1 != *inpval2 {
            1
          } else {
            0
          }
        }
        _ => panic!("unexpected op {op}"),
      };
      inputs.insert(outp, outpval);
      gates.remove(i);
    }
  }

  let mut total = 0;
  for z in z_wires {
    total += inputs.get(z).unwrap();
    total <<= 1;
  }
  total >>= 1;
  total
}

const CHECK_ITERS: usize = 150;

pub fn analyse_input2(puzzle_input: &str) -> String {
  let (z_wires, inputs, gates) = parse_input(puzzle_input);
  let z_wire_to_bit: HashMap<&str, usize> = z_wires
    .iter()
    .enumerate()
    .map(|(idx, z)| (*z, z_wires.len() - idx - 1))
    .collect();

  //inputX_names is an array with the name of bit Z at position Z
  let input1_names: Vec<&str> = itertools::sorted(inputs.keys().filter(|n| n.starts_with('x')).copied()).collect();
  let input2_names: Vec<&str> = itertools::sorted(inputs.keys().filter(|n| n.starts_with('y')).copied()).collect();

  let mut swapped_wires: Vec<&str> = vec![];

  println!("{z_wires:?}");
  let (wrong_count, _gate_suspicion, total_wrong) = validate_gates(&gates, &input1_names, &input2_names, &z_wires);

  println!("total wrong: {total_wrong}");
  println!("wrong counts: {wrong_count:?}");
  //println!("suspicious gates: {gate_suspicion:.2?}");

  let gates_per_output = gate_feeders(&z_wires, &gates);
  let output_gate_count: HashMap<&str, usize> = gates_per_output.iter().map(|(k, v)| (*k, v.len())).collect();
  println!("gate complexity: {output_gate_count:?}");
  let mut sorted_outputs = z_wires.clone();
  sorted_outputs.sort_by_key(|k| output_gate_count.get(k).unwrap());
  println!("sorted outputs: {sorted_outputs:?}");

  let (gates, total_wrong2, mut swapped) = fix_compl3_error(
    &gates,
    &input1_names,
    &input2_names,
    &z_wires,
    &sorted_outputs,
    &z_wire_to_bit,
    &output_gate_count,
    &wrong_count,
  );
  swapped_wires.append(&mut swapped);
  println!("wrong count reduced from {total_wrong} to {total_wrong2}");

  let (wrong_count, _gate_suspicion, total_wrong) = validate_gates(&gates, &input1_names, &input2_names, &z_wires);

  println!("total wrong: {total_wrong}");
  println!("wrong counts: {wrong_count:?}");
  //println!("suspicious gates: {gate_suspicion:.2?}");

  let gates_per_output = gate_feeders(&z_wires, &gates);
  let output_gate_count: HashMap<&str, usize> = gates_per_output.iter().map(|(k, v)| (*k, v.len())).collect();
  println!("gate complexity: {output_gate_count:?}");
  let mut sorted_outputs = z_wires.clone();
  sorted_outputs.sort_by_key(|k| output_gate_count.get(k).unwrap());
  println!("sorted outputs: {sorted_outputs:?}");

  let (gates, total_wrong2, mut swapped) = fix_compl_error(
    &gates,
    &input1_names,
    &input2_names,
    &z_wires,
    &sorted_outputs,
    &z_wire_to_bit,
    &output_gate_count,
    &gates_per_output,
    &wrong_count,
  );
  swapped_wires.append(&mut swapped);
  println!("wrong count reduced from {total_wrong} to {total_wrong2}");

  let (wrong_count, _gate_suspicion, total_wrong) = validate_gates(&gates, &input1_names, &input2_names, &z_wires);

  println!("total wrong: {total_wrong}");
  println!("wrong counts: {wrong_count:?}");
  //println!("suspicious gates: {gate_suspicion:.2?}");

  let gates_per_output = gate_feeders(&z_wires, &gates);
  let output_gate_count: HashMap<&str, usize> = gates_per_output.iter().map(|(k, v)| (*k, v.len())).collect();
  println!("gate complexity: {output_gate_count:?}");
  let mut sorted_outputs = z_wires.clone();
  sorted_outputs.sort_by_key(|k| output_gate_count.get(k).unwrap());
  println!("sorted outputs: {sorted_outputs:?}");

  let (gates, total_wrong2, mut swapped) = fix_compl_error(
    &gates,
    &input1_names,
    &input2_names,
    &z_wires,
    &sorted_outputs,
    &z_wire_to_bit,
    &output_gate_count,
    &gates_per_output,
    &wrong_count,
  );
  swapped_wires.append(&mut swapped);
  println!("wrong count reduced from {total_wrong} to {total_wrong2}");

  let (wrong_count, _gate_suspicion, total_wrong) = validate_gates(&gates, &input1_names, &input2_names, &z_wires);

  println!("total wrong: {total_wrong}");
  println!("wrong counts: {wrong_count:?}");
  //println!("suspicious gates: {gate_suspicion:.2?}");

  let gates_per_output = gate_feeders(&z_wires, &gates);
  let output_gate_count: HashMap<&str, usize> = gates_per_output.iter().map(|(k, v)| (*k, v.len())).collect();
  println!("gate complexity: {output_gate_count:?}");
  let mut sorted_outputs = z_wires.clone();
  sorted_outputs.sort_by_key(|k| output_gate_count.get(k).unwrap());
  println!("sorted outputs: {sorted_outputs:?}");

  let (gates, total_wrong2, mut swapped) = fix_compl_error(
    &gates,
    &input1_names,
    &input2_names,
    &z_wires,
    &sorted_outputs,
    &z_wire_to_bit,
    &output_gate_count,
    &gates_per_output,
    &wrong_count,
  );
  swapped_wires.append(&mut swapped);
  println!("wrong count reduced from {total_wrong} to {total_wrong2}");

  let (wrong_count, _gate_suspicion, total_wrong) = validate_gates(&gates, &input1_names, &input2_names, &z_wires);
  println!("Final wrong count: {total_wrong}");
  println!("wrong counts: {wrong_count:?}");

  swapped_wires.sort();
  swapped_wires.join(",")
}

fn validate_gates(
  gates: &[(&str, &str, &str, &str)],
  input1_names: &[&str],
  input2_names: &[&str],
  z_wires: &[&str],
) -> (Vec<usize>, Vec<f64>, usize) {
  let mut wrong_count: Vec<usize> = vec![0; z_wires.len()]; // entry 0 has bit 0 wrong count
  let gate_suspicion: Vec<f64> = vec![0_f64; gates.len()];

  // let mut rng = rand::thread_rng();
  let max_val = (1 << z_wires.len()) - 1;
  let rng_range = Uniform::from(0..=max_val);
  let mut rng = ChaCha8Rng::seed_from_u64(2024);

  let mut total_wrong = 0;
  for _ in 0..CHECK_ITERS {
    let input1: usize = rng_range.sample(&mut rng);
    let input2: usize = rng_range.sample(&mut rng);

    let expected = input1 + input2;

    let output = evaluate(input1, input2, gates, input1_names, input2_names, z_wires);

    let wrong = expected ^ output;
    // let mut wrong_digit_count = 0;
    for (i, wc) in wrong_count.iter_mut().enumerate() {
      if (wrong >> i) & 1 != 0 {
        *wc += 1;
        // wrong_digit_count += 1;
        total_wrong += 1;

        //apportion_blame(&mut gate_suspicion, z_wires[z_wires.len() - i - 1], &gates);
      }
    }
    // println!("Wrong digit count: {wrong_digit_count}");
  }
  (wrong_count, gate_suspicion, total_wrong)
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn fix_compl3_error<'a>(
  gates: &'a [(&'a str, &'a str, &'a str, &'a str)],
  input1_names: &[&str],
  input2_names: &[&str],
  z_wires: &[&str],
  sorted_outputs: &[&str],
  z_wire_to_bit: &HashMap<&str, usize>,
  output_gate_count: &HashMap<&str, usize>,
  wrong_count: &[usize],
) -> (Vec<(&'a str, &'a str, &'a str, &'a str)>, usize, Vec<&'a str>) {
  // let correct_outputs: HashSet<&str> = HashSet::default();
  let mut compl_3_wrong: Option<&str> = None;
  let mut wrong_gate_idx: Option<usize> = None;
  for output in sorted_outputs.iter() {
    let bit = *z_wire_to_bit.get(output).unwrap();
    let wrong = wrong_count[bit];
    if wrong == 0 {
      println!(
        "{output} complexity {} is fine. Bit {bit}",
        output_gate_count.get(output).unwrap()
      );
    } else {
      let complexity = *output_gate_count.get(output).unwrap();
      println!("{output} complexity {complexity} is wrong. Bit {bit}.",);
      if complexity == 3 {
        compl_3_wrong = Some(output);
        for (idx, g) in gates.iter().enumerate() {
          if *g.3 == **output {
            wrong_gate_idx = Some(idx);
          }
        }
      }
    }
  }

  // try adjust a compl3 and see how it looks
  let compl_3_wrong = compl_3_wrong.unwrap();
  let wrong_gate_idx = wrong_gate_idx.unwrap();
  println!("Selected wrong output {compl_3_wrong} idx {wrong_gate_idx} to fix");

  let mut min_wrong = usize::MAX;
  let mut min_wrong_idx = 0;
  let mut fixed_gates = None;
  for i in 0..gates.len() {
    if i == wrong_gate_idx {
      continue;
    }
    let new_gates = swap_gate_outputs(i, wrong_gate_idx, gates);
    let total_wrong2 = evaluate_gates(&new_gates, input1_names, input2_names, z_wires);

    if total_wrong2 < min_wrong {
      min_wrong = total_wrong2;
      min_wrong_idx = i;
      fixed_gates = Some(new_gates);
    }
    // println!("{i}: original wrong: {total_wrong}, new wrong: {total_wrong2}");
  }

  let fixed_gates = fixed_gates.unwrap();

  let swapped = vec![fixed_gates[min_wrong_idx].3, fixed_gates[wrong_gate_idx].3];
  println!(
    "min wrong now {min_wrong} at idx {min_wrong_idx}. {:?} | {:?}. was: {:?} | {:?}",
    fixed_gates[min_wrong_idx], fixed_gates[wrong_gate_idx], gates[min_wrong_idx], gates[wrong_gate_idx],
  );

  (fixed_gates, min_wrong, swapped)
}

/*
fn fix_compl7_error<'a>(
  gates: &'a [(&'a str, &'a str, &'a str, &'a str)],
  input1_names: &[&str],
  input2_names: &[&str],
  z_wires: &[&str],
  sorted_outputs: &[&str],
  z_wire_to_bit: &HashMap<&str, usize>,
  output_gate_count: &HashMap<&str, usize>,
  output_gate_complexity: &HashMap<&str, HashSet<&str>>,
  wrong_count: &[usize],
) -> (Vec<(&'a str, &'a str, &'a str, &'a str)>, usize) {
  // let correct_outputs: HashSet<&str> = HashSet::default();
  let mut compl_7_wrong: Option<&str> = None;
  let mut wrong_gate_idxs: Option<Vec<usize>> = None;
  for output in sorted_outputs.iter() {
    let bit = *z_wire_to_bit.get(output).unwrap();
    let wrong = wrong_count[bit];
    if wrong == 0 {
      println!("{output} complexity {} is fine", output_gate_count.get(output).unwrap());
    } else {
      let complexity = *output_gate_count.get(output).unwrap();
      if complexity == 7 {
        println!(
          "{output} complexity {complexity} is wrong. Potential miscreants: {:?}",
          output_gate_complexity.get(output).unwrap()
        );

        compl_7_wrong = Some(output);
        let mut indexes = Vec::new();
        for op in output_gate_complexity.get(output).unwrap() {
          if input1_names.contains(op) || input2_names.contains(op) {
            continue;
          }
          for (idx, g) in gates.iter().enumerate() {
            if *g.3 == **op {
              indexes.push(idx);
            }
          }
        }
        wrong_gate_idxs = Some(indexes);
      }
    }
  }

  let compl_7_wrong = compl_7_wrong.unwrap();
  let wrong_gate_idxs = wrong_gate_idxs.unwrap();
  println!("Selected wrong output {compl_7_wrong} idx {wrong_gate_idxs:?} to fix");

  let mut min_wrong = usize::MAX;
  let mut min_wrong_idx = 0;
  let mut other_wrong_idx = 0;
  let mut fixed_gates = None;
  for idx in wrong_gate_idxs.iter() {
    for i in 0..gates.len() {
      if i == *idx {
        continue;
      }
      let new_gates = swap_gate_outputs(i, *idx, &gates);
      if cycle_exists(&new_gates) {
        println!("cycle exists after swapping {i} {idx}");
        continue;
      }

      let total_wrong2 = evaluate_gates(&new_gates, input1_names, input2_names, z_wires);

      if total_wrong2 < min_wrong {
        min_wrong = total_wrong2;
        min_wrong_idx = i;
        other_wrong_idx = *idx;
        fixed_gates = Some(new_gates);
      } else if total_wrong2 == min_wrong {
        println!("multiple swaps led to multiple 7incorrectness!!!! min_wrong: {min_wrong}");
      }

      // println!("{i}: original wrong: {total_wrong}, new wrong: {total_wrong2}");
    }
  }

  let fixed_gates = fixed_gates.unwrap();

  println!(
    "min wrong now {min_wrong} at idx {min_wrong_idx}. {:?} | {:?}. was: {:?} | {:?}",
    fixed_gates[min_wrong_idx], fixed_gates[other_wrong_idx], gates[min_wrong_idx], gates[other_wrong_idx],
  );

  (fixed_gates, min_wrong)
}
*/

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn fix_compl_error<'a>(
  gates: &'a [(&'a str, &'a str, &'a str, &'a str)],
  input1_names: &[&str],
  input2_names: &[&str],
  z_wires: &[&str],
  sorted_outputs: &[&str],
  z_wire_to_bit: &HashMap<&str, usize>,
  output_gate_count: &HashMap<&str, usize>,
  output_gate_complexity: &HashMap<&str, HashSet<&str>>,
  wrong_count: &[usize],
) -> (Vec<(&'a str, &'a str, &'a str, &'a str)>, usize, Vec<&'a str>) {
  // let correct_outputs: HashSet<&str> = HashSet::default();
  let mut compl_7_wrong: Option<&str> = None;
  let mut wrong_gate_idxs: Option<Vec<usize>> = None;
  let mut correct_wires = HashSet::<&str>::default();
  for output in sorted_outputs.iter() {
    let bit = *z_wire_to_bit.get(output).unwrap();
    let wrong = wrong_count[bit];
    if wrong == 0 {
      println!("{output} complexity {} is fine", output_gate_count.get(output).unwrap());
      if compl_7_wrong.is_none() {
        for op in output_gate_complexity.get(output).unwrap() {
          correct_wires.insert(op);
        }
      }
    } else {
      let complexity = *output_gate_count.get(output).unwrap();
      println!("{output} complexity {complexity} is wrong. Bit {bit}.",);
      if compl_7_wrong.is_none() {
        println!("correct wires: {:?}", correct_wires.iter());
        let unfiltered_miscreants: Vec<&str> = output_gate_complexity
          .get(output)
          .unwrap()
          .iter()
          .copied()
          .filter(|op| !op.starts_with('x') && !op.starts_with('y'))
          .collect();
        println!("Potential miscreants: {:?}", unfiltered_miscreants);
        let suspicious_wires: Vec<&str> = unfiltered_miscreants
          .iter()
          .copied()
          .filter(|op| !correct_wires.contains(*op))
          .collect();

        println!("Filtered miscreants: {:?}", suspicious_wires);

        compl_7_wrong = Some(output);
        let mut indexes = Vec::new();
        for op in suspicious_wires.iter() {
          for (idx, g) in gates.iter().enumerate() {
            if *g.3 == **op {
              indexes.push(idx);
            }
          }
        }
        wrong_gate_idxs = Some(indexes);
      }
    }
  }

  let compl_7_wrong = compl_7_wrong.unwrap();
  let wrong_gate_idxs = wrong_gate_idxs.unwrap();
  println!("Selected wrong output {compl_7_wrong} idx {wrong_gate_idxs:?} to fix");

  let mut min_wrong = usize::MAX;
  let mut min_wrong_idx = 0;
  let mut other_wrong_idx = 0;
  let mut fixed_gates = None;
  for idx in wrong_gate_idxs.iter() {
    println!("trying swap with {:?}", gates[*idx]);
    for i in 0..gates.len() {
      if i == *idx {
        continue;
      }
      if correct_wires.contains(gates[i].3) {
        continue;
      }
      let new_gates = swap_gate_outputs(i, *idx, gates);
      if cycle_exists(&new_gates) {
        // println!("cycle exists after swapping {i} {idx}");
        continue;
      }

      let total_wrong2 = evaluate_gates(&new_gates, input1_names, input2_names, z_wires);

      if total_wrong2 < min_wrong {
        min_wrong = total_wrong2;
        min_wrong_idx = i;
        other_wrong_idx = *idx;
        fixed_gates = Some(new_gates);
      }
      // println!("{i}: original wrong: {total_wrong}, new wrong: {total_wrong2}");
    }
  }

  let fixed_gates = fixed_gates.unwrap();

  let swapped = vec![fixed_gates[min_wrong_idx].3, fixed_gates[other_wrong_idx].3];
  println!(
    "min wrong now {min_wrong} at idx {min_wrong_idx}. {:?} | {:?}. was: {:?} | {:?}",
    fixed_gates[min_wrong_idx], fixed_gates[other_wrong_idx], gates[min_wrong_idx], gates[other_wrong_idx],
  );

  (fixed_gates, min_wrong, swapped)
}

fn cycle_exists(gates: &[(&str, &str, &str, &str)]) -> bool {
  let mut visited = vec![false; gates.len()];
  let mut finished = vec![false; gates.len()];

  for idx in 0..gates.len() {
    if do_dfs(idx, gates, &mut visited, &mut finished) {
      return true;
    }
  }
  false
}

fn do_dfs(idx: usize, gates: &[(&str, &str, &str, &str)], visited: &mut [bool], finished: &mut [bool]) -> bool {
  if finished[idx] {
    return false;
  }
  if visited[idx] {
    return true;
  }
  visited[idx] = true;

  //neighbours is any gate that accepts as input my output
  let mut neighbours = vec![];
  let my_output = gates[idx].3;
  for (i, gate) in gates.iter().enumerate() {
    if *gate.0 == *my_output || *gate.2 == *my_output {
      neighbours.push(i);
    }
  }

  for n in neighbours {
    if do_dfs(n, gates, visited, finished) {
      return true;
    }
  }

  finished[idx] = true;
  false
}

fn swap_gate_outputs<'a>(
  gate_idx1: usize,
  gate_idx2: usize,
  gates: &[(&'a str, &'a str, &'a str, &'a str)],
) -> Vec<(&'a str, &'a str, &'a str, &'a str)> {
  let gates2: Vec<(&str, &str, &str, &str)> = gates
    .iter()
    .enumerate()
    .map(|(idx, g)| {
      if idx == gate_idx1 {
        (g.0, g.1, g.2, gates[gate_idx2].3)
      } else if idx == gate_idx2 {
        (g.0, g.1, g.2, gates[gate_idx1].3)
      } else {
        *g
      }
    })
    .collect();

  gates2
}

fn evaluate_gates(
  gates: &[(&str, &str, &str, &str)],
  input1_names: &[&str],
  input2_names: &[&str],
  z_wires: &[&str],
) -> usize {
  let max_val = (1 << z_wires.len()) - 1;
  let rng_range = Uniform::from(0..=max_val);
  let mut rng = ChaCha8Rng::seed_from_u64(2024);
  let mut total_wrong = 0;
  for _ in 0..CHECK_ITERS {
    let input1: usize = rng_range.sample(&mut rng);
    let input2: usize = rng_range.sample(&mut rng);

    let expected = input1 + input2;

    let output = evaluate(input1, input2, gates, input1_names, input2_names, z_wires);

    let wrong = expected ^ output;
    for i in 0..z_wires.len() {
      if (wrong >> i) & 1 != 0 {
        total_wrong += 1;
      }
    }
  }

  total_wrong
}

/*
// add blame to each gate that could have been involved in a wrong answer.
// question - should it be one point per gate no matter how far back in the chain?
// Or should it be split by the contribution each gate is likely to have made? i.e. halves as you go back?
fn apportion_blame(suspicion: &mut [f64], bad_output: &str, gates: &[(&str, &str, &str, &str)]) {
  let mut queue = VecDeque::<(&str, f64)>::new();
  let mut pushed = HashSet::<&str>::default();
  queue.push_back((bad_output, 1_f64));
  pushed.insert(bad_output);
  while !queue.is_empty() {
    let (output, blame) = queue.pop_front().unwrap();
    for (idx, (inp1, _, inp2, outp)) in gates.iter().enumerate() {
      if output == *outp {
        suspicion[idx] += blame;
        if !pushed.contains(inp1) {
          queue.push_back((inp1, blame / 2_f64));
          pushed.insert(inp1);
        }
        if !pushed.contains(inp2) {
          queue.push_back((inp2, blame / 2_f64));
          pushed.insert(inp2);
        }
      } else {
        //println!("output {outp} not involed in bad_output {bad_output}");
      }
    }
  }
  // println!("bad output {bad_output}, involved gates: {pushed:?}");
}
*/

fn gate_feeders<'a, 'b>(
  z_wires: &'b [&'b str],
  gates: &'a [(&'a str, &'a str, &'a str, &'a str)],
) -> HashMap<&'b str, HashSet<&'a str>>
where
  'b: 'a,
{
  let mut contributors = HashMap::default();
  for z_output in z_wires {
    let mut queue = VecDeque::<&str>::new();
    let mut pushed = HashSet::<&str>::default();
    queue.push_back(z_output);
    pushed.insert(z_output);
    while !queue.is_empty() {
      let output = queue.pop_front().unwrap();
      for (inp1, _, inp2, outp) in gates.iter() {
        if output == *outp {
          if !pushed.contains(inp1) {
            queue.push_back(inp1);
            pushed.insert(inp1);
          }
          if !pushed.contains(inp2) {
            queue.push_back(inp2);
            pushed.insert(inp2);
          }
        }
      }
    }
    contributors.insert(*z_output, pushed);
  }
  contributors
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1a() {
    let data = load_data("testinput1.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 4);
  }

  #[test]
  fn test_load1b() {
    let data = load_data("testinput2.txt");
    let result = analyse_input1(&data);
    assert_eq!(result, 2024);
  }

  #[test]
  fn test_load2() {
    let data = load_data("testinput2.txt");
    let result = analyse_input2(&data);
    assert_eq!(result, "");
  }
}
