use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::cmp::Ordering;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility() {
      let total_positions = process_path("testinput.txt", 2);
      assert!(total_positions == 13);
      let total_positions = process_path("testinput.txt", 10);
      assert!(total_positions == 1);
      let total_positions = process_path("testinput2.txt", 10);
      assert!(total_positions == 36);
    }
}

#[derive(Debug,Copy,Clone,Hash,PartialEq,Eq)]
struct Point {
  x:isize,
  y:isize,
}
impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}


fn process_path(f: &str, rope_len: usize) -> usize {
  let mut file = File::open(f).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let mut visited = HashSet::<Point>::new();
  /* origin is top left */
  let mut knots:Vec<Point> = vec!(Point{x:0,y:0}; rope_len);

  visited.insert( knots[rope_len-1] );

  for l in contents.lines() {
    let mut t = l.split_whitespace();
    let dir = t.next().unwrap();
    let count = t.next().unwrap().parse::<usize>().unwrap();
    for _ in 0..count {
      match dir {
        "U" => knots[0].y -= 1,
        "D" => knots[0].y += 1,
        "L" => knots[0].x -= 1,
        "R" => knots[0].x += 1,
        _ => panic!("Unexpected input"),
      }

      for k in 1..rope_len {
        if (knots[k-1].y - knots[k].y).abs() > 1 ||
           (knots[k-1].x - knots[k].x).abs() > 1 {
          println!("pos {k} has to move (head is at {})", knots[0]);

          match knots[k-1].y.cmp(&knots[k].y) {
            Ordering::Greater => knots[k].y += 1,
            Ordering::Less => knots[k].y -= 1,
            _ => (),
          }
          /*
          if knots[k-1].y - knots[k].y > 0 {
            knots[k].y += 1;
          }
          else if knots[k-1].y - knots[k].y < 0 {
            knots[k].y -= 1;
          }*/

          match knots[k-1].x.cmp(&knots[k].x) {
            Ordering::Greater => knots[k].x += 1,
            Ordering::Less => knots[k].x -= 1,
            _ => (),
          }
          /*
          if knots[k-1].x - knots[k].x > 0 {
            knots[k].x += 1;
          }
          else if knots[k-1].x - knots[k].x < 0 {
            knots[k].x -= 1;
          }
          */
        }
    /*
      if (h_y-t_y).abs() > 1 || (h_x-t_x).abs() > 1 {
        if h_y-t_y > 0 {
          t_y += 1;
        }
        else if h_y-t_y < 0 {
          t_y -= 1;
        }

        if h_x-t_x > 0 {
          t_x += 1;
        }
        else if h_x-t_x < 0 {
          t_x -= 1;
        }
      }
    */
      }
      visited.insert( knots[rope_len-1] );
    }
  }
  println!("Visited {}", visited.len());

  visited.len()
}



fn main() -> std::io::Result<()> {
  /* who knows what are the input traces out
   * so we can't pre-configure an arena.
   * Just track position of head and tail,
   * starting from 0.
   * Use a Hashset of tuples to track the
   * positions the tail has reached.
   */

  let total_positions = process_path("input9.txt", 10);


  println!("covered {total_positions} positions");

  Ok(())
}
