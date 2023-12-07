use std::fs::File;
use std::io::prelude::*;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility() {
      let map = load_map("testinput.txt");
      assert!(visible_trees(&map) == 21);
    }
    #[test]
    fn test_best_scenic_score() {
      let map = load_map("test_input.txt");
      assert!(best_scenic_score(&map) == 8 );
    }
}

pub struct TreeMap<T:Copy+Default> {
  v: Vec< Vec<T>>, //each inner vector is a horizontal row
  dim: usize,
}

impl<T:Copy+Default> TreeMap<T> {
  pub fn new(size: usize) -> TreeMap<T> {
    let mut tm:TreeMap<T> = TreeMap{ v: Vec::new(), dim: size };
    for _ in 0..size {
      tm.v.push(vec![T::default();size]);
    }
    tm
  }
  pub fn get(&self, x: usize, y:usize) -> T {
    let v = self.v.get(y).unwrap();
    *v.get(x).unwrap()
  }
  pub fn set(&mut self,x: usize, y:usize, val:T) {
    let v: &mut Vec<T> = self.v.get_mut(y).unwrap();
    v[x] = val;
  }
}

fn load_map(f: &str) -> TreeMap<usize> {
  /* input is a square plot of trees
   * each line has a new line char at the end
   * so the length of the file is n(n+1)
   * we can solve this. in quadratic form - n2 + n - l = 0;
   * a = 1, b = 1, c = -l
   * sqrt -b +- sqrt (b2-4ac) / (2a)
   * (-1 + sqrt(1+4l) ) /2
   */
  
  let mut file = File::open(f).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let flen =  file.metadata().unwrap().len();

  let dim = (f64::sqrt((1+4*flen) as f64) as usize -1)/2;
  println!("file size: {flen}, dim: {dim}");
  let mut tree_map = TreeMap::<usize>::new(dim);
  for (y,l) in contents.lines().enumerate() {
    for (x,c) in l.chars().enumerate() {
      let height = String::from(c).parse::<usize>().unwrap();
      assert!( height <= 9 );
      tree_map.set(x, y, height);
    }
  }
 
  tree_map
}

enum Direction {
  Forwards,
  Backwards,
}

fn scan_horiz( tree_map: &TreeMap<usize>, vis_map: &mut TreeMap<bool>, dir:Direction) {
  let x_start = 0;
  let x_end = tree_map.dim;
 
  let range:Vec<usize> = match dir {
      Direction::Forwards => (x_start..x_end).collect(),
      Direction::Backwards => (x_start..x_end).rev().collect(),
  };

  for y in 0..tree_map.dim {
    let mut highest_seen:isize = -1;
    for x in range.iter() {
      println!("{x},{y}");
      let t = tree_map.get(*x,y) as isize;
      if t > highest_seen {
        println!("{},{} is visible", x,y);
        vis_map.set(*x,y,true);
        highest_seen = t;
      }
    }
  }
}

fn scan_vert( tree_map: &TreeMap<usize>, vis_map: &mut TreeMap<bool>, dir:Direction) {
  let y_start = 0;
  let y_end = tree_map.dim;
 
  let range:Vec<usize> = match dir {
      Direction::Forwards => (y_start..y_end).collect(),
      Direction::Backwards => (y_start..y_end).rev().collect(),
  };

  for x in 0..tree_map.dim {
    let mut highest_seen:isize = -1;
    for y in range.iter() {
      println!("{x},{y}");
      let t = tree_map.get(x,*y) as isize;
      if t > highest_seen {
        println!("{},{} is visible", x,y);
        vis_map.set(x,*y,true);
        highest_seen = t;
      }
    }
  }
}

fn visible_trees( tree_map: &TreeMap<usize> ) -> usize {
  
  /* a structure to mark whether each map position is visible*/
  let mut vmap = TreeMap::<bool>::new(tree_map.dim);

  scan_horiz(tree_map, &mut vmap, Direction::Forwards);
  scan_horiz(tree_map, &mut vmap, Direction::Backwards);
  scan_vert(tree_map, &mut vmap, Direction::Forwards);
  scan_vert(tree_map, &mut vmap, Direction::Backwards);

  let mut vcount = 0;
  for y in 0..vmap.dim {
    for x in 0..vmap.dim {
      if vmap.get(y,x) {
        vcount += 1;
      }
    }
  }
  println!("{} visible trees", vcount);

  vcount
}

fn scenic_score( tree_map: &TreeMap<usize>, x: usize, y: usize ) -> usize {
  let h = tree_map.get(x,y);
  let mut right_score = 0;
  let mut xr = x+1;
  loop {
    if xr >= tree_map.dim {
      break;
    }
    right_score += 1;
    if tree_map.get(xr,y) >= h {
      break;
    }
    xr += 1;
  }

  let mut left_score = 0;
  let mut xl:isize = x as isize-1;
  loop {
    if xl < 0 {
      break;
    }
    left_score += 1;
    if tree_map.get(xl as usize,y) >= h {
      break;
    }
    xl -= 1;
  }

  let mut up_score = 0;
  let mut yu:isize = y as isize -1;
  loop {
    if yu < 0 {
      break;
    }
    up_score += 1;
    if tree_map.get(x,yu as usize) >= h {
      break;
    }
    yu -= 1;
  }

  let mut down_score = 0;
  let mut yd = y+1;
  loop {
    if yd >= tree_map.dim {
      break;
    }
    down_score += 1;
    if tree_map.get(x,yd) >= h {
      break;
    }
    yd += 1;
  }

  left_score*right_score*up_score*down_score
}

fn best_scenic_score( tree_map: &TreeMap<usize> ) -> usize {
  let mut score = 0;

  for y in 0..tree_map.dim {
    for x in 0..tree_map.dim {
      let s = scenic_score(tree_map, x, y);
      if s > score {
        score = s;
      }
    }
  }
  println!("best scenic score: {score}");

  score
}

fn main() -> std::io::Result<()> {

  let map = load_map("input8.txt");

  println!("visibility: {}", visible_trees(&map));
  println!("best scenic score: {}", best_scenic_score(&map));

  Ok(())
}
