use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;

//use advent::{TerrainMap,Dims,Point};

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_compare() {
    let r = compare_packets("testinput.txt");
    assert_eq!(r, 13);
  }
  #[test]
  fn test_decoder_key() {
    let r = decoder_key("testinput.txt");
    assert_eq!(r, 140);
  }

  #[test]
  fn test_parse1() {
    let p = parse_packet("[]");
    assert!(matches!(p, Packet::List(_)));
    if let Packet::List(v) = p {
      assert_eq!(v.len(), 0);
      println!("asserted inside if let");
    }
  }
  #[test]
  fn test_parse2() {
    let p = parse_packet("[10]");
    assert!(matches!(p, Packet::List(_)));
    if let Packet::List(v) = p {
      assert_eq!(v.len(), 1);
      assert!(matches!(v[0], Packet::Integer(10)));
    }
  }
  #[test]
  fn test_parse3() {
    let p = parse_packet("[[]]");
    assert!(matches!(p, Packet::List(_)));
    if let Packet::List(v) = p {
      assert_eq!(v.len(), 1);
      assert!(matches!(v[0], Packet::List(_)));
    }
  }

  #[test]
  fn test_cmp1() {
    let p = parse_packet("[1,1]");
    let q = parse_packet("[1,2]");
    let order = packets_cmp(&p, &q);
    assert_eq!(order, Ordering::Less);
  }
  #[test]
  fn test_cmp2() {
    let p = parse_packet("[1]");
    let q = parse_packet("[[2]]");
    let order = packets_cmp(&p, &q);
    assert_eq!(order, Ordering::Less);
  }
  #[test]
  fn test_cmp3() {
    let p = parse_packet("[1,1]");
    let q = parse_packet("[[1,2,2]]");
    let order = packets_cmp(&p, &q);
    assert_eq!(order, Ordering::Less);
  }
  #[test]
  fn test_cmp4() {
    let p = parse_packet("[]");
    let q = parse_packet("[[1,2,2]]");
    let order = packets_cmp(&p, &q);
    assert_eq!(order, Ordering::Less);
  }
  #[test]
  fn test_cmp5() {
    let p = parse_packet("[[],[]]");
    let q = parse_packet("[[],[1,2,2]]");
    let order = packets_cmp(&p, &q);
    assert_eq!(order, Ordering::Less);
  }
}

/* This function expects an iterator at a position just after
 * an opening '['.
 * This function continues parsing until (and including) the
 * matching ']'.
 */
fn do_parse_packet(chars: &mut std::str::Chars) -> Packet {
  let mut v = Vec::<Packet>::new();

  let mut cur: String = String::new();
  loop {
    let c = chars.next();
    match c {
      Some('[') => v.push(do_parse_packet(chars)),
      Some(']') => {
        if !cur.is_empty() {
          v.push(Packet::Integer(cur.parse::<usize>().unwrap()));
        }
        return Packet::List(v);
      }
      Some(',') => {
        if !cur.is_empty() {
          v.push(Packet::Integer(cur.parse::<usize>().unwrap()));
          cur.clear();
        }
      }
      Some(d @ '0'..='9') => cur.push(d),
      _ => panic!("Invalid data"),
    }
  }
}

fn parse_packet(p: &str) -> Packet {
  let mut iter = p.chars();

  match iter.next() {
    Some('[') => (),
    _ => panic!("Malformed input"),
  };

  do_parse_packet(&mut iter)
}

/* Returns Less if packet1 comes before packet2 by the weird task rules.
 * */
fn packets_cmp(packet1: &Packet, packet2: &Packet) -> Ordering {
  if let Packet::Integer(x) = packet1 {
    if let Packet::Integer(y) = packet2 {
      return x.cmp(y);
    }
  }

  if let Packet::List(v) = packet1 {
    if let Packet::List(w) = packet2 {
      let mut i = 0;
      loop {
        let a = v.get(i);
        let b = w.get(i);
        if a.is_none() && b.is_none() {
          return Ordering::Equal;
        } else if a.is_none() && b.is_some() {
          return Ordering::Less;
        } else if a.is_some() && b.is_none() {
          return Ordering::Greater;
        }
        //both are Some
        if let Some(aa) = a {
          if let Some(bb) = b {
            let order = packets_cmp(aa, bb);
            if order != Ordering::Equal {
              return order;
            }
            //otherwise we'll try the next element in the list
          }
        }
        i += 1;
      }
    }
  }

  //one is a list and the other an integer
  if let Packet::List(_) = packet1 {
    if let Packet::Integer(x) = packet2 {
      return packets_cmp(packet1, &Packet::List(vec![Packet::Integer(*x)]));
    }
  }

  if let Packet::Integer(x) = packet1 {
    if let Packet::List(_) = packet2 {
      return packets_cmp(&Packet::List(vec![Packet::Integer(*x)]), packet2);
    }
  }

  panic!("Shouldn't be able to reach here");
}

fn compare_packets(filename: &str) -> usize {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut lines = contents.lines();
  let mut idx = 1;
  let mut total = 0;
  loop {
    let packet1_str = lines.next().unwrap();
    let packet2_str = lines.next().unwrap();
    let packet1 = parse_packet(packet1_str);
    let packet2 = parse_packet(packet2_str);

    if packets_cmp(&packet1, &packet2) != Ordering::Greater {
      total += idx;
    }

    idx += 1;
    let n = lines.next();
    if n.is_none() {
      break;
    }
  }

  total
}

#[derive(Eq, PartialEq)]
enum Packet {
  List(Vec<Packet>),
  Integer(usize),
}

impl Ord for Packet {
  fn cmp(&self, other: &Self) -> Ordering {
    packets_cmp(self, other)
  }
}

impl PartialOrd for Packet {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

fn decoder_key(filename: &str) -> usize {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut lines = contents.lines();
  let mut packets: Vec<Packet> = Vec::new();

  let divider1_str = "[[2]]";
  let divider2_str = "[[6]]";
  packets.push(parse_packet(divider1_str));
  packets.push(parse_packet(divider2_str));
  loop {
    match lines.next() {
      None => break,
      Some(packet_str) => {
        if packet_str.is_empty() {
          continue;
        }
        packets.push(parse_packet(packet_str));
      }
    }
  }

  packets.sort();

  let divider1 = parse_packet(divider1_str);
  let divider2 = parse_packet(divider2_str);

  let mut idx1 = 0;
  let mut idx2 = 0;

  for (i, packet) in packets.iter().enumerate() {
    if packets_cmp(&divider1, packet) == Ordering::Equal {
      idx1 = i + 1;
    } else if packets_cmp(&divider2, packet) == Ordering::Equal {
      idx2 = i + 1;
    }
  }

  idx1 * idx2
}

fn main() -> std::io::Result<()> {
  let r = compare_packets("input13.txt");

  println!("{r}");

  let r = decoder_key("input13.txt");
  println!("{r}");

  Ok(())
}
