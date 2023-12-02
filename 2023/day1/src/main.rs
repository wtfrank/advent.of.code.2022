
use std::fs::File;
use std::io::Read;
use clap::Parser;
use log::debug;


use rustc_hash::FxHashMap;
type HashMap<T,U> = FxHashMap<T,U>;

/// Day 19 of Advent of Code 2022
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value_t=false)]
   benchmark: bool,
}


#[derive(Debug,PartialEq,Clone)]
struct Materials {
  ore: u16,
  clay: u16,
  obsidian: u16,
}

#[derive(Debug,PartialEq,Clone)]
struct Blueprint {
  orebot: Materials,
  claybot: Materials,
  obsidianbot: Materials,
  geodebot: Materials
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn test_load1() {
      let calib = load_calib( "testinput.txt" );
      assert_eq!(calib, 142 );
    }

    #[test]
    fn test_load2() {
      let calib = load_calib2( "testinput2.txt" );
      assert_eq!(calib, 281 );
    }

}
 
fn load_calib( filename: &str) -> usize
{
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut calib:usize = 0;
  for line in contents.lines() {
    for c in line.chars() {
      if c >= '0' && c <= '9' {
        let val = c.to_digit(10).unwrap() as usize;
        calib += 10 * val;
        break;
      }
    }
    for c in line.chars().rev() {
      if c >= '0' && c <= '9' {
        let val = c.to_digit(10).unwrap() as usize;
        calib += val;
        break;
      }
    }
  }
  calib
}



fn load_calib2( filename: &str) -> usize
{
  let mut tokens = HashMap::<&str,usize>::default();
  tokens.insert("one", 1);
  tokens.insert("1", 1);
  tokens.insert("two", 2);
  tokens.insert("2", 2);
  tokens.insert("three", 3);
  tokens.insert("3", 3);
  tokens.insert("four", 4);
  tokens.insert("4", 4);
  tokens.insert("five", 5);
  tokens.insert("5", 5);
  tokens.insert("six", 6);
  tokens.insert("6", 6);
  tokens.insert("seven", 7);
  tokens.insert("7", 7);
  tokens.insert("eight", 8);
  tokens.insert("8", 8);
  tokens.insert("nine", 9);
  tokens.insert("9", 9);

  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  let mut calib:usize = 0;
  for line in contents.lines() {
    let mut lowest_pos = 9999999999;
    let mut lowest_value = 0;
    for (token,value) in tokens.iter() {
      let pos = line.find(token);
      if pos.is_some() {
        let pos = pos.unwrap();
        if pos < lowest_pos {
          lowest_pos = pos;
          lowest_value = *value;
        }
      }
    }
    calib += 10 * lowest_value;

    let mut highest_pos = 0;
    let mut highest_value = 0;
    for (token,value) in tokens.iter() {
      let pos = line.rfind(token);
      if pos.is_some() {
        let pos = pos.unwrap();
        if pos >= highest_pos {
          highest_pos = pos;
          highest_value = *value;
        }
      }
    }
 
    calib += highest_value;
  }
  calib
}



fn main() {
    env_logger::init();

    let args = Args::parse();
    if args.benchmark {
      return;
    }

    let answer1 = load_calib( "input1.txt" );
    println!("answer: {answer1}");
    let answer2 = load_calib2( "input1.txt" );
    println!("answer: {answer2}");
}
