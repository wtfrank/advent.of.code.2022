
use std::fs::File;
use std::io::Read;
use clap::Parser;

use std::collections::HashSet;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

/// Day 3 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value_t=false)]
   benchmark: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
      let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
      let (card_id, winning_nos, elf_nos) = parse_line(&line);
      assert_eq!(card_id, 1);
      assert_eq!(winning_nos.len(), 5);
      assert_eq!(elf_nos.len(), 8);
    }
    #[test]
    fn test_load1() {
      let (score1, score2) = load_data( "testinput.txt" );
      assert_eq!(score1, 13);
      assert_eq!(score2, 30);
    }

}


fn parse_line( line: &str) -> (usize, HashSet::<usize>, Vec::<usize>) {
  let mut winning_nos = HashSet::<usize>::new();
  let mut elf_nos = Vec::<usize>::new();


  let r = sscanf::sscanf_unescaped!(line, "Card *{usize}: {String} [|] {String}").unwrap();

  let card_id = r.0;
  let win_str = r.1;
  let elf_str = r.2;

  for n in win_str.split(' ') {
    if n.is_empty() { continue; }
    let number = n.parse::<usize>().unwrap();
    winning_nos.insert(number);
  }

  //println!("win_str: \"{win_str}\", elf_str: \"{elf_str}\"");
  for n in elf_str.split(' ') {
    if n.is_empty() { continue; }
    let number = n.parse::<usize>().unwrap();
    elf_nos.push(number);
  }

  (card_id, winning_nos, elf_nos)
}

fn analyse_card( winning_nos: &HashSet::<usize>, elf_nos: &Vec::<usize>) -> (usize, usize) {
  let mut score = 0;
  let mut wins = 0;
  for n in elf_nos {
    if winning_nos.contains(n) {
      wins += 1;
      if score == 0 {
        score = 1;
      }
      else {
        score *= 2;
      }
    }
  }
  (wins, score)
}

fn load_data( filename: &str) -> (usize, usize)
{
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut card_qty = Vec::<usize>::new();

  for _i in contents.lines() {
    card_qty.push(1);
  }

  let mut score = 0;
  for line in contents.lines() {
    let (card_id, winning_nos, elf_nos) = parse_line(line);

    let (wins, s) = analyse_card(&winning_nos, &elf_nos);
    score += s;

    //println!("card id {card_id}, wins: {wins}");

    //card idx is one less than id
    for i in (card_id) .. (card_id+wins) {
      if i > card_qty.len() -1 {
        println!("skipping win {i}");
        continue;
      }
      card_qty[i] = card_qty.get(i).unwrap() + card_qty.get(card_id-1).unwrap();
    }

  }

  for i in card_qty.iter() {
    println!("{i}");
  }
  let total_cards = card_qty.iter().sum();

  (score, total_cards)
}


fn main() {
    env_logger::init();

    let args = Args::parse();
    if args.benchmark {
      return;
    }

    let (score1, score2) = load_data( "testinput.txt" );
    println!("score: {score1}, {score2} ");

    let (score1, score2) = load_data( "input4.txt" );
    println!("score: {score1}, {score2} ");

}
