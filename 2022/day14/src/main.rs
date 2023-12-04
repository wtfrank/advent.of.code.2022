use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

use advent::{TerrainMap,Dims,Point};

#[cfg(test)]
mod tests {
    use super::*;

   #[test]
    fn test_compare() {
      let mut m = load_scan("testinput.txt");
      let units = simulate_fall(&mut m);
      assert_eq!(units, 24);
    }
   #[test]
    fn test_compare2() {
      let mut m = load_scan("testinput.txt");
      add_floor(&mut m);
      let units = simulate_fall(&mut m);
      assert_eq!(units, 93);
    }
}

#[derive(Default,Copy,Clone,PartialEq,Eq)]
enum ScanData {
  #[default]
  Air,
  Rock,
  Sand
}

fn load_scan(filename: &str) -> TerrainMap<ScanData> {
  let mut file = File::open(filename).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();

  let lines = contents.lines();
  let mut paths: Vec< Vec< Point >> = Vec::new();
  let mut max_x = 0;
  let mut max_y = 0;
  for line in lines {
    let mut path:Vec<Point> = Vec::new();
    for tok in line.split_whitespace() {
      if tok == "->" {
        continue;
      }

      let mut i = tok.split(',');
      let x = i.next().unwrap().parse::<usize>().unwrap();
      let y = i.next().unwrap().parse::<usize>().unwrap();
      if x > max_x {
        max_x = x;
      }
      if y > max_y {
        max_y = y;
      }
      path.push(Point{x:x as isize,y:y as isize});
      assert_eq!(i.next(), None);
    }
    paths.push(path);
  }

  let mut tm = TerrainMap::<ScanData>::new(Dims{width:max_x+1+max_y, height:max_y+3,..Default::default()});
  for path in paths {
    for i in 1..path.len() {
      let mut p = path[i-1];
      let q = path[i];
      assert!(p.x == q.x || p.y == q.y);
      loop {
        tm.set(&p, ScanData::Rock);
        if p == q {
          break;
        }

        match p.x.cmp(&q.x) {
          Ordering::Greater => p.x -= 1,
          Ordering::Less => p.x += 1,
          _ => (),
        }

        /*
        if p.x > q.x {
          p.x -= 1;
        }
        else if p.x < q.x {
          p.x += 1;
        }*/

        match p.y.cmp(&q.y) {
          Ordering::Greater => p.y -= 1,
          Ordering::Less => p.y += 1,
          _ => (),
        }
        /*
        if p.y > q.y {
          p.y -= 1;
        }
        else if p.y < q.y {
          p.y += 1;
        }
        */
      }
    }
  }

  tm
}

fn add_floor(map: &mut TerrainMap<ScanData>) {
  for x in 0..map.dims.width {
    map.set(&Point{x:x as isize, y:map.dims.height as isize -1}, ScanData::Rock);
  }
}

fn simulate_fall(map: &mut TerrainMap<ScanData>) -> usize {
  let mut count = 0;
  loop {
    let mut s = Point{x:500,y:0};

    loop {
      if map.get(&Point{x:s.x, y:s.y+1}) == ScanData::Air {
        s.y += 1;
      }
      else if map.get(&Point{x:s.x-1, y:s.y+1}) == ScanData::Air {
        s.x -= 1;
        s.y += 1;
      }
      else if map.get(&Point{x:s.x+1, y:s.y+1}) == ScanData::Air {
        s.x += 1;
        s.y += 1;
      }
      else {
        map.set(&s, ScanData::Sand);
        count += 1;
        if s.x == 500 && s.y == 0 {
          return count;
        }
        break;
      }
     
      if s.y >= map.dims.height as isize - 1 {
        return count;
      }
    }

  }
}


fn main() -> std::io::Result<()> {
  let mut m = load_scan("input14.txt");
  let units = simulate_fall(&mut m);

  println!("{units}");
  let mut m = load_scan("input14.txt");
  add_floor(&mut m);
  let units = simulate_fall(&mut m);
  println!("{units}");

  Ok(())
}
