use std::fs::File;
use std::io::prelude::*;

fn parse_stack(columns: &mut Vec<Vec<char>>, line: &str) {
  let v: Vec<char> = line.chars().collect();
  let num_stacks: usize = (v.len() + 1) / 4;
  if columns.is_empty() {
    println!("dealing with {num_stacks} stacks");
    for _ in 0..num_stacks {
      columns.push(Vec::<char>::new());
    }
  }

  for i in 0..num_stacks {
    let col: &mut Vec<char> = columns.get_mut(i).unwrap();
    let c = v[1 + (i * 4)];
    if let 'A'..='Z' = c {
      col.insert(0, c);
      println!("adding {c} to stack {i}");
    }
    /*
    match c {
      'A'..='Z' => {col.insert(0, c); println!("adding {c} to stack {i}");},
      _ => ()
    }*/
  }
}

fn apply_move(columns: &mut [Vec<char>], line: &str) {
  let (count, mut from, mut to) =
    sscanf::sscanf!(line, "move {usize} from {usize} to {usize}").expect("invalid input line");
  from -= 1;
  to -= 1;
  println!("moving {count} from {} to {}", from, to);

  let f = columns.get_mut(from).unwrap();
  let mut temp: Vec<char> = f.drain(f.len() - count..).collect::<Vec<char>>();
  columns.get_mut(to).unwrap().append(&mut temp);
  /*
  for _ in 0..count {
    let c:char = columns.get_mut(from).unwrap().pop().unwrap();
    columns.get_mut(to).unwrap().push(c);
  }*/
}

fn main() -> std::io::Result<()> {
  let mut file = File::open("input5.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let mut columns: Vec<Vec<char>> = Vec::new();
  for line in contents.lines() {
    let mut chars = line.chars();
    let c = chars.next();
    match c {
      None => continue,
      Some(cc) => {
        if cc == '[' {
          parse_stack(&mut columns, line);
        } else if cc == 'm' {
          apply_move(&mut columns, line);
        } else {
          continue;
        }
      }
    }
  }

  let mut output: String = String::new();
  for mut c in columns {
    output.push(c.pop().unwrap());
  }

  println!("result: {}", output);

  Ok(())
}
