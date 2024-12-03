use clap::Parser;
use std::fs::File;
use std::io::Read;
//use log::debug;

/// Day 3 of Advent of Code 2024
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

enum MulParseStage {
  Start,
  SymM,
  SymU,
  SymL,
  SymOpenBr,
  Arg1,
  SymComma,
  Arg2,
}

fn analyse_input1(filename: &str) -> usize {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut state = MulParseStage::Start;
  let mut arg1: usize = 0;
  let mut arg2: usize = 0;
  let mut arg1_len: usize = 0;
  let mut arg2_len: usize = 0;

  let mut total = 0;

  for b in contents.as_bytes() {
    state = match state {
      MulParseStage::Start => {
        if *b == b'm' {
          MulParseStage::SymM
        } else {
          MulParseStage::Start
        }
      }
      MulParseStage::SymM => {
        if *b == b'u' {
          MulParseStage::SymU
        } else {
          MulParseStage::Start
        }
      }
      MulParseStage::SymU => {
        if *b == b'l' {
          MulParseStage::SymL
        } else {
          MulParseStage::Start
        }
      }
      MulParseStage::SymL => {
        if *b == b'(' {
          MulParseStage::SymOpenBr
        } else {
          MulParseStage::Start
        }
      }
      MulParseStage::SymOpenBr => {
        if *b >= b'0' && *b <= b'9' {
          arg1_len = 1;
          arg1 = (*b - b'0') as usize;
          MulParseStage::Arg1
        } else {
          MulParseStage::Start
        }
      }
      MulParseStage::Arg1 => {
        if *b == b',' {
          MulParseStage::SymComma
        } else if arg1_len >= 3 {
          MulParseStage::Start
        } else if *b >= b'0' && *b <= b'9' {
          arg1_len += 1;
          arg1 = 10 * arg1 + (*b - b'0') as usize;
          MulParseStage::Arg1
        } else {
          MulParseStage::Start
        }
      }
      MulParseStage::SymComma => {
        if *b >= b'0' && *b <= b'9' {
          arg2_len = 1;
          arg2 = (*b - b'0') as usize;
          MulParseStage::Arg2
        } else {
          MulParseStage::Start
        }
      }
      MulParseStage::Arg2 => {
        if *b == b')' {
          total += arg1 * arg2;
          MulParseStage::Start
        } else if arg2_len >= 3 {
          MulParseStage::Start
        } else if *b >= b'0' && *b <= b'9' {
          arg2_len += 1;
          arg2 = 10 * arg2 + (*b - b'0') as usize;
          MulParseStage::Arg2
        } else {
          MulParseStage::Start
        }
      }
    }
  }
  total
}

enum MulParseStage2 {
  Start,
  SymM,
  SymU,
  SymL,
  SymOpenBr,
  Arg1,
  SymComma,
  Arg2,
  SymDoOpenBr,
  SymDontD,
  SymDontO,
  SymDontN,
  SymDontApost,
  SymDontT,
  SymDontOpenBr,
}

fn analyse_input2(filename: &str) -> usize {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut state = MulParseStage2::Start;
  let mut arg1: usize = 0;
  let mut arg2: usize = 0;
  let mut arg1_len: usize = 0;
  let mut arg2_len: usize = 0;

  let mut total = 0;
  let mut doing = true;

  for b in contents.as_bytes() {
    state = match state {
      MulParseStage2::Start => {
        if *b == b'm' {
          MulParseStage2::SymM
        } else if *b == b'd' {
          MulParseStage2::SymDontD
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymM => {
        if *b == b'u' {
          MulParseStage2::SymU
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymU => {
        if *b == b'l' {
          MulParseStage2::SymL
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymL => {
        if *b == b'(' {
          MulParseStage2::SymOpenBr
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymOpenBr => {
        if *b >= b'0' && *b <= b'9' {
          arg1_len = 1;
          arg1 = (*b - b'0') as usize;
          MulParseStage2::Arg1
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::Arg1 => {
        if *b == b',' {
          MulParseStage2::SymComma
        } else if arg1_len >= 3 {
          MulParseStage2::Start
        } else if *b >= b'0' && *b <= b'9' {
          arg1_len += 1;
          arg1 = 10 * arg1 + (*b - b'0') as usize;
          MulParseStage2::Arg1
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymComma => {
        if *b >= b'0' && *b <= b'9' {
          arg2_len = 1;
          arg2 = (*b - b'0') as usize;
          MulParseStage2::Arg2
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::Arg2 => {
        if *b == b')' {
          if doing {
            total += arg1 * arg2;
          }
          MulParseStage2::Start
        } else if arg2_len >= 3 {
          MulParseStage2::Start
        } else if *b >= b'0' && *b <= b'9' {
          arg2_len += 1;
          arg2 = 10 * arg2 + (*b - b'0') as usize;
          MulParseStage2::Arg2
        } else {
          MulParseStage2::Start
        }
      }

      // Do/Don't
      MulParseStage2::SymDontD => {
        if *b == b'o' {
          MulParseStage2::SymDontO
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymDontO => {
        if *b == b'(' {
          MulParseStage2::SymDoOpenBr
        } else if *b == b'n' {
          MulParseStage2::SymDontN
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymDontN => {
        if *b == b'\'' {
          MulParseStage2::SymDontApost
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymDontApost => {
        if *b == b't' {
          MulParseStage2::SymDontT
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymDontT => {
        if *b == b'(' {
          MulParseStage2::SymDontOpenBr
        } else {
          MulParseStage2::Start
        }
      }
      MulParseStage2::SymDontOpenBr => {
        if *b == b')' {
          doing = false;
          MulParseStage2::Start
        } else {
          MulParseStage2::Start
        }
      }
      //Do
      MulParseStage2::SymDoOpenBr => {
        if *b == b')' {
          doing = true;
          MulParseStage2::Start
        } else {
          MulParseStage2::Start
        }
      }
    }
  }
  total
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let answer1 = analyse_input1("input3.txt");
  println!("answer: {answer1}");
  let answer2 = analyse_input2("input3.txt");
  println!("answer2: {answer2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let calib = analyse_input1("testinput1.txt");
    assert_eq!(calib, 161);
  }

  #[test]
  fn test_load2() {
    let calib = analyse_input2("testinput2.txt");
    assert_eq!(calib, 48);
  }
}
