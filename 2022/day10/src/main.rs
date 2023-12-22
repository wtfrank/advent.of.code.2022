use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Point {
  x: isize,
  y: isize,
}
impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

fn render_image(x_states: &Vec<isize>) -> String {
  let mut output = String::new();
  for i in 1..x_states.len() {
    let x = x_states[i - 1];
    let col: isize = (i as isize - 1) % 40;
    if x >= col - 1 && x <= col + 1 {
      output += "#";
    } else {
      output += ".";
    }
    if (i % 40) == 0 {
      output += "\n";
    }
  }

  output
}

fn calculate_signal_strength(x_states: &[isize]) -> isize {
  let mut ret = 0;
  let mut i = 20;
  loop {
    ret += x_states[i - 1] * i as isize;
    i += 40;
    if i > 220 {
      break;
    }
  }

  ret
}

/* @return vector containing the state of x register after executing
 * the nth cycle (where n starts at 1).
 * so 0th position is before executing 1st cycle,
 * 1st position is after executing 1st cycle
 */
fn simulate_execution(f: &str) -> Vec<isize> {
  let mut file = File::open(f).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut x_states = Vec::<isize>::new();
  x_states.push(1);

  for l in contents.lines() {
    let mut t = l.split_whitespace();
    let first = t.next().unwrap();
    match first {
      "noop" => {
        x_states.push(x_states[x_states.len() - 1]);
      }
      "addx" => {
        let arg = t.next().unwrap().parse::<isize>().unwrap();
        x_states.push(x_states[x_states.len() - 1]);
        x_states.push(x_states[x_states.len() - 1] + arg);
      }
      _ => panic!("unexpected input"),
    }
  }

  x_states
}

fn main() -> std::io::Result<()> {
  let x_states = simulate_execution("input10.txt");

  let signal_strength = calculate_signal_strength(&x_states);

  println!("signal strength: {}", signal_strength);
  println!("states: {}", x_states.len());

  let screenshot = render_image(&x_states);
  print!("{screenshot}");

  Ok(())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_visibility() {
    let x_states = simulate_execution("testinput.txt");
    let signal_strength = calculate_signal_strength(&x_states);
    assert!(signal_strength == 13140);
  }
  #[test]
  fn test_render() {
    let expected = "##..##..##..##..##..##..##..##..##..##..\n\
###...###...###...###...###...###...###.\n\
####....####....####....####....####....\n\
#####.....#####.....#####.....#####.....\n\
######......######......######......####\n\
#######.......#######.......#######.....\n";
    let x_states = simulate_execution("testinput.txt");
    let screenshot = render_image(&x_states);
    println!("Rendered:\n{screenshot}");
    println!("Expected:\n{expected}");
    assert!(screenshot == expected);
  }
}
