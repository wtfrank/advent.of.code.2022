use clap::Parser;
use std::fs::File;
use std::io::Read;

//use std::iter::zip;

//use std::collections::HashSet;
use std::cmp::Ordering;

//use rustc_hash::FxHashMap;
//type HashMap<T,U> = FxHashMap<T,U>;

use std::collections::HashMap;

/// Day 3 of Advent of Code 2023
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  /// Name of the person to greet
  #[arg(short, long, default_value_t = false)]
  benchmark: bool,
}

#[derive(Debug, Eq)]
struct Hand {
  cards: String,
  bid: usize,
}

impl PartialOrd for Hand {
  fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl PartialEq for Hand {
  fn eq(&self, other: &Hand) -> bool {
    self.cards == other.cards
  }
}

impl Ord for Hand {
  fn cmp(&self, other: &Hand) -> Ordering {
    let a = self;
    let b = other;
    let a_strength = hand_strength(&a.cards);
    let b_strength = hand_strength(&b.cards);

    //println!("comparing {} {a_strength:?} with {} {b_strength:?}", a.cards, b.cards);

    let mut ordering = a_strength.cmp(&b_strength);
    if ordering == Ordering::Equal {
      ordering = a.cmp_high_card(b);
    }
    ordering
    /*
    if a_strength > b_strength {
      //println!("greater");
      Ordering::Greater
    }
    else if a_strength < b_strength {
      //println!("less");
      Ordering::Less
    }
    else {
      let mut a_it = a.cards.chars();
      let mut b_it = b.cards.chars();
      let mut a = a_it.next();
      let mut b = b_it.next();
      while a.is_some() {
        let mut ac = a.unwrap();
        let mut bc = b.unwrap();
        if ac != bc {
          ac = card_to_char(ac);
          bc = card_to_char(bc);
          if ac > bc { return Ordering::Greater; }
          if bc > ac { return Ordering::Less; }
          panic!("cards were equal");
        }

        a = a_it.next();
        b = b_it.next();

      }

      println!("cards were equal");
      Ordering::Equal
    }*/
  }
}

impl Hand {
  fn cmp_high_card(&self, other: &Hand) -> Ordering {
    let a = self;
    let b = other;
    let mut a_it = a.cards.chars();
    let mut b_it = b.cards.chars();
    let mut a = a_it.next();
    let mut b = b_it.next();
    while a.is_some() {
      let mut ac = a.unwrap();
      let mut bc = b.unwrap();
      if ac != bc {
        ac = card_to_char(ac);
        bc = card_to_char(bc);
        if ac > bc {
          return Ordering::Greater;
        }
        if bc > ac {
          return Ordering::Less;
        }
        panic!("cards were equal");
      }

      a = a_it.next();
      b = b_it.next();
    }

    println!("cards were equal");
    Ordering::Equal
  }

  fn cmp_high_card_j(&self, other: &Hand) -> Ordering {
    let a = self;
    let b = other;
    let mut a_it = a.cards.chars();
    let mut b_it = b.cards.chars();
    let mut a = a_it.next();
    let mut b = b_it.next();
    while a.is_some() {
      let mut ac = a.unwrap();
      let mut bc = b.unwrap();
      if ac != bc {
        ac = card_to_char_j(ac);
        bc = card_to_char_j(bc);
        if ac > bc {
          return Ordering::Greater;
        }
        if bc > ac {
          return Ordering::Less;
        }
        panic!("cards were equal");
      }

      a = a_it.next();
      b = b_it.next();
    }

    println!("cards were equal");
    Ordering::Equal
  }
}

fn cmp_j(a: &Hand, b: &Hand) -> Ordering {
  let a_strength = hand_strength_j(&a.cards);
  let b_strength = hand_strength_j(&b.cards);

  //println!("comparing {} {a_strength:?} with {} {b_strength:?}", a.cards, b.cards);

  let mut ordering = a_strength.cmp(&b_strength);
  if ordering == Ordering::Equal {
    ordering = a.cmp_high_card_j(b);
  }
  ordering
  /*
  if a_strength > b_strength {
    //println!("greater");
    Ordering::Greater
  }
  else if a_strength < b_strength {
    //println!("less");
    Ordering::Less
  }
  else {
    /*
    let a_j = a.cards.find('J').is_some();
    let b_j = b.cards.find('J').is_some();
    if !a_j && b_j {
      return Ordering::Greater;
    }
    else if a_j && !b_j {
      return Ordering::Less;
    }*/
    let mut a_it = a.cards.chars();
    let mut b_it = b.cards.chars();
    let mut a = a_it.next();
    let mut b = b_it.next();
    while a.is_some() {
      let mut ac = a.unwrap();
      let mut bc = b.unwrap();
      if ac != bc {
        ac = card_to_char_j(ac);
        bc = card_to_char_j(bc);
        if ac > bc { return Ordering::Greater; }
        if bc > ac { return Ordering::Less; }
        panic!("cards were equal");
      }

      a = a_it.next();
      b = b_it.next();

    }

    println!("cards were equal");
    Ordering::Equal
  }*/
}

fn card_to_char(card: char) -> char {
  if card == 'T' {
    'a'
  } else if card == 'J' {
    'b'
  } else if card == 'Q' {
    'c'
  } else if card == 'K' {
    'd'
  } else if card == 'A' {
    'e'
  } else {
    card
  }
}

fn card_to_char_j(card: char) -> char {
  if card == 'T' {
    'a'
  } else if card == 'J' {
    '0'
  } else if card == 'Q' {
    'c'
  } else if card == 'K' {
    'd'
  } else if card == 'A' {
    'e'
  } else {
    card
  }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
enum HandStrength {
  HighCard,
  OnePair,
  TwoPair,
  ThreeOAK,
  FullHouse,
  FourOAK,
  FiveOAK,
}

fn hand_strength(hand: &str) -> HandStrength {
  let mut counts = HashMap::<char, usize>::default();
  for card in hand.chars() {
    counts.entry(card).and_modify(|e| *e += 1).or_insert(1);
  }

  for value in counts.values() {
    if *value == 5 {
      return HandStrength::FiveOAK;
    }
    if *value == 4 {
      return HandStrength::FourOAK;
    }
  }

  if counts.len() == 2 {
    return HandStrength::FullHouse;
  }

  for value in counts.values() {
    if *value == 3 {
      return HandStrength::ThreeOAK;
    }
  }

  if counts.len() == 3 {
    return HandStrength::TwoPair;
  }

  for value in counts.values() {
    if *value == 2 {
      return HandStrength::OnePair;
    }
  }

  HandStrength::HighCard
}

fn hand_strength_j(hand: &str) -> HandStrength {
  let mut counts = HashMap::<char, usize>::default();
  let mut joker_count = 0;
  for card in hand.chars() {
    if card == 'J' {
      joker_count += 1;
    } else {
      //counts.entry(card).and_modify(|e| { *e += 1 }).or_insert(1);

      match counts.entry(card) {
        std::collections::hash_map::Entry::Occupied(mut e) => {
          e.insert(e.get() + 1);
        }
        std::collections::hash_map::Entry::Vacant(e) => {
          e.insert(1);
        }
      }

      /*
      match counts.entry(card) {
        std::collections::hash_map::Entry::Occupied(mut e) => {
          e.insert(counts.get(&card).unwrap()+1);
        }
        std::collections::hash_map::Entry::Vacant(e) => {
          e.insert(1);
        }
      }*/
    }
    /*

      if counts.contains_key(&card) {
      counts.insert(card, counts.get(&card).unwrap()+1);
    }
    else {
      counts.insert(card, 1);
    }*/
  }
  if joker_count == 5 {
    return HandStrength::FiveOAK;
  }

  for value in counts.values() {
    //println!("jokers {joker_count}, value {value}");
    if *value + joker_count == 5 {
      return HandStrength::FiveOAK;
    }
    if *value + joker_count == 4 {
      return HandStrength::FourOAK;
    }
  }

  if counts.len() == 2 {
    return HandStrength::FullHouse;
  }

  for value in counts.values() {
    if *value + joker_count == 3 {
      return HandStrength::ThreeOAK;
    }
  }

  if counts.len() == 3 {
    return HandStrength::TwoPair;
  }

  for value in counts.values() {
    if *value + joker_count == 2 {
      return HandStrength::OnePair;
    }
  }

  HandStrength::HighCard
}

fn analyse_data(hands: &mut [Hand]) -> usize {
  hands.sort();

  let mut score = 0;
  for (i, hand) in hands.iter().enumerate() {
    let rank = i + 1;
    score += rank * hand.bid;
    //println!("rank: {rank}, bid: {}, score: {score}", hand.bid);
  }
  score
}

fn analyse_data_j(hands: &mut [Hand]) -> usize {
  hands.sort_by(cmp_j);

  let mut score = 0;
  for (i, hand) in hands.iter().enumerate() {
    let rank = i + 1;
    score += rank * hand.bid;
    //println!("rank: {rank}, bid: {}, score: {score}", hand.bid);
  }
  score
}

fn load_data(filename: &str) -> Vec<Hand> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut hands = Vec::<Hand>::new();
  for line in contents.lines() {
    let r = sscanf::sscanf_unescaped!(line, "{String} {usize}").unwrap();
    hands.push(Hand { cards: r.0, bid: r.1 });
  }
  hands
}

fn main() {
  env_logger::init();

  let args = Args::parse();
  if args.benchmark {
    return;
  }

  let mut data = load_data("input7.txt");
  let score1 = analyse_data(&mut data);

  println!("score1: {score1}");

  let score2 = analyse_data_j(&mut data);
  println!("score2: {score2}");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load1() {
    let mut data = load_data("testinput.txt");
    let score = analyse_data(&mut data);

    assert_eq!(data[0].bid, 765);
    assert_eq!(data[1].bid, 220);
    assert_eq!(data[2].bid, 28);
    assert_eq!(data[3].bid, 684);
    assert_eq!(data[4].bid, 483);
    println!("{data:?}");
    assert_eq!(score, 6440);
  }

  #[test]
  fn test_load2() {
    let mut data = load_data("testinput.txt");
    let score = analyse_data_j(&mut data);

    assert_eq!(data[0].bid, 765);
    assert_eq!(data[1].bid, 28);
    assert_eq!(data[2].bid, 684);
    assert_eq!(data[3].bid, 483);
    assert_eq!(data[4].bid, 220);
    println!("{data:?}");
    assert_eq!(score, 5905);
  }

  #[test]
  fn test_hand_eval() {
    let s1 = hand_strength("KKKKK");
    assert_eq!(s1, HandStrength::FiveOAK);
    let s2 = hand_strength("KKKKJ");
    assert_eq!(s2, HandStrength::FourOAK);
    let s3 = hand_strength("KKK11");
    assert_eq!(s3, HandStrength::FullHouse);
    let s4 = hand_strength("22234");
    assert_eq!(s4, HandStrength::ThreeOAK);
    let s5 = hand_strength("KK5TT");
    assert_eq!(s5, HandStrength::TwoPair);
    let s6 = hand_strength("J123J");
    assert_eq!(s6, HandStrength::OnePair);
    let s7 = hand_strength("KA987");
    assert_eq!(s7, HandStrength::HighCard);
  }

  #[test]
  fn test_hand_eval_j() {
    let s0 = hand_strength_j("JJJJJ");
    assert_eq!(s0, HandStrength::FiveOAK);
    let s1 = hand_strength_j("KKJKK");
    assert_eq!(s1, HandStrength::FiveOAK);
    let s2 = hand_strength_j("KJJJ1");
    assert_eq!(s2, HandStrength::FourOAK);
    let s3 = hand_strength_j("KKJ11");
    assert_eq!(s3, HandStrength::FullHouse);
    let s4 = hand_strength_j("2J234");
    assert_eq!(s4, HandStrength::ThreeOAK);
    let s5 = hand_strength_j("KK5TT");
    assert_eq!(s5, HandStrength::TwoPair);
    let s6 = hand_strength_j("J1234");
    assert_eq!(s6, HandStrength::OnePair);
    let s7 = hand_strength_j("KAQ87");
    assert_eq!(s7, HandStrength::HighCard);

    let s8 = hand_strength_j("T55J5");
    assert_eq!(s8, HandStrength::FourOAK);
    let s9 = hand_strength_j("KTJJT");
    assert_eq!(s9, HandStrength::FourOAK);
    let s10 = hand_strength_j("QQQJA");
    assert_eq!(s10, HandStrength::FourOAK);
  }
}
