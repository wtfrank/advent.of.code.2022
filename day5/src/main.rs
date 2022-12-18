use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::vec;
use std::fmt;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prio() {
        //assert_eq!(item_prio(&'a'), 1);
    }
}

#[derive(Debug)]
struct Range {
  start: u32,
  end: u32,
}

impl fmt::Display for Range{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "[{}-{}]", self.start, self.end)
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

fn parse_stack(columns: &mut Vec< Vec< char >>, line: &str) {
  let v:Vec<char> = line.chars().collect();
  let num_stacks:usize = (v.len()+1) / 4;
  if columns.len() == 0 {
    println!("dealing with {num_stacks} stacks");
    for _ in 0..num_stacks {
      columns.push( Vec::<char>::new() );
    }
  }

  for i in 0..num_stacks {
    let col:&mut Vec<char> = columns.get_mut(i).unwrap();
    let c = v[1+(i*4)];
    match c {
      'A'..='Z' => {col.insert(0, c); println!("adding {c} to stack {i}");},
      _ => ()
    }
  }
}

fn apply_move(columns: &mut Vec< Vec< char >>, line: &str) {
  let (count, mut from, mut to) = sscanf::sscanf!(line, "move {usize} from {usize} to {usize}").expect("invalid input line {line}");
  from -=1;
  to -=1;
  println!("moving {count} from {} to {}", from, to); 

  let f = columns.get_mut(from).unwrap();
  let mut temp:Vec<char> = f.drain(f.len()-count..).collect::<Vec<char>>();
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


  let mut columns: Vec< Vec< char > > = Vec::new();
  for line in contents.lines() {
    let mut chars = line.chars();
    let c = chars.next();
    match c {
      None => continue,
      Some(cc) => {
        if cc == '[' {
          parse_stack(&mut columns, &line);
        }
        else if cc == 'm' {
          apply_move(&mut columns, &line);
        }
        else {
            continue;
        }
      }
    }
  }

  let mut output:String = String::new();
  for mut c in columns {
    output.push(c.pop().unwrap());
  }

  println!("result: {}", output);

  Ok(())
}
