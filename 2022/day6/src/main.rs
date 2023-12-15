use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

use std::collections::VecDeque;

const MSG_SIZE: usize = 14;

fn main() -> std::io::Result<()> {
  let mut file = File::open("input6.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let mut v: VecDeque<char> = VecDeque::new();
  let mut num_chars = 0;

  for c in contents.chars() {
    v.push_back(c);
    num_chars += 1;
    if v.len() > MSG_SIZE {
      v.pop_front();
    }

    if v.len() < MSG_SIZE {
      continue;
    }

    let mut s = HashSet::<char>::new();
    for vc in &v {
      s.insert(*vc);
    }
    println!("{:?}", v);
    if s.len() == MSG_SIZE {
      break;
    }
  }

  println!("result: {}", num_chars);

  Ok(())
}
