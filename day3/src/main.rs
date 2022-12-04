use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prio() {
        assert_eq!(item_prio(&'a'), 1);
        assert_eq!(item_prio(&'z'), 26);
        assert_eq!(item_prio(&'A'), 27);
        assert_eq!(item_prio(&'Z'), 52);
    }
}

fn item_prio(c:&char) -> u32 {
  let mut v = *c as u32;
  if v >= 97 {
    v -= 96;
  }
  else if v >= 65 {
    v -= 38;
  }
  else {
    panic!("unexpected character {c}");
  }
  v
}

struct TripleLine<I> {
  iter: I,
}

impl<I>  TripleLine<I> {
  fn new( i: I ) -> TripleLine<I> {
    TripleLine { iter: i }
  }
}

impl<I> Iterator for TripleLine<I> where I: Iterator {
  type Item = (I::Item, I::Item, I::Item);

  fn next(&mut self) -> Option<Self::Item> {
    match self.iter.next() {
      None => None,
      Some(v1) => {
        let v2 = self.iter.next().expect("number of items divisible by 3");
        let v3 = self.iter.next().expect("number of items divisible by 3");
        Some( (v1, v2, v3) )
      },
    }
  }
}

fn main() -> std::io::Result<()> {
  let mut file = File::open("input.txt")?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;


  let mut score:u32 = 0;
  let mut score2:u32 = 0;
  for line in contents.lines() {
    assert!(line.len() %2 == 0);

    let comp1 = &line[0..line.len()/2];
    let comp2 = &line[line.len()/2..line.len()];
    let mut set1:HashSet<char> = HashSet::new();
    let mut set2:HashSet<char> = HashSet::new();
    for c in comp1.chars() {
      set1.insert(c);
    }
    for c in comp2.chars() {
      set2.insert(c);
    }

    let mut i = set1.intersection(&set2);
    let c = i.next().expect("one mispack expected");
    score += item_prio(c);

  }
  println!("score: {score}");


  let i = TripleLine::new(contents.lines());
  for (line1, line2, line3) in i {
    let mut set1:HashSet<char> = HashSet::new();
    let mut set2:HashSet<char> = HashSet::new();
    let mut set3:HashSet<char> = HashSet::new();

    for c in line1.chars() {
      set1.insert(c);
    }
    for c in line2.chars() {
      set2.insert(c);
    }
    for c in line3.chars() {
      set3.insert(c);
    }

    //println!("1: {line1}, 2: {line2}, 3: {line3}");
   
   /* wow it turns out intersecting more than 2 sets in rust is a shitshow */
    let set12:HashSet<char> = set1.intersection(&set2).cloned().collect();
    let mut i = set12.intersection(&set3);
    //let mut i = set1.intersection(&set2).cloned().collect::<HashSet<char>>().intersection(&set3);
    //let mut i = set1.intersection(&set2.intersection(&set3).cloned().collect());
    let c = i.next().expect("one common item expected");

    score2 += item_prio(c);
  }

  println!("score2: {score2}");

  Ok(())
}
